//! The whole backend in one process: an iroh relay, the static client and the
//! channel hub.
//!
//! * **Relay.** [`RelayService`] is the HTTP service of `iroh-relay`. It owns
//!   the listener, upgrades `/relay` to the relay protocol and answers the
//!   `/ping` latency probe. Browsers reach every peer through it; the traffic
//!   it forwards is end-to-end encrypted between endpoints.
//! * **Static client.** The same service takes extra handlers for exact
//!   paths, which is how the page, its JavaScript shim and the WebAssembly
//!   module are served.
//! * **Hub.** An iroh endpoint in this process (homed on the relay over
//!   loopback) that tells the members of the channel about each other. `/hub`
//!   returns its endpoint id.
//!
//! The relay and the hub keep their state in memory, so run exactly one
//! instance.

mod hub;

use std::net::{Ipv6Addr, SocketAddr};
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use http::header::{ACCEPT_ENCODING, CACHE_CONTROL, CONNECTION, CONTENT_ENCODING, CONTENT_TYPE};
use http::{HeaderMap, Method, Request, Response, StatusCode, response::Builder};
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use iroh::SecretKey;
use iroh_relay::KeyCache;
use iroh_relay::server::http_server::{BytesBody, HyperResult};
use iroh_relay::server::{AllowAll, ClientRateLimit, Handlers, Metrics, RelayService};
use tokio::net::TcpListener;

type Error = Box<dyn std::error::Error + Send + Sync>;

/// A connection that has not become a relay WebSocket is dropped after this
/// long. Static responses close their connection, so it bounds one download.
const ESTABLISH_TIMEOUT: Duration = Duration::from_secs(30);

/// Per-client receive budget of the relay: room for several video streams.
const RELAY_RX_BYTES_PER_SECOND: u32 = 16 * 1024 * 1024;
const RELAY_RX_BURST_BYTES: u32 = 64 * 1024 * 1024;

/// The files of the client: URL path, file name in the static directory,
/// content type.
const FILES: [(&str, &str, &str); 3] = [
    ("/", "index.html", "text/html; charset=utf-8"),
    ("/tall.js", "tall.js", "text/javascript"),
    ("/tall_bg.wasm", "tall_bg.wasm", "application/wasm"),
];

fn respond(response: Builder, status: StatusCode, body: impl Into<Bytes>) -> HyperResult<Response<BytesBody>> {
    let body: BytesBody = Box::new(Full::new(body.into()));
    Ok(response.status(status).header(CONNECTION, "close").body(body)?)
}

/// Serves one file, preferring a `.gz` sibling when the client accepts gzip.
/// Files are read per request: they are few, small and change on rebuild.
fn serve_file(
    dir: PathBuf,
    file: &'static str,
    content_type: &'static str,
) -> impl Fn(Request<Incoming>, Builder) -> HyperResult<Response<BytesBody>> {
    move |request, response| {
        let gzip = request.headers().get(ACCEPT_ENCODING).and_then(|value| value.to_str().ok()).is_some_and(|value| value.contains("gzip"));
        let response = response.header(CONTENT_TYPE, content_type).header(CACHE_CONTROL, "no-cache");
        if gzip && let Ok(bytes) = std::fs::read(dir.join(format!("{file}.gz"))) {
            return respond(response.header(CONTENT_ENCODING, "gzip"), StatusCode::OK, bytes);
        }
        match std::fs::read(dir.join(file)) {
            Ok(bytes) => respond(response, StatusCode::OK, bytes),
            Err(_) => respond(response, StatusCode::NOT_FOUND, "not built"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let port: u16 = std::env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080);
    let dir = PathBuf::from(std::env::var("STATIC_DIR").unwrap_or_else(|_| "web/dist".into()));

    // A fresh identity per start; clients read it from /hub when they load
    // and again whenever they lose the hub.
    let hub_key = SecretKey::generate();
    let hub_id = hub_key.public().to_string();

    let mut handlers = Handlers::default();
    for (path, file, content_type) in FILES {
        handlers.insert((Method::GET, path), Box::new(serve_file(dir.clone(), file, content_type)));
    }
    handlers.insert(
        (Method::GET, "/hub"),
        Box::new(move |_, response| respond(response.header(CACHE_CONTROL, "no-store"), StatusCode::OK, hub_id.clone())),
    );
    handlers.insert((Method::GET, "/ping"), Box::new(|_, response| respond(response, StatusCode::OK, "")));
    handlers.insert((Method::GET, "/healthz"), Box::new(|_, response| respond(response, StatusCode::OK, "ok")));

    let mut limit = ClientRateLimit::new(NonZeroU32::new(RELAY_RX_BYTES_PER_SECOND).expect("nonzero"));
    limit.max_burst_bytes = NonZeroU32::new(RELAY_RX_BURST_BYTES);
    let relay = RelayService::new(handlers, HeaderMap::new(), Some(limit), KeyCache::new(1024), Arc::new(AllowAll), Arc::new(Metrics::default()));

    let listener = TcpListener::bind(SocketAddr::from((Ipv6Addr::UNSPECIFIED, port))).await?;
    println!("tall: http://localhost:{port} (relay, hub and client from {})", dir.display());
    let accept = {
        let relay = relay.clone();
        async move {
            loop {
                match listener.accept().await {
                    Ok((stream, _)) => {
                        stream.set_nodelay(true).ok();
                        tokio::spawn(relay.clone().handle_connection(stream, None, ESTABLISH_TIMEOUT));
                    }
                    Err(error) => {
                        eprintln!("accept: {error}");
                        tokio::time::sleep(Duration::from_millis(100)).await;
                    }
                }
            }
        }
    };

    tokio::select! {
        () = accept => {}
        result = hub::run(hub_key, port) => result?,
        _ = shutdown() => println!("tall: shutting down"),
    }
    relay.shutdown().await;
    Ok(())
}

async fn shutdown() {
    use tokio::signal::unix::{SignalKind, signal};
    let mut term = signal(SignalKind::terminate()).expect("signal handler");
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
        _ = term.recv() => {}
    }
}

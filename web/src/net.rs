//! iroh in the browser: the endpoint, the hub client and the mesh of peer
//! links (`tall_core::proto`). Browsers reach each other through the relay;
//! every connection is end-to-end encrypted QUIC between endpoint keys.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use iroh::endpoint::{Connection, QuicTransportConfig, RecvStream, SendStream, presets};
use iroh::{Endpoint, EndpointAddr, EndpointId, RelayConfig, RelayMap, RelayMode, RelayUrl, SecretKey, TransportAddr, Watcher as _};
use n0_future::time;
use tall_core::proto::{self, GroupHeader, Id};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;

use crate::media::{self, Decoder};
use crate::{App, js};

/// How long to wait for the relay before giving up on a bind.
const ONLINE_TIMEOUT: Duration = Duration::from_secs(20);
const RETRY: Duration = Duration::from_millis(1500);
/// Encoded frames queued for one peer before it is considered behind.
const VIDEO_QUEUE: usize = 6;

fn transport() -> QuicTransportConfig {
    let idle = Duration::from_secs(proto::IDLE_TIMEOUT_SECS).try_into().expect("idle timeout");
    QuicTransportConfig::builder().max_idle_timeout(Some(idle)).build()
}

/// Binds the endpoint and waits until it is connected to the relay.
pub async fn bind(key: SecretKey, relay: &RelayUrl) -> Result<Endpoint, String> {
    let relays = RelayMap::from_iter([relay.clone()]);
    let endpoint = Endpoint::builder(presets::Minimal)
        .secret_key(key)
        .alpns(vec![proto::PEER_ALPN.to_vec()])
        .transport_config(transport())
        .relay_mode(RelayMode::Custom(relays.clone()))
        .bind()
        .await
        .map_err(|error| error.to_string())?;
    // iroh picks its home relay from a latency report and only retries a
    // failed one after 20 s; re-inserting the relay asks for a fresh report.
    if wait_online(&endpoint, &relays).await { Ok(endpoint) } else { Err("the relay is unreachable".into()) }
}

async fn wait_online(endpoint: &Endpoint, relays: &RelayMap) -> bool {
    let deadline = time::Instant::now() + ONLINE_TIMEOUT;
    let mut online = std::pin::pin!(endpoint.online());
    let mut step = Duration::from_secs(4);
    loop {
        let left = deadline.saturating_duration_since(time::Instant::now());
        if left.is_zero() {
            return false;
        }
        if time::timeout(left.min(step), &mut online).await.is_ok() {
            return true;
        }
        step = Duration::from_secs(2);
        if endpoint.home_relay_status().get().is_empty() {
            let relays: Vec<Arc<RelayConfig>> = relays.relays();
            if let Some(relay) = relays.into_iter().next() {
                endpoint.insert_relay(relay.url.clone(), relay).await;
            }
        }
    }
}

fn address(id: &Id, relay: &RelayUrl) -> Option<EndpointAddr> {
    let id = EndpointId::from_bytes(id).ok()?;
    Some(EndpointAddr::from_parts(id, [TransportAddr::Relay(relay.clone())]))
}

/// Stays in the channel: asks the server for the hub's endpoint id, joins, and
/// applies every roster the hub sends. Starts over whenever the hub is lost.
pub async fn hub_loop(app: Rc<App>, endpoint: Endpoint) {
    loop {
        if let Err(error) = hub_session(&app, &endpoint).await {
            js::log(&format!("hub: {error}"));
        }
        app.online.set(false);
        time::sleep(RETRY).await;
    }
}

async fn hub_session(app: &Rc<App>, endpoint: &Endpoint) -> Result<(), String> {
    let describe = |error: JsValue| js::describe(&error);
    let response = js::call_async(&js::global(), "fetch", &["/hub".into()]).await.map_err(describe)?;
    let hub = js::call_async(&response, "text", &[]).await.map_err(describe)?.as_string().unwrap_or_default();
    let hub: EndpointId = hub.trim().parse().map_err(|_| "bad hub id".to_string())?;
    let addr = EndpointAddr::from_parts(hub, [TransportAddr::Relay(app.relay.clone())]);
    let connection = endpoint.connect(addr, proto::HUB_ALPN).await.map_err(|e| e.to_string())?;
    let (mut send, mut recv) = connection.open_bi().await.map_err(|e| e.to_string())?;
    send.write_all(&[proto::HELLO]).await.map_err(|e| e.to_string())?;
    loop {
        let mut count = [0u8; 1];
        if recv.read_exact(&mut count).await.is_err() {
            let reason = connection.close_reason().map(|reason| reason.to_string()).unwrap_or_default();
            app.full.set(reason.contains("channel full"));
            return Err(reason);
        }
        let mut body = vec![0u8; proto::roster_body_len(count[0])];
        recv.read_exact(&mut body).await.map_err(|e| e.to_string())?;
        app.online.set(true);
        app.full.set(false);
        app.on_roster(endpoint, proto::decode_roster_body(&body));
    }
}

/// Admits incoming peer connections; `attach` turns away strangers.
pub async fn accept_loop(app: Rc<App>, endpoint: Endpoint) {
    while let Some(incoming) = endpoint.accept().await {
        let (app, endpoint) = (app.clone(), endpoint.clone());
        spawn_local(async move {
            if let Ok(connection) = incoming.await {
                app.attach(&endpoint, connection);
            }
        });
    }
}

/// One encoded video frame on its way to a peer.
struct VideoOut {
    /// Present on a key frame: the group header that starts a new stream.
    header: Option<Bytes>,
    frame_header: [u8; proto::FRAME_HEADER_LEN],
    data: Bytes,
}

/// The connection to one peer and everything received over it.
pub struct Link {
    pub id: Id,
    connection: Connection,
    /// The peer's `FLAG_MIC` / `FLAG_CAM`.
    pub flags: Cell<u8>,
    /// Loudness of the peer's recent audio, 0 to 1; decays every frame.
    pub level: Cell<f32>,
    /// The newest decoded `VideoFrame`.
    pub frame: RefCell<Option<JsValue>>,
    /// The newest video group seen; older streams are stopped.
    group: Cell<u32>,
    pub video_decoder: RefCell<Option<Decoder>>,
    pub audio_decoder: RefCell<Option<Decoder>>,
    /// When the next audio buffer should start, in `AudioContext` time.
    pub next_audio_time: Cell<f64>,
    pub audio_seq: Cell<Option<u16>>,
    video_tx: async_channel::Sender<VideoOut>,
    video_rx: async_channel::Receiver<VideoOut>,
    abort_tx: async_channel::Sender<()>,
    abort_rx: async_channel::Receiver<()>,
    /// Set when this peer fell behind: nothing is sent until the next key frame.
    skipping: Cell<bool>,
}

impl Link {
    pub fn set_frame(&self, frame: Option<JsValue>) {
        if let Some(old) = self.frame.replace(frame) {
            js::call(&old, "close", &[]);
        }
    }

    /// Hangs up, as the page goes away.
    pub fn close(&self) {
        self.connection.close(proto::CLOSE_LEAVE.into(), b"left");
    }

    pub fn send_datagram(&self, bytes: Bytes) {
        self.connection.send_datagram(bytes).ok();
    }
}

impl App {
    /// Applies a roster from the hub: retile, drop links to members who left
    /// and dial the newcomers this endpoint is responsible for.
    fn on_roster(self: &Rc<Self>, endpoint: &Endpoint, roster: Vec<Id>) {
        if *self.roster.borrow() == roster {
            return;
        }
        {
            // The first roster sets the order (join order, the same for
            // everyone); after that only arrivals and departures change it.
            let mut stack = self.stack.borrow_mut();
            if self.online_before.replace(true) {
                stack.sync(&roster);
            } else {
                *stack = tall_core::layout::Stack { items: roster.clone(), focus: 0 };
            }
        }
        let gone: Vec<Rc<Link>> = self.links.borrow().values().filter(|link| !roster.contains(&link.id)).cloned().collect();
        for link in gone {
            link.connection.close(proto::CLOSE_LEAVE.into(), b"left");
        }
        *self.roster.borrow_mut() = roster.clone();
        for id in roster {
            self.dial(endpoint, id);
        }
    }

    /// Dials `id` until it is linked or leaves the channel. Of two members, the
    /// one with the smaller id dials.
    fn dial(self: &Rc<Self>, endpoint: &Endpoint, id: Id) {
        if self.me >= id || self.links.borrow().contains_key(&id) || !self.dialing.borrow_mut().insert(id) {
            return;
        }
        let (app, endpoint) = (self.clone(), endpoint.clone());
        spawn_local(async move {
            while app.roster.borrow().contains(&id) && !app.links.borrow().contains_key(&id) {
                let Some(addr) = address(&id, &app.relay) else { break };
                match endpoint.connect(addr, proto::PEER_ALPN).await {
                    Ok(connection) => {
                        app.dialing.borrow_mut().remove(&id);
                        return app.attach(&endpoint, connection);
                    }
                    Err(_) => time::sleep(RETRY).await,
                }
            }
            app.dialing.borrow_mut().remove(&id);
        });
    }

    /// Starts a call over an established connection, if its remote is a
    /// member of the channel.
    fn attach(self: &Rc<Self>, endpoint: &Endpoint, connection: Connection) {
        let id: Id = *connection.remote_id().as_bytes();
        if !self.roster.borrow().contains(&id) {
            // The dialer may have heard of it before this side did; it retries.
            return connection.close(proto::CLOSE_NOT_ON_ROSTER.into(), b"not on the roster");
        }
        let (video_tx, video_rx) = async_channel::bounded(VIDEO_QUEUE);
        let (abort_tx, abort_rx) = async_channel::bounded(1);
        let link = Rc::new(Link {
            id,
            connection: connection.clone(),
            flags: Cell::new(0),
            level: Cell::new(0.0),
            frame: RefCell::new(None),
            group: Cell::new(0),
            video_decoder: RefCell::new(None),
            audio_decoder: RefCell::new(None),
            next_audio_time: Cell::new(0.0),
            audio_seq: Cell::new(None),
            video_tx,
            video_rx,
            abort_tx,
            abort_rx,
            skipping: Cell::new(true),
        });
        if let Some(old) = self.links.borrow_mut().insert(id, link.clone()) {
            old.connection.close(proto::CLOSE_REPLACED.into(), b"replaced");
        }
        self.media.want_key.set(true);
        self.send_state(&link);

        spawn_local(read_datagrams(self.clone(), link.clone()));
        spawn_local(accept_video(link.clone()));
        spawn_local(send_video(link.clone()));
        let (app, endpoint) = (self.clone(), endpoint.clone());
        spawn_local(async move {
            link.connection.closed().await;
            link.video_rx.close();
            link.set_frame(None);
            link.video_decoder.take();
            link.audio_decoder.take();
            let current = app.links.borrow().get(&id).is_some_and(|l| Rc::ptr_eq(l, &link));
            if current {
                app.links.borrow_mut().remove(&id);
                time::sleep(RETRY).await;
                app.dial(&endpoint, id);
            }
        });
    }

    /// Tells one peer whether the microphone and camera are on.
    pub fn send_state(&self, link: &Link) {
        link.send_datagram(Bytes::copy_from_slice(&[proto::DATAGRAM_STATE, self.media.flags()]));
    }

    /// Hands one encoded frame to every peer. A key frame starts a new group.
    /// A peer whose queue is full is behind: its queue and the stream being
    /// written are dropped and it waits for the next key frame, requested now.
    pub fn send_video(&self, key: bool, timestamp_us: u64, data: Bytes, codec: &str, width: u32, height: u32) {
        let header = key.then(|| {
            let group = self.media.group.get() + 1;
            self.media.group.set(group);
            Bytes::from(GroupHeader { group, width: width as u16, height: height as u16, codec: codec.into() }.encode())
        });
        let frame_header = proto::encode_frame_header(data.len() as u32, timestamp_us);
        for link in self.links.borrow().values() {
            if key {
                link.skipping.set(false);
            }
            if link.skipping.get() {
                continue;
            }
            let out = VideoOut { header: header.clone(), frame_header, data: data.clone() };
            if link.video_tx.try_send(out).is_err() {
                while link.video_rx.try_recv().is_ok() {}
                link.abort_tx.try_send(()).ok();
                link.skipping.set(true);
                self.media.want_key.set(true);
            }
        }
    }
}

async fn read_datagrams(app: Rc<App>, link: Rc<Link>) {
    while let Ok(datagram) = link.connection.read_datagram().await {
        match datagram.first() {
            Some(&proto::DATAGRAM_STATE) if datagram.len() >= 2 => link.flags.set(datagram[1]),
            Some(&proto::DATAGRAM_AUDIO) if datagram.len() > 4 => {
                let seq = u16::from_be_bytes([datagram[1], datagram[2]]);
                media::play_audio(&app, &link, seq, datagram[3], &datagram[4..]);
            }
            _ => {}
        }
    }
}

async fn accept_video(link: Rc<Link>) {
    while let Ok(stream) = link.connection.accept_uni().await {
        let link = link.clone();
        spawn_local(async move {
            read_group(link, stream).await;
        });
    }
}

/// Reads one group of pictures and feeds it to the decoder, for as long as it
/// is the newest group.
async fn read_group(link: Rc<Link>, mut stream: RecvStream) -> Option<()> {
    let mut fixed = [0u8; GroupHeader::FIXED_LEN];
    stream.read_exact(&mut fixed).await.ok()?;
    let (header, codec_len) = GroupHeader::decode_fixed(&fixed);
    let mut codec = vec![0u8; codec_len];
    stream.read_exact(&mut codec).await.ok()?;
    let codec = String::from_utf8(codec).ok()?;
    if header.group <= link.group.get() {
        stream.stop(0u32.into()).ok();
        return None;
    }
    link.group.set(header.group);
    let mut key = true;
    loop {
        let mut frame_header = [0u8; proto::FRAME_HEADER_LEN];
        stream.read_exact(&mut frame_header).await.ok()?;
        let (len, timestamp_us) = proto::decode_frame_header(&frame_header);
        if len > proto::MAX_FRAME {
            return None;
        }
        let mut data = vec![0u8; len];
        stream.read_exact(&mut data).await.ok()?;
        if link.group.get() != header.group || !media::decode_video(&link, &codec, key, timestamp_us, &data) {
            stream.stop(0u32.into()).ok();
            return None;
        }
        key = false;
    }
}

/// Writes queued frames to the peer, one stream per group.
async fn send_video(link: Rc<Link>) {
    let mut stream: Option<SendStream> = None;
    while let Ok(out) = link.video_rx.recv().await {
        // An abort that found nothing in flight is stale.
        while link.abort_rx.try_recv().is_ok() {}
        if out.header.is_some() {
            if let Some(mut old) = stream.take() {
                old.finish().ok();
            }
            match link.connection.open_uni().await {
                Ok(new) => stream = Some(new),
                Err(_) => break,
            }
        }
        let Some(current) = stream.as_mut() else { continue };
        let write = async {
            if let Some(header) = &out.header {
                current.write_all(header).await.ok()?;
            }
            current.write_all(&out.frame_header).await.ok()?;
            current.write_all(&out.data).await.ok()
        };
        let abort = async {
            link.abort_rx.recv().await.ok();
            None
        };
        if n0_future::future::or(write, abort).await.is_none()
            && let Some(mut broken) = stream.take()
        {
            broken.reset(0u32.into()).ok();
        }
    }
}

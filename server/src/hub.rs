//! The hub (`tall/hub/1`, see `tall_core::proto`): members of the channel
//! learn each other's endpoint ids here and then talk directly.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use iroh::endpoint::{Connection, QuicTransportConfig, presets};
use iroh::{Endpoint, RelayMap, RelayMode, RelayUrl, SecretKey};
use tall_core::proto::{self, Id};
use tokio::sync::watch;

use crate::Error;

/// How long a client has to say hello.
const HELLO_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Default)]
struct Channel {
    /// Members in join order, each with the serial of its connection.
    members: Vec<(Id, u64)>,
    serial: u64,
    roster: watch::Sender<Vec<Id>>,
}

impl Channel {
    fn publish(&self) {
        self.roster.send_replace(self.members.iter().map(|(id, _)| *id).collect());
    }
}

type Shared = Arc<Mutex<Channel>>;

fn transport() -> QuicTransportConfig {
    let idle = Duration::from_secs(proto::IDLE_TIMEOUT_SECS).try_into().expect("idle timeout");
    QuicTransportConfig::builder().max_idle_timeout(Some(idle)).build()
}

/// Runs the hub endpoint until it closes. It homes on this process's own
/// relay over loopback; clients address it through the relay's public URL.
pub async fn run(key: SecretKey, port: u16) -> Result<(), Error> {
    let relay: RelayUrl = format!("http://127.0.0.1:{port}").parse()?;
    let endpoint = Endpoint::builder(presets::Minimal)
        .secret_key(key)
        .alpns(vec![proto::HUB_ALPN.to_vec()])
        .transport_config(transport())
        .relay_mode(RelayMode::Custom(RelayMap::from_iter([relay])))
        .bind()
        .await?;
    println!("tall: hub {}", endpoint.id());
    let channel = Shared::default();
    while let Some(incoming) = endpoint.accept().await {
        let channel = channel.clone();
        tokio::spawn(async move {
            if let Ok(connection) = incoming.await {
                serve(connection, channel).await;
            }
        });
    }
    Ok(())
}

async fn serve(connection: Connection, channel: Shared) {
    let id: Id = *connection.remote_id().as_bytes();
    let hello = tokio::time::timeout(HELLO_TIMEOUT, async {
        let (send, mut recv) = connection.accept_bi().await.ok()?;
        let mut hello = [0u8; 1];
        recv.read_exact(&mut hello).await.ok()?;
        (hello[0] == proto::HELLO).then_some(send)
    });
    let Ok(Some(mut send)) = hello.await else {
        return connection.close(proto::CLOSE_LEAVE.into(), b"no hello");
    };

    let (serial, mut roster) = {
        let mut channel = channel.lock().expect("channel lock");
        channel.serial += 1;
        let serial = channel.serial;
        // A member that reconnects keeps its place.
        if let Some(member) = channel.members.iter_mut().find(|(member, _)| *member == id) {
            member.1 = serial;
        } else if channel.members.len() >= proto::MAX_MEMBERS {
            return connection.close(proto::CLOSE_FULL.into(), b"channel full");
        } else {
            channel.members.push((id, serial));
        }
        channel.publish();
        (serial, channel.roster.subscribe())
    };
    println!("tall: + {}", connection.remote_id().fmt_short());

    let heartbeat = Duration::from_secs(proto::ROSTER_INTERVAL_SECS);
    loop {
        let bytes = proto::encode_roster(&roster.borrow_and_update());
        if send.write_all(&bytes).await.is_err() {
            break;
        }
        tokio::select! {
            _ = connection.closed() => break,
            changed = roster.changed() => if changed.is_err() { break },
            () = tokio::time::sleep(heartbeat) => {}
        }
    }

    let mut channel = channel.lock().expect("channel lock");
    // Only the connection that holds the place gives it up.
    channel.members.retain(|member| *member != (id, serial));
    channel.publish();
    println!("tall: - {}", connection.remote_id().fmt_short());
}

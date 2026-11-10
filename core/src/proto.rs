//! Wire formats. All integers are big-endian.
//!
//! # `tall/hub/1`: who is in the channel
//!
//! There is one channel: whoever opens the page is in it. A client dials the
//! hub (the server's iroh endpoint), opens one bidirectional stream and
//! writes [`HELLO`]. The hub answers on the same stream with the roster,
//! again and again: every time it changes and every [`ROSTER_INTERVAL_SECS`]
//! as a heartbeat. A roster is one count byte and that many 32-byte endpoint
//! ids in join order ([`encode_roster`]). Leaving is closing the connection.
//! The hub closes with [`CLOSE_FULL`] when the channel has [`MAX_MEMBERS`].
//!
//! # `tall/peer/1`: the call
//!
//! Every two members hold one iroh connection, dialed by the one
//! with the smaller endpoint id. A connection from an endpoint that is not on
//! the roster is closed.
//!
//! * **Datagrams** start with a kind byte.
//!   [`DATAGRAM_STATE`]: one flags byte ([`FLAG_MIC`], [`FLAG_CAM`]), sent
//!   twice a second; it also keeps the connection alive.
//!   [`DATAGRAM_AUDIO`]: a `u16` sequence number, the channel count, then
//!   one Opus packet (48 kHz, 20 ms).
//! * **Video** is one unidirectional stream per group of pictures. The stream
//!   starts with a [`GroupHeader`]; then frames follow, each a `u32` length,
//!   a `u64` timestamp in microseconds and the encoded bytes. The first frame
//!   of a stream is a key frame. A receiver plays the newest group it has seen
//!   and stops older streams, so a slow link skips ahead instead of lagging.

pub const HUB_ALPN: &[u8] = b"tall/hub/1";
pub const PEER_ALPN: &[u8] = b"tall/peer/1";

/// Most members of the channel. Every member sends its video to every
/// other, so this bounds each member's upload to six streams.
pub const MAX_MEMBERS: usize = 7;
pub const HELLO: u8 = 1;
pub const ROSTER_INTERVAL_SECS: u64 = 3;
/// A connection that has been silent this long is gone: a closed laptop or a
/// dropped network frees its place and its tile this quickly. The roster
/// heartbeat and the state datagrams keep live connections well inside it.
pub const IDLE_TIMEOUT_SECS: u64 = 8;

pub const CLOSE_LEAVE: u32 = 1;
pub const CLOSE_FULL: u32 = 2;
pub const CLOSE_NOT_ON_ROSTER: u32 = 4;
pub const CLOSE_REPLACED: u32 = 5;

pub const DATAGRAM_STATE: u8 = 0;
pub const DATAGRAM_AUDIO: u8 = 1;
pub const FLAG_MIC: u8 = 1;
pub const FLAG_CAM: u8 = 2;

/// Largest encoded video frame a receiver accepts.
pub const MAX_FRAME: usize = 4 * 1024 * 1024;
pub const FRAME_HEADER_LEN: usize = 12;

pub type Id = [u8; 32];

pub fn encode_roster(ids: &[Id]) -> Vec<u8> {
    let mut out = Vec::with_capacity(1 + ids.len() * 32);
    out.push(ids.len() as u8);
    for id in ids {
        out.extend_from_slice(id);
    }
    out
}

/// The byte length of a roster of `count` ids, after the count byte.
pub fn roster_body_len(count: u8) -> usize {
    usize::from(count) * 32
}

pub fn decode_roster_body(body: &[u8]) -> Vec<Id> {
    body.chunks_exact(32).map(|c| c.try_into().expect("32 bytes")).collect()
}

/// Starts every video stream: which group this is, its codec (a WebCodecs
/// codec string such as `avc1.640028`) and the coded size.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupHeader {
    pub group: u32,
    pub width: u16,
    pub height: u16,
    pub codec: String,
}

impl GroupHeader {
    /// Bytes before the codec string: group, width, height, codec length.
    pub const FIXED_LEN: usize = 9;

    pub fn encode(&self) -> Vec<u8> {
        let codec = self.codec.as_bytes();
        let mut out = Vec::with_capacity(Self::FIXED_LEN + codec.len());
        out.extend_from_slice(&self.group.to_be_bytes());
        out.extend_from_slice(&self.width.to_be_bytes());
        out.extend_from_slice(&self.height.to_be_bytes());
        out.push(codec.len() as u8);
        out.extend_from_slice(codec);
        out
    }

    /// Parses the fixed part; returns the header with an empty codec and the
    /// codec string's length.
    pub fn decode_fixed(b: &[u8; Self::FIXED_LEN]) -> (Self, usize) {
        let header = Self {
            group: u32::from_be_bytes([b[0], b[1], b[2], b[3]]),
            width: u16::from_be_bytes([b[4], b[5]]),
            height: u16::from_be_bytes([b[6], b[7]]),
            codec: String::new(),
        };
        (header, usize::from(b[8]))
    }
}

pub fn encode_frame_header(len: u32, timestamp_us: u64) -> [u8; FRAME_HEADER_LEN] {
    let mut out = [0u8; FRAME_HEADER_LEN];
    out[..4].copy_from_slice(&len.to_be_bytes());
    out[4..].copy_from_slice(&timestamp_us.to_be_bytes());
    out
}

pub fn decode_frame_header(b: &[u8; FRAME_HEADER_LEN]) -> (usize, u64) {
    let len = u32::from_be_bytes([b[0], b[1], b[2], b[3]]) as usize;
    let ts = u64::from_be_bytes([b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11]]);
    (len, ts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roster_round_trip() {
        let ids = [[1u8; 32], [2u8; 32]];
        let bytes = encode_roster(&ids);
        assert_eq!(roster_body_len(bytes[0]), 64);
        assert_eq!(decode_roster_body(&bytes[1..]), ids);
    }

    #[test]
    fn group_header_round_trip() {
        let header = GroupHeader { group: 7, width: 1280, height: 720, codec: "avc1.640028".into() };
        let bytes = header.encode();
        let (mut parsed, len) = GroupHeader::decode_fixed(bytes[..9].try_into().unwrap());
        parsed.codec = String::from_utf8(bytes[9..9 + len].to_vec()).unwrap();
        assert_eq!(parsed, header);
    }

    #[test]
    fn frame_header_round_trip() {
        let bytes = encode_frame_header(1234, 99_000_000);
        assert_eq!(decode_frame_header(&bytes), (1234, 99_000_000));
    }
}

//! Capture, encode, decode and play, all through WebCodecs: camera frames
//! and microphone audio are read from `MediaStreamTrackProcessor`, encoded
//! once and sent to every peer; what arrives is decoded into `VideoFrame`s
//! for the renderer and into audio buffers scheduled on an `AudioContext`.

use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

use bytes::Bytes;
use js_sys::{Float32Array, Uint8Array};
use tall_core::proto;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::net::Link;
use crate::{App, js, obj};

/// Preferred first. H.264 High is hardware encoded almost everywhere.
const VIDEO_CODECS: [&str; 4] = ["avc1.640028", "avc1.42E01F", "vp09.00.31.08", "vp8"];
const FRAME_RATE: u32 = 30;
/// A key frame at least this often, so a group is two seconds.
const KEY_INTERVAL: u32 = 2 * FRAME_RATE;
/// Requested key frames (a peer joined or fell behind) are at least this far apart.
const KEY_SPACING_MS: f64 = 1000.0;
/// Video is 720p at 30 frames a second. Each peer gets its own copy, so the
/// upload is this times the number of peers: 4 Mbit/s each up to three
/// peers, down to 2.5 Mbit/s each (15 Mbit/s in all) in a full channel.
const UPLINK_BITS: u32 = 15_000_000;
const MAX_VIDEO_BITS: u32 = 4_000_000;
const MIN_VIDEO_BITS: u32 = 2_500_000;
const AUDIO_BITS_PER_CHANNEL: u32 = 64_000;
/// Opus frames are 20 ms.
const AUDIO_FRAME_US: f64 = 20_000.0;
/// Audio is scheduled this far ahead to absorb jitter, and dropped beyond the limit.
const AUDIO_CUSHION: f64 = 0.08;
const AUDIO_BACKLOG_LIMIT: f64 = 0.5;

pub struct Media {
    pub audio_context: JsValue,
    pub mic_on: Cell<bool>,
    pub cam_on: Cell<bool>,
    mic_track: RefCell<Option<JsValue>>,
    cam_track: RefCell<Option<JsValue>>,
    /// The newest camera `VideoFrame`, for the local tile.
    pub local_frame: RefCell<Option<JsValue>>,
    /// The next encoded frame should be a key frame.
    pub want_key: Cell<bool>,
    last_key_ms: Cell<f64>,
    /// The number of the newest video group sent.
    pub group: Cell<u32>,
    audio_seq: Cell<u16>,
}

impl Media {
    pub fn new() -> Self {
        let options = obj! { "latencyHint": "interactive", "sampleRate": 48000 };
        Self {
            audio_context: js::try_new("AudioContext", &[options]).unwrap_or(JsValue::NULL),
            mic_on: Cell::new(false),
            cam_on: Cell::new(false),
            mic_track: RefCell::new(None),
            cam_track: RefCell::new(None),
            local_frame: RefCell::new(None),
            want_key: Cell::new(false),
            last_key_ms: Cell::new(0.0),
            group: Cell::new(0),
            audio_seq: Cell::new(0),
        }
    }

    pub fn flags(&self) -> u8 {
        u8::from(self.mic_on.get()) * proto::FLAG_MIC | u8::from(self.cam_on.get()) * proto::FLAG_CAM
    }

    /// Browsers keep audio suspended until the page is clicked or typed in.
    pub fn sound_blocked(&self) -> bool {
        js::text(&self.audio_context, "state") == "suspended"
    }

    pub fn resume_sound(&self) {
        if self.sound_blocked() {
            js::call(&self.audio_context, "resume", &[]);
        }
    }

    fn set_local_frame(&self, frame: Option<JsValue>) {
        if let Some(old) = self.local_frame.replace(frame) {
            js::call(&old, "close", &[]);
        }
    }

    /// Whether to make the next frame a key frame on request.
    fn take_key(&self) -> bool {
        let now = js_sys::Date::now();
        let take = self.want_key.get() && now - self.last_key_ms.get() >= KEY_SPACING_MS;
        if take {
            self.want_key.set(false);
            self.last_key_ms.set(now);
        }
        take
    }
}

async fn user_media(constraints: JsValue) -> Option<JsValue> {
    let devices = js::get(&js::get(&js::global(), "navigator"), "mediaDevices");
    js::call_async(&devices, "getUserMedia", &[constraints]).await.ok()
}

fn first_track(stream: &JsValue, getter: &str) -> Option<JsValue> {
    let track = js::get(&js::call(stream, getter, &[]), "0");
    js::object_is(&track).then_some(track)
}

fn video_constraints() -> JsValue {
    obj! {
        "width": obj! { "ideal": 1280 },
        "height": obj! { "ideal": 720 },
        "frameRate": obj! { "ideal": FRAME_RATE },
        "facingMode": "user",
    }
}

fn audio_constraints() -> JsValue {
    obj! { "echoCancellation": true, "noiseSuppression": true, "autoGainControl": true }
}

/// Opens the microphone and the camera (one permission prompt). Either may
/// be missing or refused; the call goes on with what there is.
pub async fn start(app: Rc<App>) {
    let both = obj! { "audio": audio_constraints(), "video": video_constraints() };
    let (mut mic, mut cam) = (None, None);
    if let Some(stream) = user_media(both).await {
        mic = first_track(&stream, "getAudioTracks");
        cam = first_track(&stream, "getVideoTracks");
    } else {
        if let Some(stream) = user_media(obj! { "audio": audio_constraints() }).await {
            mic = first_track(&stream, "getAudioTracks");
        }
        if let Some(stream) = user_media(obj! { "video": video_constraints() }).await {
            cam = first_track(&stream, "getVideoTracks");
        }
    }
    if let Some(track) = mic {
        app.media.mic_on.set(true);
        *app.media.mic_track.borrow_mut() = Some(track.clone());
        spawn_local(microphone_loop(app.clone(), track));
    }
    if let Some(track) = cam {
        start_camera(&app, track);
    }
    app.broadcast_state();
}

fn start_camera(app: &Rc<App>, track: JsValue) {
    app.media.cam_on.set(true);
    *app.media.cam_track.borrow_mut() = Some(track.clone());
    spawn_local(camera_loop(app.clone(), track));
}

/// Muting keeps the microphone open but sends nothing.
pub fn toggle_mic(app: &Rc<App>) {
    let Some(track) = app.media.mic_track.borrow().clone() else { return };
    let on = !app.media.mic_on.get();
    app.media.mic_on.set(on);
    js::set(&track, "enabled", on);
    app.broadcast_state();
}

/// Turning the camera off releases it (the light goes out); turning it on
/// opens it again.
pub fn toggle_cam(app: &Rc<App>) {
    if let Some(track) = app.media.cam_track.take() {
        js::call(&track, "stop", &[]);
        app.media.cam_on.set(false);
        app.media.set_local_frame(None);
        app.broadcast_state();
        return;
    }
    let app = app.clone();
    spawn_local(async move {
        let stream = user_media(obj! { "video": video_constraints() }).await;
        if let Some(track) = stream.and_then(|stream| first_track(&stream, "getVideoTracks"))
            && app.media.cam_track.borrow().is_none()
        {
            start_camera(&app, track);
            app.broadcast_state();
        }
    });
}

fn reader(track: &JsValue) -> Option<JsValue> {
    let processor =
        js::try_new("MediaStreamTrackProcessor", &[obj! { "track": track.clone() }]).inspect_err(|error| js::log(&js::describe(error))).ok()?;
    Some(js::call(&js::get(&processor, "readable"), "getReader", &[]))
}

/// Reads frames or audio from a track until it ends.
async fn read(reader: &JsValue) -> Option<JsValue> {
    let result = js::call_async(reader, "read", &[]).await.ok()?;
    (!js::get(&result, "done").is_truthy()).then(|| js::get(&result, "value"))
}

fn chunk_bytes(chunk: &JsValue) -> Vec<u8> {
    let buffer = Uint8Array::new_with_length(js::num(chunk, "byteLength") as u32);
    js::call(chunk, "copyTo", &[buffer.clone().into()]);
    buffer.to_vec()
}

/// A WebCodecs encoder or decoder with the callbacks it calls into.
pub struct Decoder {
    js: JsValue,
    /// What it was configured for.
    config: String,
    broken: Rc<Cell<bool>>,
    _output: Closure<dyn FnMut(JsValue)>,
    _error: Closure<dyn FnMut(JsValue)>,
}

type Encoder = Decoder;

impl Decoder {
    fn new(class: &str, config_key: &str, config: JsValue, output: impl FnMut(JsValue) + 'static) -> Option<Self> {
        let broken = Rc::new(Cell::new(false));
        let output = Closure::<dyn FnMut(JsValue)>::new(output);
        let error = {
            let (broken, class) = (broken.clone(), class.to_string());
            Closure::<dyn FnMut(JsValue)>::new(move |error: JsValue| {
                broken.set(true);
                js::log(&format!("{class}: {}", js::describe(&error)));
            })
        };
        let init = obj! { "output": output.as_ref().clone(), "error": error.as_ref().clone() };
        let js = js::try_new(class, &[init]).inspect_err(|e| js::log(&js::describe(e))).ok()?;
        js::try_call(&js, "configure", &[config]).inspect_err(|e| js::log(&js::describe(e))).ok()?;
        Some(Self { js, config: config_key.into(), broken, _output: output, _error: error })
    }

    fn usable(&self, config: &str) -> bool {
        self.config == config && !self.broken.get() && js::text(&self.js, "state") == "configured"
    }

    fn queue(&self, property: &str) -> f64 {
        js::num(&self.js, property)
    }
}

impl Drop for Decoder {
    fn drop(&mut self) {
        if js::text(&self.js, "state") != "closed" {
            js::call(&self.js, "close", &[]);
        }
    }
}

fn video_bitrate(peers: usize) -> u32 {
    (UPLINK_BITS / peers.max(1) as u32).clamp(MIN_VIDEO_BITS, MAX_VIDEO_BITS)
}

/// The first codec this browser can encode at the given size, configured.
async fn video_encoder(app: &Rc<App>, width: u32, height: u32, peers: usize) -> Option<Encoder> {
    let class = js::get(&js::global(), "VideoEncoder");
    for codec in VIDEO_CODECS {
        let config = obj! {
            "codec": codec,
            "width": width,
            "height": height,
            "bitrate": video_bitrate(peers),
            "framerate": FRAME_RATE,
            "latencyMode": "realtime",
        };
        if codec.starts_with("avc1") {
            // Parameter sets travel in the key frames; no out-of-band description.
            js::set(&config, "avc", obj! { "format": "annexb" });
        }
        let supported = match js::try_call(&class, "isConfigSupported", &[config.clone()]) {
            Ok(promise) => js::wait(promise).await.is_ok_and(|support| js::get(&support, "supported").is_truthy()),
            Err(_) => false,
        };
        if !supported {
            continue;
        }
        let app = app.clone();
        let key = format!("{codec} {width}x{height} {peers}");
        return Encoder::new("VideoEncoder", &key, config, move |chunk: JsValue| {
            let is_key = js::text(&chunk, "type") == "key";
            let timestamp = js::num(&chunk, "timestamp") as u64;
            app.send_video(is_key, timestamp, Bytes::from(chunk_bytes(&chunk)), codec, width, height);
        });
    }
    js::log("no video codec can be encoded here");
    None
}

/// Shows the camera in the local tile and, while anyone is listening,
/// encodes it. The encoder is rebuilt when the size or the number of peers
/// (and so the bitrate) changes; its first frame is a key frame.
async fn camera_loop(app: Rc<App>, track: JsValue) {
    let Some(reader) = reader(&track) else { return };
    let mut encoder: Option<Encoder> = None;
    let mut since_key = 0;
    while let Some(frame) = read(&reader).await {
        let stopped = app.media.cam_track.borrow().as_ref() != Some(&track);
        if stopped {
            js::call(&frame, "close", &[]);
            break;
        }
        let (width, height) = (js::num(&frame, "displayWidth") as u32, js::num(&frame, "displayHeight") as u32);
        let peers = app.links.borrow().len();
        if peers == 0 {
            encoder = None;
        } else {
            let config = |codec: &str| format!("{codec} {width}x{height} {peers}");
            if !encoder.as_ref().is_some_and(|e| VIDEO_CODECS.iter().any(|codec| e.usable(&config(codec)))) {
                encoder = video_encoder(&app, width, height, peers).await;
                since_key = KEY_INTERVAL;
            }
            if let Some(encoder) = encoder.as_ref().filter(|e| e.queue("encodeQueueSize") < 3.0) {
                let key = app.media.take_key() || since_key >= KEY_INTERVAL;
                since_key = if key { 1 } else { since_key + 1 };
                js::call(&encoder.js, "encode", &[frame.clone(), obj! { "keyFrame": key }]);
            }
        }
        app.media.set_local_frame(Some(frame));
    }
    app.media.set_local_frame(None);
}

/// Encodes the microphone to Opus and sends each packet as a datagram.
async fn microphone_loop(app: Rc<App>, track: JsValue) {
    let Some(reader) = reader(&track) else { return };
    let mut encoder: Option<Encoder> = None;
    while let Some(data) = read(&reader).await {
        if app.media.mic_on.get() && !app.links.borrow().is_empty() {
            let (rate, channels) = (js::num(&data, "sampleRate") as u32, js::num(&data, "numberOfChannels") as u32);
            let key = format!("{rate} {channels}");
            if !encoder.as_ref().is_some_and(|e| e.usable(&key)) {
                let config = obj! {
                    "codec": "opus",
                    "sampleRate": rate,
                    "numberOfChannels": channels,
                    "bitrate": AUDIO_BITS_PER_CHANNEL * channels,
                };
                let app = app.clone();
                encoder = Encoder::new("AudioEncoder", &key, config, move |chunk: JsValue| {
                    let seq = app.media.audio_seq.get();
                    app.media.audio_seq.set(seq.wrapping_add(1));
                    let mut datagram = vec![proto::DATAGRAM_AUDIO, (seq >> 8) as u8, seq as u8, channels as u8];
                    datagram.extend_from_slice(&chunk_bytes(&chunk));
                    let datagram = Bytes::from(datagram);
                    for link in app.links.borrow().values() {
                        link.send_datagram(datagram.clone());
                    }
                });
            }
            if let Some(encoder) = &encoder {
                js::call(&encoder.js, "encode", &[data.clone()]);
            }
        }
        js::call(&data, "close", &[]);
    }
}

/// Decodes one video frame from a peer; `false` means the rest of its group
/// cannot be shown (the decoder failed or is too far behind).
pub fn decode_video(link: &Rc<Link>, codec: &str, key: bool, timestamp_us: u64, data: &[u8]) -> bool {
    let mut slot = link.video_decoder.borrow_mut();
    if key && !slot.as_ref().is_some_and(|decoder| decoder.usable(codec)) {
        let weak: Weak<Link> = Rc::downgrade(link);
        let config = obj! { "codec": codec, "optimizeForLatency": true };
        *slot = Decoder::new("VideoDecoder", codec, config, move |frame: JsValue| match weak.upgrade() {
            Some(link) => link.set_frame(Some(frame)),
            None => drop(js::call(&frame, "close", &[])),
        });
    }
    let Some(decoder) = slot.as_ref().filter(|decoder| decoder.usable(codec)) else { return false };
    if decoder.queue("decodeQueueSize") > 30.0 {
        return false;
    }
    let chunk = obj! {
        "type": if key { "key" } else { "delta" },
        "timestamp": timestamp_us as f64,
        "data": Uint8Array::from(data),
    };
    js::try_new("EncodedVideoChunk", &[chunk]).and_then(|chunk| js::try_call(&decoder.js, "decode", &[chunk])).is_ok()
}

/// Decodes one Opus packet from a peer; the decoder's output is scheduled
/// on the audio context right after the previous buffer.
pub fn play_audio(app: &Rc<App>, link: &Rc<Link>, seq: u16, channels: u8, data: &[u8]) {
    // Late and duplicate packets are dropped.
    if link.audio_seq.get().is_some_and(|last| (seq.wrapping_sub(last) as i16) <= 0) {
        return;
    }
    link.audio_seq.set(Some(seq));
    let key = channels.to_string();
    let mut slot = link.audio_decoder.borrow_mut();
    if !slot.as_ref().is_some_and(|decoder| decoder.usable(&key)) {
        let weak: Weak<Link> = Rc::downgrade(link);
        let context = app.media.audio_context.clone();
        let config = obj! { "codec": "opus", "sampleRate": 48000, "numberOfChannels": channels };
        *slot = Decoder::new("AudioDecoder", &key, config, move |audio: JsValue| {
            if let Some(link) = weak.upgrade() {
                schedule(&context, &link, &audio);
            }
            js::call(&audio, "close", &[]);
        });
    }
    let Some(decoder) = slot.as_ref() else { return };
    let chunk = obj! { "type": "key", "timestamp": f64::from(seq) * AUDIO_FRAME_US, "data": Uint8Array::from(data) };
    if let Ok(chunk) = js::try_new("EncodedAudioChunk", &[chunk]) {
        js::try_call(&decoder.js, "decode", &[chunk]).ok();
    }
}

/// Copies decoded audio into an `AudioBuffer` and starts it when the
/// previous one ends, keeping a small cushion against jitter.
fn schedule(context: &JsValue, link: &Link, audio: &JsValue) {
    let frames = js::num(audio, "numberOfFrames") as u32;
    let channels = js::num(audio, "numberOfChannels") as u32;
    let rate = js::num(audio, "sampleRate");
    if frames == 0 || js::text(context, "state") != "running" {
        return;
    }
    let buffer = js::call(context, "createBuffer", &[channels.into(), frames.into(), rate.into()]);
    for channel in 0..channels {
        let samples = Float32Array::new_with_length(frames);
        js::call(audio, "copyTo", &[samples.clone().into(), obj! { "planeIndex": channel, "format": "f32-planar" }]);
        js::call(&buffer, "copyToChannel", &[samples.clone().into(), channel.into()]);
        if channel == 0 {
            let power = samples.to_vec().iter().map(|s| s * s).sum::<f32>() / frames as f32;
            link.level.set(link.level.get().max((power.sqrt() * 4.0).min(1.0)));
        }
    }
    let now = js::num(context, "currentTime");
    let mut at = link.next_audio_time.get();
    if at < now + 0.005 {
        at = now + AUDIO_CUSHION;
    } else if at > now + AUDIO_BACKLOG_LIMIT {
        // Too far behind real time: skip this buffer to catch up.
        return;
    }
    let source = js::call(context, "createBufferSource", &[]);
    js::set(&source, "buffer", buffer);
    js::call(&source, "connect", &[js::get(context, "destination")]);
    js::call(&source, "start", &[at.into()]);
    link.next_audio_time.set(at + f64::from(frames) / rate);
}

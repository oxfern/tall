//! tall: video chat that tiles like xmonad. The page is one canvas; this
//! module is the application: it owns the window stack and layout, draws the
//! bar and the tiles every frame, and turns keys and taps into xmonad's
//! actions.

mod gpu;
mod js;
mod media;
mod net;

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use iroh::{RelayUrl, SecretKey};
use tall_core::layout::{Layout, Rect, Stack};
use tall_core::proto::{self, Id};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use gpu::{Gpu, Quad, Source};
use media::Media;
use net::Link;

// xmonad's defaults.
const NORMAL_BORDER: [f32; 4] = [0.867, 0.867, 0.867, 1.0]; // #dddddd
const FOCUSED_BORDER: [f32; 4] = [1.0, 0.0, 0.0, 1.0]; // #ff0000
const SPEAKING: [f32; 4] = [0.2, 0.9, 0.4, 1.0];
const MUTED: [f32; 4] = [1.0, 0.25, 0.2, 1.0];
/// Logical pixels; scaled by the device pixel ratio.
const BAR_HEIGHT: f64 = 24.0;
const BORDER: f64 = 1.5;
/// A peer at least this loud is shown as speaking.
const SPEAKING_LEVEL: f32 = 0.06;

#[derive(Clone, Copy, PartialEq)]
enum Action {
    None,
    NextLayout,
    ToggleMic,
    ToggleCam,
}

pub struct App {
    relay: RelayUrl,
    me: Id,
    /// Whether the hub has been heard from recently.
    online: Cell<bool>,
    /// Whether a roster has arrived yet.
    online_before: Cell<bool>,
    /// The hub turned this endpoint away: the channel has its seven.
    full: Cell<bool>,
    /// A problem the user should see in the bar.
    trouble: RefCell<String>,
    /// The channel's members, in join order.
    roster: RefCell<Vec<Id>>,
    links: RefCell<HashMap<Id, Rc<Link>>>,
    dialing: RefCell<HashSet<Id>>,
    stack: RefCell<Stack<Id>>,
    layout: Cell<Layout>,
    media: Media,
    gpu: Gpu,
    /// The bar: its 2D canvas, what it shows, and where its buttons are.
    bar_canvas: RefCell<Option<JsValue>>,
    /// The loading screen: its 2D canvas and the animation step it shows.
    splash_canvas: RefCell<Option<JsValue>>,
    splash_step: Cell<(u64, u32, u32)>,
    /// Whether the endpoint has reached the relay.
    bound: Cell<bool>,
    bar_text: RefCell<String>,
    bar_hits: RefCell<Vec<(f64, f64, Action)>>,
    /// Where each tile was drawn last, for taps.
    tiles: RefCell<Vec<(Id, Rect)>>,
}

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
    spawn_local(async {
        if let Err(error) = run().await {
            // Without a renderer there is nowhere to draw the message.
            js::set(&js::get(&js::global(), "document"), "title", format!("tall: {error}"));
            js::log(&error);
        }
    });
}

async fn run() -> Result<(), String> {
    let window = js::global();
    let canvas = js::call(&js::get(&window, "document"), "querySelector", &["canvas".into()]);
    let gpu = Gpu::new(canvas).await?;
    let relay: RelayUrl = js::text(&js::get(&window, "location"), "origin").parse().map_err(|_| "bad origin".to_string())?;
    let key = SecretKey::generate();
    let me: Id = *key.public().as_bytes();
    let app = Rc::new(App {
        relay: relay.clone(),
        me,
        online: Cell::new(false),
        full: Cell::new(false),
        online_before: Cell::new(false),
        trouble: RefCell::new(String::new()),
        roster: RefCell::new(vec![me]),
        links: RefCell::new(HashMap::new()),
        dialing: RefCell::new(HashSet::new()),
        stack: RefCell::new(Stack { items: vec![me], focus: 0 }),
        layout: Cell::new(Layout::default()),
        media: Media::new(),
        gpu,
        bar_canvas: RefCell::new(None),
        splash_canvas: RefCell::new(None),
        splash_step: Cell::new((0, 0, 0)),
        bound: Cell::new(false),
        bar_text: RefCell::new(String::new()),
        bar_hits: RefCell::new(Vec::new()),
        tiles: RefCell::new(Vec::new()),
    });

    let on_key = app.clone();
    js::listen(&window, "keydown", move |event: JsValue| on_key.key(&event));
    let on_pointer = app.clone();
    js::listen(&window, "pointerdown", move |event: JsValue| on_pointer.pointer(&event));
    let on_leave = app.clone();
    js::listen(&window, "pagehide", move |_: JsValue| {
        for link in on_leave.links.borrow().values() {
            link.close();
        }
    });

    // Draw from the first frame on; the network and the devices join in.
    let frame: Rc<RefCell<Option<Closure<dyn FnMut(JsValue)>>>> = Rc::new(RefCell::new(None));
    let (again, drawn) = (frame.clone(), app.clone());
    let request = |frame: &Closure<dyn FnMut(JsValue)>| drop(js::call(&js::global(), "requestAnimationFrame", &[frame.as_ref().clone()]));
    *frame.borrow_mut() = Some(Closure::new(move |_: JsValue| {
        drawn.frame();
        request(again.borrow().as_ref().expect("frame callback"));
    }));
    request(frame.borrow().as_ref().expect("frame callback"));

    spawn_local(media::start(app.clone()));
    let ticker = app.clone();
    js::call(&window, "setInterval", &[js::forever(move |_: JsValue| ticker.broadcast_state()), 500.into()]);

    let endpoint = match net::bind(key, &relay).await {
        Ok(endpoint) => endpoint,
        Err(error) => {
            *app.trouble.borrow_mut() = error.clone();
            return Err(error);
        }
    };
    app.bound.set(true);
    spawn_local(net::accept_loop(app.clone(), endpoint.clone()));
    spawn_local(net::hub_loop(app, endpoint));
    Ok(())
}

impl App {
    /// Tells every peer whether the microphone and camera are on. Sent on
    /// every change and twice a second, which also keeps the links alive.
    pub fn broadcast_state(&self) {
        for link in self.links.borrow().values() {
            self.send_state(link);
        }
    }

    fn act(self: &Rc<Self>, action: Action) {
        match action {
            Action::None => {}
            Action::NextLayout => self.update_layout(Layout::next),
            Action::ToggleMic => media::toggle_mic(self),
            Action::ToggleCam => media::toggle_cam(self),
        }
    }

    fn update_layout(&self, change: impl FnOnce(&mut Layout)) {
        let mut layout = self.layout.get();
        change(&mut layout);
        self.layout.set(layout);
    }

    /// xmonad's default bindings, without the modifier.
    fn key(self: &Rc<Self>, event: &JsValue) {
        self.media.resume_sound();
        if ["metaKey", "ctrlKey", "altKey"].iter().any(|modifier| js::get(event, modifier).is_truthy()) {
            return;
        }
        let mut stack = self.stack.borrow_mut();
        match js::text(event, "key").as_str() {
            "j" | "ArrowDown" | "ArrowRight" | "Tab" => stack.focus_down(),
            "k" | "ArrowUp" | "ArrowLeft" => stack.focus_up(),
            "J" => stack.swap_down(),
            "K" => stack.swap_up(),
            "Enter" => stack.swap_master(),
            "h" => self.update_layout(|layout| layout.tall.shrink()),
            "l" => self.update_layout(|layout| layout.tall.expand()),
            "," => self.update_layout(|layout| layout.tall.inc_master(1)),
            "." => self.update_layout(|layout| layout.tall.inc_master(-1)),
            " " => self.act(Action::NextLayout),
            "m" => self.act(Action::ToggleMic),
            "v" => self.act(Action::ToggleCam),
            _ => return,
        }
        js::call(event, "preventDefault", &[]);
    }

    /// A tap on the bar presses a button; on a tile it focuses it, and on the
    /// focused tile makes it the master.
    fn pointer(self: &Rc<Self>, event: &JsValue) {
        self.media.resume_sound();
        let scale = js::num(&js::global(), "devicePixelRatio");
        let (x, y) = (js::num(event, "clientX") * scale, js::num(event, "clientY") * scale);
        if y < (BAR_HEIGHT * scale).round() {
            let action = self.bar_hits.borrow().iter().find(|(from, to, _)| (*from..*to).contains(&x)).map(|hit| hit.2);
            return self.act(action.unwrap_or(Action::None));
        }
        let hit = self.tiles.borrow().iter().find(|(_, rect)| rect.contains(x as i32, y as i32)).map(|(id, _)| *id);
        if let Some(id) = hit {
            let mut stack = self.stack.borrow_mut();
            if stack.focused() == Some(&id) {
                stack.swap_master();
            } else {
                stack.focus_item(&id);
            }
        }
    }

    /// Redraws the bar when what it says changes. The text is set on an
    /// `OffscreenCanvas` and uploaded as a texture.
    fn bar(&self, width: u32, height: u32, scale: f64) {
        let on = |on: bool| if on { "on" } else { "off" };
        let members = self.roster.borrow().len();
        let status = if !self.trouble.borrow().is_empty() {
            self.trouble.borrow().clone()
        } else if self.full.get() {
            format!("the channel is full ({} here), waiting for a place", proto::MAX_MEMBERS)
        } else if !self.online.get() {
            "connecting".into()
        } else if self.media.sound_blocked() {
            "click for sound".into()
        } else if members == 1 {
            "nobody else is here yet".into()
        } else {
            String::new()
        };
        let segments: [(String, &str, Action); 7] = [
            ("tall".into(), "#ff0000", Action::None),
            (format!("[{}]", self.layout.get().description()), "#dddddd", Action::NextLayout),
            (format!("mic {}", on(self.media.mic_on.get())), if self.media.mic_on.get() { "#dddddd" } else { "#ff4433" }, Action::ToggleMic),
            (format!("cam {}", on(self.media.cam_on.get())), if self.media.cam_on.get() { "#dddddd" } else { "#ff4433" }, Action::ToggleCam),
            (format!("{members} here"), "#888888", Action::None),
            (status, "#ffcc00", Action::None),
            ("j/k focus  enter master  h/l resize  ,/. masters  space layout  m mic  v cam".into(), "#555555", Action::None),
        ];
        let text = format!("{width}x{height} {}", segments.iter().map(|s| s.0.as_str()).collect::<Vec<_>>().join("|"));
        if *self.bar_text.borrow() == text {
            return;
        }
        *self.bar_text.borrow_mut() = text;

        let mut slot = self.bar_canvas.borrow_mut();
        let canvas = slot.get_or_insert_with(|| js::try_new("OffscreenCanvas", &[width.into(), height.into()]).expect("OffscreenCanvas"));
        js::set(canvas, "width", width);
        js::set(canvas, "height", height);
        let context = js::call(canvas, "getContext", &["2d".into()]);
        js::set(&context, "fillStyle", "#000000");
        js::call(&context, "fillRect", &[0.into(), 0.into(), width.into(), height.into()]);
        js::set(&context, "font", format!("{}px ui-monospace, Menlo, Consolas, monospace", (13.0 * scale).round()));
        js::set(&context, "textBaseline", "middle");
        let mut hits = Vec::new();
        let (gap, mut x) = (14.0 * scale, 8.0 * scale);
        for (label, color, action) in segments.iter().filter(|segment| !segment.0.is_empty()) {
            js::set(&context, "fillStyle", *color);
            js::call(&context, "fillText", &[label.as_str().into(), x.into(), (f64::from(height) / 2.0 + scale).into()]);
            let end = x + js::num(&js::call(&context, "measureText", &[label.as_str().into()]), "width");
            hits.push((x - gap / 2.0, end + gap / 2.0, *action));
            x = end + gap;
        }
        *self.bar_hits.borrow_mut() = hits;
        self.gpu.set_image(gpu::BAR, canvas, width, height);
    }

    /// The loading screen, in ASCII: the name, xmonad's `tile` filling a
    /// character grid with one to seven windows, and what is being waited for.
    fn splash(&self, width: u32, height: u32, scale: f64) {
        const LOGO: [&str; 5] = [r" _        _ _ ", r"| |_ __ _| | |", r"| __/ _` | | |", r"| || (_| | | |", r" \__\__,_|_|_|"];
        const COLUMNS: usize = 49;
        const ROWS: usize = 17;
        let step = (js_sys::Date::now() / 125.0) as u64;
        if self.splash_step.replace((step, width, height)) == (step, width, height) {
            return;
        }

        // The windows: borders on a character grid, neighbours sharing edges.
        let mut grid = vec![vec![' '; COLUMNS]; ROWS];
        let windows = 1 + (step / 5) % proto::MAX_MEMBERS as u64;
        let screen = Rect::new(0, 0, COLUMNS as u32 - 1, ROWS as u32 - 1);
        for (number, rect) in tall_core::layout::tile(50, screen, 1, windows as u32).into_iter().enumerate() {
            let (left, top) = (rect.x as usize, rect.y as usize);
            let (right, bottom) = (left + rect.w as usize, top + rect.h as usize);
            for x in left..=right {
                for y in top..=bottom {
                    let (on_column, on_row) = (x == left || x == right, y == top || y == bottom);
                    grid[y][x] = match (on_column, on_row) {
                        (true, true) => '+',
                        (true, false) if grid[y][x] != '+' => '|',
                        (false, true) if grid[y][x] != '+' => '-',
                        _ => grid[y][x],
                    };
                }
            }
            if rect.h >= 2 {
                grid[top + rect.h as usize / 2][left + rect.w as usize / 2] = char::from(b'1' + number as u8);
            }
        }

        let waiting = if !self.trouble.borrow().is_empty() {
            self.trouble.borrow().clone()
        } else if self.full.get() {
            format!("the channel is full ({} here), waiting for a place", proto::MAX_MEMBERS)
        } else if self.bound.get() {
            "joining the channel".into()
        } else {
            "reaching the relay".into()
        };
        let spinner = ['|', '/', '-', '\\'][(step % 4) as usize];
        let centered = |text: &str| format!("{}{text}", " ".repeat(COLUMNS.saturating_sub(text.chars().count()) / 2));
        let mut lines: Vec<(String, &str)> = LOGO.iter().map(|line| (centered(line), "#ff0000")).collect();
        lines.push((String::new(), ""));
        lines.extend(grid.into_iter().map(|row| (row.into_iter().collect(), "#dddddd")));
        lines.push((String::new(), ""));
        lines.push((centered(&format!("{spinner} {waiting}")), "#ffcc00"));
        lines.push((centered("iroh / webgpu / xmonad"), "#555555"));

        let mut slot = self.splash_canvas.borrow_mut();
        let canvas = slot.get_or_insert_with(|| js::try_new("OffscreenCanvas", &[width.into(), height.into()]).expect("OffscreenCanvas"));
        if (js::num(canvas, "width"), js::num(canvas, "height")) != (f64::from(width), f64::from(height)) {
            js::set(canvas, "width", width);
            js::set(canvas, "height", height);
        }
        let context = js::call(canvas, "getContext", &["2d".into()]);
        js::set(&context, "fillStyle", "#000000");
        js::call(&context, "fillRect", &[0.into(), 0.into(), width.into(), height.into()]);
        // As large as fits, up to a comfortable terminal size.
        let size = (16.0 * scale).min(f64::from(width) / (COLUMNS as f64 * 0.62)).min(f64::from(height) / (lines.len() as f64 * 1.25)).floor();
        js::set(&context, "font", format!("{size}px ui-monospace, Menlo, Consolas, monospace"));
        js::set(&context, "textBaseline", "top");
        let column = js::num(&js::call(&context, "measureText", &["M".into()]), "width");
        let line_height = (size * 1.2).round();
        let x = ((f64::from(width) - column * COLUMNS as f64) / 2.0).round();
        let mut y = ((f64::from(height) - line_height * lines.len() as f64) / 2.0).round();
        for (text, color) in &lines {
            if !text.is_empty() {
                js::set(&context, "fillStyle", *color);
                js::call(&context, "fillText", &[text.as_str().into(), x.into(), y.into()]);
            }
            y += line_height;
        }
        self.gpu.set_image(gpu::SPLASH, canvas, width, height);
    }

    /// Draws one frame: the bar, then every tile where xmonad's layout puts it.
    fn frame(&self) {
        let window = js::global();
        let scale = js::num(&window, "devicePixelRatio").max(1.0);
        let width = ((js::num(&window, "innerWidth") * scale).round() as u32).max(1);
        let height = ((js::num(&window, "innerHeight") * scale).round() as u32).max(1);
        self.gpu.resize(width, height);
        let (width, height) = self.gpu.size();
        let bar_height = ((BAR_HEIGHT * scale).round() as u32).min(height);
        self.bar(width, bar_height, scale);

        let border = (BORDER * scale).round().max(1.0) as u32;
        let mut quads = vec![Quad {
            rect: [0.0, 0.0, width as f32, bar_height as f32],
            uv: [0.0, 0.0, 1.0, 1.0],
            color: [1.0; 4],
            source: Source::Image(gpu::BAR),
        }];
        if !self.online_before.get() {
            // Not in the channel yet: the loading screen takes the place of the tiles.
            let splash_height = height - bar_height;
            self.splash(width, splash_height, scale);
            let rect = [0.0, bar_height as f32, width as f32, splash_height as f32];
            quads.push(Quad { rect, uv: [0.0, 0.0, 1.0, 1.0], color: [1.0; 4], source: Source::Image(gpu::SPLASH) });
            return self.gpu.draw(&quads);
        }
        let stack = self.stack.borrow();
        let screen = Rect::new(0, bar_height as i32, width, height - bar_height);
        let rects = self.layout.get().run(screen, stack.len() as u32, stack.focus);
        let links = self.links.borrow();
        let mut tiles = Vec::new();
        for (index, (id, rect)) in stack.items.iter().zip(rects).enumerate() {
            let Some(rect) = rect.filter(|rect| rect.w > 0 && rect.h > 0) else { continue };
            tiles.push((*id, rect));
            let link = links.get(id);
            let (flags, frame, level) = match link {
                _ if *id == self.me => (self.media.flags(), self.media.local_frame.borrow().clone(), 0.0),
                Some(link) => {
                    let level = link.level.get();
                    link.level.set(level * 0.92);
                    (link.flags.get(), link.frame.borrow().clone(), level)
                }
                None => (0, None, 0.0),
            };
            let speaking = flags & proto::FLAG_MIC != 0 && level > SPEAKING_LEVEL;
            let color = if speaking {
                SPEAKING
            } else if index == stack.focus {
                FOCUSED_BORDER
            } else {
                NORMAL_BORDER
            };
            quads.push(Quad::solid(as_floats(rect), color));
            let inner = rect.inset(if speaking { 2 * border } else { border });
            match frame.filter(|_| flags & proto::FLAG_CAM != 0) {
                Some(frame) => {
                    let aspect = js::num(&frame, "displayWidth") / js::num(&frame, "displayHeight").max(1.0);
                    quads.push(Quad {
                        rect: as_floats(inner),
                        uv: cover(aspect, inner, *id == self.me),
                        color: [1.0; 4],
                        source: Source::Video(frame),
                    });
                }
                None => {
                    // No picture: a square in the member's colour. Dim until linked.
                    let tint = if link.is_some() || *id == self.me { 1.0 } else { 0.35 };
                    let [r, g, b] = member_color(id).map(|c| c * tint);
                    quads.push(Quad::solid(as_floats(inner), [r * 0.18, g * 0.18, b * 0.18, 1.0]));
                    let side = inner.w.min(inner.h) / 4;
                    let badge = Rect::new(inner.x + ((inner.w - side) / 2) as i32, inner.y + ((inner.h - side) / 2) as i32, side, side);
                    quads.push(Quad::solid(as_floats(badge), [r, g, b, 1.0]));
                }
            }
            if flags & proto::FLAG_MIC == 0 && (link.is_some() || *id == self.me) {
                let side = (10.0 * scale).round() as u32;
                let corner = Rect::new(inner.x + inner.w as i32 - 2 * side as i32, inner.y + side as i32, side, side);
                if inner.w > 3 * side && inner.h > 3 * side {
                    quads.push(Quad::solid(as_floats(corner), MUTED));
                }
            }
        }
        *self.tiles.borrow_mut() = tiles;
        self.gpu.draw(&quads);
    }
}

fn as_floats(rect: Rect) -> [f32; 4] {
    [rect.x as f32, rect.y as f32, rect.w as f32, rect.h as f32]
}

/// Texture coordinates that fill `rect` with a picture of the given aspect
/// ratio, cropping the overflow evenly; the own picture is mirrored.
fn cover(aspect: f64, rect: Rect, mirror: bool) -> [f32; 4] {
    let tile = f64::from(rect.w) / f64::from(rect.h.max(1));
    let (w, h) = if aspect > tile { (tile / aspect, 1.0) } else { (1.0, aspect / tile) };
    let (x, y) = ((1.0 - w) / 2.0, (1.0 - h) / 2.0);
    if mirror { [(x + w) as f32, y as f32, -w as f32, h as f32] } else { [x as f32, y as f32, w as f32, h as f32] }
}

/// A stable, saturated colour for a member, from its endpoint id.
fn member_color(id: &Id) -> [f32; 3] {
    let hue = f32::from(id[0]) / 256.0 * 6.0;
    let channel = |shift: f32| {
        let k = (shift + hue) % 6.0;
        1.0 - 0.75 * k.min(4.0 - k).clamp(0.0, 1.0)
    };
    [channel(5.0), channel(3.0), channel(1.0)]
}

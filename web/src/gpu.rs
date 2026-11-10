//! The renderer: every pixel is a textured quad drawn with WebGPU. Video
//! tiles sample `VideoFrame`s directly as external textures (no copy); the
//! bar is a texture; borders and placeholders are a white texel times a
//! colour.

use std::cell::{Cell, RefCell};

use js_sys::Float32Array;
use wasm_bindgen::JsValue;

use crate::{arr, js, obj};

const SHADER: &str = r"
struct Quad { rect: vec4f, uv: vec4f, color: vec4f, screen: vec4f }
@group(0) @binding(0) var<uniform> quad: Quad;
@group(1) @binding(0) var samp: sampler;
struct Vertex { @builtin(position) position: vec4f, @location(0) uv: vec2f }

@vertex fn vertex(@builtin(vertex_index) index: u32) -> Vertex {
    let corner = vec2f(f32(index & 1u), f32(index >> 1u));
    let pixel = quad.rect.xy + corner * quad.rect.zw;
    var out: Vertex;
    out.position = vec4f(pixel.x / quad.screen.x * 2.0 - 1.0, 1.0 - pixel.y / quad.screen.y * 2.0, 0.0, 1.0);
    out.uv = quad.uv.xy + corner * quad.uv.zw;
    return out;
}
";
const FRAGMENT_TEXTURE: &str = r"
@group(1) @binding(1) var image: texture_2d<f32>;
@fragment fn fragment(in: Vertex) -> @location(0) vec4f {
    return textureSample(image, samp, in.uv) * quad.color;
}
";
const FRAGMENT_VIDEO: &str = r"
@group(1) @binding(1) var image: texture_external;
@fragment fn fragment(in: Vertex) -> @location(0) vec4f {
    return textureSampleBaseClampToEdge(image, samp, in.uv) * quad.color;
}
";

/// Bytes between quads in the uniform buffer (the alignment WebGPU requires
/// for dynamic offsets) and how many fit.
const SLOT: u32 = 256;
const MAX_QUADS: usize = 128;

const VERTEX: u32 = 1;
const FRAGMENT: u32 = 2;
const COPY_DST: u32 = 0x08;
const UNIFORM: u32 = 0x40;
const TEXTURE_COPY_DST: u32 = 0x02;
const TEXTURE_BINDING: u32 = 0x04;
const RENDER_ATTACHMENT: u32 = 0x10;

/// The image slots.
pub const BAR: usize = 0;
pub const SPLASH: usize = 1;

pub enum Source {
    /// A flat colour.
    Solid,
    /// A texture uploaded with `set_image`: [`BAR`] or [`SPLASH`].
    Image(usize),
    /// A `VideoFrame`.
    Video(JsValue),
}

pub struct Quad {
    /// x, y, width, height in canvas pixels.
    pub rect: [f32; 4],
    /// Texture coordinates of the top-left corner, and their extent.
    pub uv: [f32; 4],
    pub color: [f32; 4],
    pub source: Source,
}

impl Quad {
    pub fn solid(rect: [f32; 4], color: [f32; 4]) -> Self {
        Self { rect, uv: [0.0, 0.0, 1.0, 1.0], color, source: Source::Solid }
    }
}

pub struct Gpu {
    canvas: JsValue,
    context: JsValue,
    device: JsValue,
    queue: JsValue,
    texture_pipeline: JsValue,
    video_pipeline: JsValue,
    texture_layout: JsValue,
    video_layout: JsValue,
    sampler: JsValue,
    uniforms: JsValue,
    uniform_group: JsValue,
    white_group: JsValue,
    /// Per image slot: the texture, its bind group and its size.
    images: [RefCell<Option<(JsValue, JsValue, u32, u32)>>; 2],
    size: Cell<(u32, u32)>,
}

fn entry(binding: u32, resource: JsValue) -> JsValue {
    obj! { "binding": binding, "resource": resource }
}

impl Gpu {
    pub async fn new(canvas: JsValue) -> Result<Self, String> {
        let describe = |error: JsValue| js::describe(&error);
        let gpu = js::get(&js::get(&js::global(), "navigator"), "gpu");
        if !js::object_is(&gpu) {
            return Err("this browser has no WebGPU".into());
        }
        let adapter = js::call_async(&gpu, "requestAdapter", &[]).await.map_err(describe)?;
        if !js::object_is(&adapter) {
            return Err("no WebGPU adapter".into());
        }
        let device = js::call_async(&adapter, "requestDevice", &[]).await.map_err(describe)?;
        let queue = js::get(&device, "queue");
        let context = js::call(&canvas, "getContext", &["webgpu".into()]);
        let format = js::call(&gpu, "getPreferredCanvasFormat", &[]);
        js::call(&context, "configure", &[obj! { "device": device.clone(), "format": format.clone(), "alphaMode": "opaque" }]);

        let layout = |entries: JsValue| js::call(&device, "createBindGroupLayout", &[obj! { "entries": entries }]);
        let uniform_layout = layout(arr![obj! {
            "binding": 0,
            "visibility": VERTEX | FRAGMENT,
            "buffer": obj! { "type": "uniform", "hasDynamicOffset": true, "minBindingSize": 64 },
        }]);
        let sampler_entry = || obj! { "binding": 0, "visibility": FRAGMENT, "sampler": obj! {} };
        let texture_layout = layout(arr![sampler_entry(), obj! { "binding": 1, "visibility": FRAGMENT, "texture": obj! {} }]);
        let video_layout = layout(arr![sampler_entry(), obj! { "binding": 1, "visibility": FRAGMENT, "externalTexture": obj! {} }]);

        let pipeline = |fragment: &str, layout: &JsValue| {
            let module = js::call(&device, "createShaderModule", &[obj! { "code": format!("{SHADER}{fragment}") }]);
            let blend = obj! { "srcFactor": "src-alpha", "dstFactor": "one-minus-src-alpha" };
            let target = obj! { "format": format.clone(), "blend": obj! { "color": blend.clone(), "alpha": blend } };
            js::call(
                &device,
                "createRenderPipeline",
                &[obj! {
                    "layout": js::call(&device, "createPipelineLayout", &[obj! { "bindGroupLayouts": arr![uniform_layout.clone(), layout.clone()] }]),
                    "vertex": obj! { "module": module.clone(), "entryPoint": "vertex" },
                    "fragment": obj! { "module": module, "entryPoint": "fragment", "targets": arr![target] },
                    "primitive": obj! { "topology": "triangle-strip" },
                }],
            )
        };
        let texture_pipeline = pipeline(FRAGMENT_TEXTURE, &texture_layout);
        let video_pipeline = pipeline(FRAGMENT_VIDEO, &video_layout);

        let sampler = js::call(&device, "createSampler", &[obj! { "magFilter": "linear", "minFilter": "linear" }]);
        let uniforms = js::call(&device, "createBuffer", &[obj! { "size": SLOT * MAX_QUADS as u32, "usage": UNIFORM | COPY_DST }]);
        let uniform_group = js::call(
            &device,
            "createBindGroup",
            &[obj! {
                "layout": uniform_layout,
                "entries": arr![entry(0, obj! { "buffer": uniforms.clone(), "size": 64 })],
            }],
        );

        let white = js::call(
            &device,
            "createTexture",
            &[obj! {
                "size": arr![1, 1],
                "format": "rgba8unorm",
                "usage": TEXTURE_BINDING | TEXTURE_COPY_DST,
            }],
        );
        let pixel = js_sys::Uint8Array::from(&[255u8, 255, 255, 255][..]);
        js::call(&queue, "writeTexture", &[obj! { "texture": white.clone() }, pixel.into(), obj! {}, arr![1, 1]]);
        let gpu = Self {
            canvas,
            context,
            queue,
            texture_pipeline,
            video_pipeline,
            video_layout,
            uniforms,
            uniform_group,
            white_group: JsValue::UNDEFINED,
            images: [RefCell::new(None), RefCell::new(None)],
            size: Cell::new((0, 0)),
            sampler,
            texture_layout,
            device,
        };
        let white_group = gpu.texture_group(&white);
        Ok(Self { white_group, ..gpu })
    }

    fn texture_group(&self, texture: &JsValue) -> JsValue {
        let view = js::call(texture, "createView", &[]);
        js::call(
            &self.device,
            "createBindGroup",
            &[obj! {
                "layout": self.texture_layout.clone(),
                "entries": arr![entry(0, self.sampler.clone()), entry(1, view)],
            }],
        )
    }

    pub fn size(&self) -> (u32, u32) {
        self.size.get()
    }

    /// Sets the canvas's pixel size.
    pub fn resize(&self, width: u32, height: u32) {
        if self.size.get() != (width, height) {
            js::set(&self.canvas, "width", width);
            js::set(&self.canvas, "height", height);
            self.size.set((width, height));
        }
    }

    /// Uploads text drawn on an `OffscreenCanvas` into an image slot.
    pub fn set_image(&self, slot: usize, image: &JsValue, width: u32, height: u32) {
        let mut bar = self.images[slot].borrow_mut();
        if !bar.as_ref().is_some_and(|(_, _, w, h)| (*w, *h) == (width, height)) {
            if let Some((old, ..)) = bar.take() {
                js::call(&old, "destroy", &[]);
            }
            let texture = js::call(
                &self.device,
                "createTexture",
                &[obj! {
                    "size": arr![width, height],
                    "format": "rgba8unorm",
                    "usage": TEXTURE_BINDING | TEXTURE_COPY_DST | RENDER_ATTACHMENT,
                }],
            );
            *bar = Some((texture.clone(), self.texture_group(&texture), width, height));
        }
        let (texture, ..) = bar.as_ref().expect("image texture");
        let source = obj! { "source": image.clone() };
        js::call(&self.queue, "copyExternalImageToTexture", &[source, obj! { "texture": texture.clone() }, arr![width, height]]);
    }

    /// Draws one frame: the quads in order, over black.
    pub fn draw(&self, quads: &[Quad]) {
        let quads = &quads[..quads.len().min(MAX_QUADS)];
        let (width, height) = self.size.get();
        let floats_per_slot = (SLOT / 4) as usize;
        let mut data = vec![0f32; quads.len().max(1) * floats_per_slot];
        for (i, quad) in quads.iter().enumerate() {
            let at = i * floats_per_slot;
            data[at..at + 4].copy_from_slice(&quad.rect);
            data[at + 4..at + 8].copy_from_slice(&quad.uv);
            data[at + 8..at + 12].copy_from_slice(&quad.color);
            data[at + 12] = width as f32;
            data[at + 13] = height as f32;
        }
        js::call(&self.queue, "writeBuffer", &[self.uniforms.clone(), 0.into(), Float32Array::from(&data[..]).into()]);

        let Ok(target) = js::try_call(&self.context, "getCurrentTexture", &[]) else { return };
        let encoder = js::call(&self.device, "createCommandEncoder", &[]);
        let pass = js::call(
            &encoder,
            "beginRenderPass",
            &[obj! {
                "colorAttachments": arr![obj! {
                    "view": js::call(&target, "createView", &[]),
                    "clearValue": obj! { "r": 0, "g": 0, "b": 0, "a": 1 },
                    "loadOp": "clear",
                    "storeOp": "store",
                }],
            }],
        );
        for (i, quad) in quads.iter().enumerate() {
            let (pipeline, group) = match &quad.source {
                Source::Solid => (&self.texture_pipeline, self.white_group.clone()),
                Source::Image(slot) => match self.images[*slot].borrow().as_ref() {
                    Some((_, group, ..)) => (&self.texture_pipeline, group.clone()),
                    None => continue,
                },
                // An external texture lives until the end of this frame.
                Source::Video(frame) => match js::try_call(&self.device, "importExternalTexture", &[obj! { "source": frame.clone() }]) {
                    Ok(texture) => {
                        let entries = arr![entry(0, self.sampler.clone()), entry(1, texture)];
                        let group = js::call(&self.device, "createBindGroup", &[obj! { "layout": self.video_layout.clone(), "entries": entries }]);
                        (&self.video_pipeline, group)
                    }
                    Err(_) => continue,
                },
            };
            js::call(&pass, "setPipeline", &[pipeline.clone()]);
            js::call(&pass, "setBindGroup", &[0.into(), self.uniform_group.clone(), arr![i as u32 * SLOT]]);
            js::call(&pass, "setBindGroup", &[1.into(), group]);
            js::call(&pass, "draw", &[4.into()]);
        }
        js::call(&pass, "end", &[]);
        js::call(&self.queue, "submit", &[arr![js::call(&encoder, "finish", &[])]]);
    }
}

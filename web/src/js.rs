//! Dynamic calls into the browser. WebGPU and WebCodecs are used exactly as
//! their JavaScript APIs are specified, through `Reflect`, so the code reads
//! like the specs and does not depend on generated bindings.

use js_sys::{Array, Function, Object, Promise, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

/// A JavaScript object literal: `obj! { "codec": "opus", "bitrate": 64000 }`.
#[macro_export]
macro_rules! obj {
    ($($key:literal : $value:expr),* $(,)?) => {{
        let object = ::js_sys::Object::new();
        $( ::js_sys::Reflect::set(&object, &::wasm_bindgen::JsValue::from_str($key), &::wasm_bindgen::JsValue::from($value)).expect("set"); )*
        ::wasm_bindgen::JsValue::from(object)
    }};
}

/// A JavaScript array literal.
#[macro_export]
macro_rules! arr {
    ($($value:expr),* $(,)?) => {{
        let array = ::js_sys::Array::new();
        $( array.push(&::wasm_bindgen::JsValue::from($value)); )*
        ::wasm_bindgen::JsValue::from(array)
    }};
}

pub fn global() -> JsValue {
    js_sys::global().into()
}

pub fn get(object: &JsValue, key: &str) -> JsValue {
    Reflect::get(object, &JsValue::from_str(key)).unwrap_or(JsValue::UNDEFINED)
}

pub fn set(object: &JsValue, key: &str, value: impl Into<JsValue>) {
    Reflect::set(object, &JsValue::from_str(key), &value.into()).expect("set");
}

pub fn num(object: &JsValue, key: &str) -> f64 {
    get(object, key).as_f64().unwrap_or(0.0)
}

pub fn text(object: &JsValue, key: &str) -> String {
    get(object, key).as_string().unwrap_or_default()
}

/// `object.method(...args)`, or the exception it threw.
pub fn try_call(object: &JsValue, method: &str, args: &[JsValue]) -> Result<JsValue, JsValue> {
    let function: Function = get(object, method).dyn_into().map_err(|_| JsValue::from_str(&format!("no method {method}")))?;
    function.apply(object, &args.iter().collect::<Array>())
}

/// `object.method(...args)`; an exception is logged and yields `undefined`.
pub fn call(object: &JsValue, method: &str, args: &[JsValue]) -> JsValue {
    try_call(object, method, args).unwrap_or_else(|error| {
        log(&format!("{method}: {}", describe(&error)));
        JsValue::UNDEFINED
    })
}

/// `new Constructor(...args)` for a global constructor.
pub fn try_new(constructor: &str, args: &[JsValue]) -> Result<JsValue, JsValue> {
    let function: Function = get(&global(), constructor).dyn_into().map_err(|_| JsValue::from_str(&format!("{constructor} is not available")))?;
    Reflect::construct(&function, &args.iter().collect::<Array>())
}

pub async fn wait(promise: JsValue) -> Result<JsValue, JsValue> {
    JsFuture::from(Promise::from(promise)).await
}

/// `await object.method(...args)`.
pub async fn call_async(object: &JsValue, method: &str, args: &[JsValue]) -> Result<JsValue, JsValue> {
    wait(try_call(object, method, args)?).await
}

pub fn describe(error: &JsValue) -> String {
    error.as_string().or_else(|| get(error, "message").as_string()).unwrap_or_else(|| format!("{error:?}"))
}

pub fn log(message: &str) {
    call(&get(&global(), "console"), "log", &[JsValue::from_str(&format!("tall: {message}"))]);
}

/// A callback that lives as long as the page.
pub fn forever<A: wasm_bindgen::convert::FromWasmAbi + 'static>(f: impl FnMut(A) + 'static) -> JsValue {
    Closure::<dyn FnMut(A)>::new(f).into_js_value()
}

pub fn listen<A: wasm_bindgen::convert::FromWasmAbi + 'static>(target: &JsValue, event: &str, f: impl FnMut(A) + 'static) {
    call(target, "addEventListener", &[JsValue::from_str(event), forever(f)]);
}

pub fn object_is(value: &JsValue) -> bool {
    value.is_instance_of::<Object>()
}

use godot::{
    classes::{DisplayServer, Input},
    global::Key,
};
use js::class::Trace;

use crate::prelude::*;

#[derive(js::JsLifetime, Debug)]
#[js::class]
pub struct JsInput {}

impl<'js> Trace<'js> for JsInput {
    fn trace<'a>(&self, _tracer: js::class::Tracer<'a, 'js>) {}
}

#[js::methods]
impl JsInput {
    #[qjs(rename = "isKeyPressed")]
    fn is_key_pressed(&self, key: i32) -> bool {
        let gd_key = Key::try_from_ord(key).unwrap_or(Key::NONE);
        Input::singleton().is_key_pressed(gd_key)
    }
    #[qjs(get, rename = "x")]
    fn get_x(&self) -> f64 {
        DisplayServer::singleton().mouse_get_position().x as f64
    }
    #[qjs(get, rename = "y")]
    fn get_y(&self) -> f64 {
        DisplayServer::singleton().mouse_get_position().y as f64
    }
}

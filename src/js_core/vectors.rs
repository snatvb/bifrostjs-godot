use crate::prelude::*;

use rquickjs::JsLifetime;
use rquickjs::class::{Trace, Tracer};

#[derive(JsLifetime, Copy, Clone, Debug)]
#[rquickjs::class]
pub struct JsVector2 {
    pub x: f32,
    pub y: f32,
}

impl<'js> Trace<'js> for JsVector2 {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
}

#[rquickjs::methods]
impl JsVector2 {
    #[qjs(constructor)]
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[qjs(get, rename = "x")]
    fn get_x(&self) -> f32 {
        self.x
    }

    #[qjs(set, rename = "x")]
    fn set_x(&mut self, val: f32) {
        self.x = val;
    }

    #[qjs(get, rename = "y")]
    fn get_y(&self) -> f32 {
        self.y
    }

    #[qjs(set, rename = "y")]
    fn set_y(&mut self, val: f32) {
        self.y = val;
    }

    #[qjs(rename = "set")]
    fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    #[qjs(rename = "add")]
    fn add(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }
}

use rquickjs::JsLifetime;
use rquickjs::class::{Trace, Tracer};

#[derive(JsLifetime, Copy, Clone, Debug)]
#[rquickjs::class]
pub struct JsVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl<'js> Trace<'js> for JsVector3 {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
}

#[rquickjs::methods]
impl JsVector3 {
    #[qjs(constructor)]
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
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

    #[qjs(get, rename = "z")]
    fn get_z(&self) -> f32 {
        self.z
    }
    #[qjs(set, rename = "z")]
    fn set_z(&mut self, val: f32) {
        self.z = val;
    }

    #[qjs(rename = "set")]
    fn set(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    #[qjs(rename = "add")]
    fn add(&mut self, x: f32, y: f32, z: f32) {
        self.x += x;
        self.y += y;
        self.z += z;
    }

    #[qjs(rename = "length")]
    fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

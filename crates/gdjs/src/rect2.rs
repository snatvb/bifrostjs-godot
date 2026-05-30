use rquickjs::JsLifetime;
use rquickjs::class::{Trace, Tracer};

#[derive(JsLifetime, Copy, Clone, Debug)]
#[rquickjs::class]
pub struct JsRect2 {
    pub position_x: f32,
    pub position_y: f32,
    pub size_x: f32,
    pub size_y: f32,
}

impl<'js> Trace<'js> for JsRect2 {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
}

#[rquickjs::methods]
impl JsRect2 {
    #[qjs(constructor)]
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position_x: x,
            position_y: y,
            size_x: width,
            size_y: height,
        }
    }

    #[qjs(get, rename = "x")]
    fn get_x(&self) -> f32 {
        self.position_x
    }
    #[qjs(set, rename = "x")]
    fn set_x(&mut self, val: f32) {
        self.position_x = val;
    }

    #[qjs(get, rename = "y")]
    fn get_y(&self) -> f32 {
        self.position_y
    }
    #[qjs(set, rename = "y")]
    fn set_y(&mut self, val: f32) {
        self.position_y = val;
    }

    #[qjs(get, rename = "width")]
    fn get_width(&self) -> f32 {
        self.size_x
    }
    #[qjs(set, rename = "width")]
    fn set_width(&mut self, val: f32) {
        self.size_x = val;
    }

    #[qjs(get, rename = "height")]
    fn get_height(&self) -> f32 {
        self.size_y
    }
    #[qjs(set, rename = "height")]
    fn set_height(&mut self, val: f32) {
        self.size_y = val;
    }

    #[qjs(rename = "set")]
    fn set(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.position_x = x;
        self.position_y = y;
        self.size_x = width;
        self.size_y = height;
    }
}

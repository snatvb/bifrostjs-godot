use rquickjs::class::{Trace, Tracer};
use rquickjs::JsLifetime;

#[derive(JsLifetime, Copy, Clone, Debug)]
#[rquickjs::class]
pub struct JsTransform2D {
    pub xx: f32,
    pub xy: f32,
    pub yx: f32,
    pub yy: f32,
    pub ox: f32,
    pub oy: f32,
}

impl<'js> Trace<'js> for JsTransform2D {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
}

#[rquickjs::methods]
impl JsTransform2D {
    #[qjs(constructor)]
    fn new(xx: f32, xy: f32, yx: f32, yy: f32, ox: f32, oy: f32) -> Self {
        Self { xx, xy, yx, yy, ox, oy }
    }

    #[qjs(get, rename = "xx")] fn get_xx(&self) -> f32 { self.xx }
    #[qjs(set, rename = "xx")] fn set_xx(&mut self, v: f32) { self.xx = v; }
    #[qjs(get, rename = "xy")] fn get_xy(&self) -> f32 { self.xy }
    #[qjs(set, rename = "xy")] fn set_xy(&mut self, v: f32) { self.xy = v; }
    #[qjs(get, rename = "yx")] fn get_yx(&self) -> f32 { self.yx }
    #[qjs(set, rename = "yx")] fn set_yx(&mut self, v: f32) { self.yx = v; }
    #[qjs(get, rename = "yy")] fn get_yy(&self) -> f32 { self.yy }
    #[qjs(set, rename = "yy")] fn set_yy(&mut self, v: f32) { self.yy = v; }
    #[qjs(get, rename = "ox")] fn get_ox(&self) -> f32 { self.ox }
    #[qjs(set, rename = "ox")] fn set_ox(&mut self, v: f32) { self.ox = v; }
    #[qjs(get, rename = "oy")] fn get_oy(&self) -> f32 { self.oy }
    #[qjs(set, rename = "oy")] fn set_oy(&mut self, v: f32) { self.oy = v; }

    #[qjs(rename = "set_origin")]
    fn set_origin(&mut self, x: f32, y: f32) {
        self.ox = x;
        self.oy = y;
    }
}

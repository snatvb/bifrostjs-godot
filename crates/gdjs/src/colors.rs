use rquickjs::class::{Trace, Tracer};
use rquickjs::JsLifetime;

#[derive(JsLifetime, Copy, Clone, Debug)]
#[rquickjs::class]
pub struct JsColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl<'js> Trace<'js> for JsColor {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
}

#[rquickjs::methods]
impl JsColor {
    #[qjs(constructor)]
    fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        }
    }

    #[qjs(get, rename = "r")]
    fn get_r(&self) -> f32 {
        self.r
    }
    #[qjs(set, rename = "r")]
    fn set_r(&mut self, val: f32) {
        self.r = val.clamp(0.0, 1.0);
    }

    #[qjs(get, rename = "g")]
    fn get_g(&self) -> f32 {
        self.g
    }
    #[qjs(set, rename = "g")]
    fn set_g(&mut self, val: f32) {
        self.g = val.clamp(0.0, 1.0);
    }

    #[qjs(get, rename = "b")]
    fn get_b(&self) -> f32 {
        self.b
    }
    #[qjs(set, rename = "b")]
    fn set_b(&mut self, val: f32) {
        self.b = val.clamp(0.0, 1.0);
    }

    #[qjs(get, rename = "a")]
    fn get_a(&self) -> f32 {
        self.a
    }
    #[qjs(set, rename = "a")]
    fn set_a(&mut self, val: f32) {
        self.a = val.clamp(0.0, 1.0);
    }

    #[qjs(rename = "set")]
    fn set(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.r = r.clamp(0.0, 1.0);
        self.g = g.clamp(0.0, 1.0);
        self.b = b.clamp(0.0, 1.0);
        self.a = a.clamp(0.0, 1.0);
    }

    #[qjs(rename = "toRgba32")]
    fn to_rgba32(&self) -> u32 {
        let ri = (self.r * 255.0).round().clamp(0.0, 255.0) as u32;
        let gi = (self.g * 255.0).round().clamp(0.0, 255.0) as u32;
        let bi = (self.b * 255.0).round().clamp(0.0, 255.0) as u32;
        let ai = (self.a * 255.0).round().clamp(0.0, 255.0) as u32;
        (ai << 24) | (bi << 16) | (gi << 8) | ri
    }

    #[qjs(rename = "toString")]
    fn to_string(&self) -> String {
        let ri = (self.r * 255.0).round().clamp(0.0, 255.0) as u8;
        let gi = (self.g * 255.0).round().clamp(0.0, 255.0) as u8;
        let bi = (self.b * 255.0).round().clamp(0.0, 255.0) as u8;
        let ai = (self.a * 255.0).round().clamp(0.0, 255.0) as u8;
        format!("#{:02x}{:02x}{:02x}{:02x}", ri, gi, bi, ai)
    }
}

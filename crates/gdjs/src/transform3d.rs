use rquickjs::JsLifetime;
use rquickjs::class::{Trace, Tracer};

#[derive(JsLifetime, Copy, Clone, Debug)]
#[rquickjs::class]
pub struct JsTransform3D {
    pub xx: f32,
    pub xy: f32,
    pub xz: f32,
    pub yx: f32,
    pub yy: f32,
    pub yz: f32,
    pub zx: f32,
    pub zy: f32,
    pub zz: f32,
    pub ox: f32,
    pub oy: f32,
    pub oz: f32,
}

impl<'js> Trace<'js> for JsTransform3D {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
}

#[rquickjs::methods]
impl JsTransform3D {
    #[qjs(constructor)]
    #[allow(clippy::too_many_arguments)]
    fn new(
        xx: f32, xy: f32, xz: f32,
        yx: f32, yy: f32, yz: f32,
        zx: f32, zy: f32, zz: f32,
        ox: f32, oy: f32, oz: f32,
    ) -> Self {
        Self { xx, xy, xz, yx, yy, yz, zx, zy, zz, ox, oy, oz }
    }

    #[qjs(get, rename = "xx")] fn get_xx(&self) -> f32 { self.xx }
    #[qjs(set, rename = "xx")] fn set_xx(&mut self, v: f32) { self.xx = v; }
    #[qjs(get, rename = "xy")] fn get_xy(&self) -> f32 { self.xy }
    #[qjs(set, rename = "xy")] fn set_xy(&mut self, v: f32) { self.xy = v; }
    #[qjs(get, rename = "xz")] fn get_xz(&self) -> f32 { self.xz }
    #[qjs(set, rename = "xz")] fn set_xz(&mut self, v: f32) { self.xz = v; }
    #[qjs(get, rename = "yx")] fn get_yx(&self) -> f32 { self.yx }
    #[qjs(set, rename = "yx")] fn set_yx(&mut self, v: f32) { self.yx = v; }
    #[qjs(get, rename = "yy")] fn get_yy(&self) -> f32 { self.yy }
    #[qjs(set, rename = "yy")] fn set_yy(&mut self, v: f32) { self.yy = v; }
    #[qjs(get, rename = "yz")] fn get_yz(&self) -> f32 { self.yz }
    #[qjs(set, rename = "yz")] fn set_yz(&mut self, v: f32) { self.yz = v; }
    #[qjs(get, rename = "zx")] fn get_zx(&self) -> f32 { self.zx }
    #[qjs(set, rename = "zx")] fn set_zx(&mut self, v: f32) { self.zx = v; }
    #[qjs(get, rename = "zy")] fn get_zy(&self) -> f32 { self.zy }
    #[qjs(set, rename = "zy")] fn set_zy(&mut self, v: f32) { self.zy = v; }
    #[qjs(get, rename = "zz")] fn get_zz(&self) -> f32 { self.zz }
    #[qjs(set, rename = "zz")] fn set_zz(&mut self, v: f32) { self.zz = v; }
    #[qjs(get, rename = "ox")] fn get_ox(&self) -> f32 { self.ox }
    #[qjs(set, rename = "ox")] fn set_ox(&mut self, v: f32) { self.ox = v; }
    #[qjs(get, rename = "oy")] fn get_oy(&self) -> f32 { self.oy }
    #[qjs(set, rename = "oy")] fn set_oy(&mut self, v: f32) { self.oy = v; }
    #[qjs(get, rename = "oz")] fn get_oz(&self) -> f32 { self.oz }
    #[qjs(set, rename = "oz")] fn set_oz(&mut self, v: f32) { self.oz = v; }

    #[qjs(rename = "set_origin")]
    fn set_origin(&mut self, x: f32, y: f32, z: f32) {
        self.ox = x;
        self.oy = y;
        self.oz = z;
    }
}

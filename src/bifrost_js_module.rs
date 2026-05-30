use js::module::Declarations;
use js::module::ModuleDef;
use rquickjs::module::Exports;

use crate::engine::create_engine;
use crate::prelude::*;

pub struct BifrostModule;

impl ModuleDef for BifrostModule {
    fn declare(decl: &Declarations) -> js::Result<()> {
        decl.declare("default")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &js::Ctx<'js>, exports: &Exports<'js>) -> js::Result<()> {
        let engine = create_engine(ctx)?;

        exports.export("default", engine)?;
        Ok(())
    }
}

mod js_core;
mod js_utility;
mod manager;
mod node;
mod prelude;

use crate::prelude::*;

struct RsJs;

#[gdextension]
unsafe impl ExtensionLibrary for RsJs {}

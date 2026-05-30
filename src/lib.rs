mod js_importer;
mod manager;
mod manager_context;
mod node;
mod prelude;
mod proxy_deps;

use crate::prelude::*;

struct RsJs;

#[gdextension]
unsafe impl ExtensionLibrary for RsJs {}

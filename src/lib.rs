mod manager;
mod manager_context;
mod node;
mod prelude;
mod proxy_deps;
mod signal_bridge;

use crate::prelude::*;

struct RsJs;

#[gdextension]
unsafe impl ExtensionLibrary for RsJs {}

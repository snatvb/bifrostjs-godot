use crate::prelude::*;

#[derive(Clone)]
pub struct ProxyDeps<'js> {
    pub ctx: Ctx<'js>,
    pub node: Gd<godot::prelude::Object>,
    pub manager_ctx: ManagerCtxRef,
}

impl<'js> ProxyDeps<'js> {
    pub fn with_node(&self, node: Gd<Node>) -> Self {
        ProxyDeps {
            node: node.upcast(),
            ..self.clone()
        }
    }

    pub fn try_node(&self) -> Option<Gd<Node>> {
        self.node.clone().try_cast::<Node>().ok()
    }
}

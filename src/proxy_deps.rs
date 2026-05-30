use crate::prelude::*;

#[derive(Clone)]
pub struct ProxyDeps<'js> {
    pub ctx: Ctx<'js>,
    pub node: Gd<Node>,
    pub manager_ctx: ManagerCtxRef,
}

impl<'js> ProxyDeps<'js> {
    pub fn with_node(&self, node: Gd<Node>) -> Self {
        ProxyDeps {
            node,
            ..self.clone()
        }
    }
}

use crate::prelude::*;

#[derive(GodotClass)]
#[class(base=RefCounted, no_init)]
pub struct JsSignalBridge {
    context: ManagerCtxWeak,
    callback_id: u64,
    base: Base<RefCounted>,
}

#[godot_api]
impl JsSignalBridge {
    pub fn create(callback_id: u64, context: ManagerCtxWeak) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            callback_id,
            base,
            context,
        })
    }

    #[func]
    fn on_signal_fired(&self) {
        println!("SIGNAAAL");
        if let Some(context) = self.context.upgrade() {
            context.borrow_mut().enqueue(self.callback_id);
        }
    }
}

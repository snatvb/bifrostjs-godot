use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::rc::{Rc, Weak};

use godot::classes::class_macros::private::virtuals::Os::Callable;
use godot::global::godot_error;
use godot::obj::InstanceId;
use rquickjs::{Ctx, Function, Persistent};

#[derive(Debug)]
pub struct JsSignalMeta {
    pub callback: Persistent<Function<'static>>,
    pub callable: Callable,
    pub id: u64,
    pub node_id: InstanceId,
    pub signal_name: String,
}

#[derive(Default, Debug)]
pub struct JsManagerContext {
    last_id: u64,
    signal_registry: HashMap<u64, JsSignalMeta>,
    signal_queue: VecDeque<u64>,
}

impl JsManagerContext {
    pub fn clear(&mut self) {
        self.last_id = 0;
        self.signal_registry.clear();
        self.signal_queue.clear();
    }

    pub fn next_callback_id(&mut self) -> u64 {
        self.last_id += 1;
        self.last_id
    }

    pub fn save_callback(&mut self, id: u64, meta: JsSignalMeta) {
        self.signal_registry.insert(id, meta);
    }

    pub fn drop_callback(&mut self, id: u64) -> Option<JsSignalMeta> {
        self.signal_registry.remove(&id)
    }

    pub fn remove_callback(&mut self, id: u64) {
        self.signal_registry.remove(&id);
    }

    pub fn enqueue(&mut self, id: u64) {
        self.signal_queue.push_back(id);
    }

    pub fn queue_is_empty(&self) -> bool {
        self.signal_queue.is_empty()
    }

    pub fn get_meta(&self, id: u64) -> Option<&JsSignalMeta> {
        self.signal_registry.get(&id)
    }

    pub(super) fn pop_signal(&mut self) -> Option<u64> {
        self.signal_queue.pop_front()
    }

    // pub fn process_queue(&mut self, ctx: &Ctx<'_>) {
    //     while let Some(id) = self.signal_queue.pop_front() {
    //         if let Some(persistent_func) = self.signal_registry.get(&id)
    //             && let Ok(func) = persistent_func.callback.clone().restore(ctx)
    //         {
    //             let res: rquickjs::Result<()> = func.call(());
    //             if let Err(e) = res {
    //                 godot_error!("Error inside JS Signal Callback (ID {}): {:?}", id, e);
    //             }
    //         }
    //     }
    // }
}

#[derive(Clone, Default, Debug)]
pub struct ManagerCtxRef(Rc<RefCell<JsManagerContext>>);

impl ManagerCtxRef {
    pub fn new(ctx: JsManagerContext) -> Self {
        Self(Rc::new(RefCell::new(ctx)))
    }

    pub fn downgrade(&self) -> ManagerCtxWeak {
        ManagerCtxWeak(Rc::downgrade(&self.0))
    }

    pub fn process_queue(&self, ctx: &Ctx<'_>) {
        loop {
            let mut this = self.0.borrow_mut();
            let f = this
                .pop_signal()
                .and_then(|id| this.get_meta(id))
                .and_then(|m| Some(m.id).zip(m.callback.clone().restore(ctx).ok()));
            drop(this);

            match f {
                Some((id, f)) => {
                    let res: rquickjs::Result<()> = f.call(());
                    if let Err(e) = res {
                        godot_error!("Error inside JS Signal Callback (ID {}): {:?}", id, e);
                    }
                }
                None => break,
            }
        }
        // while let Some(id) = self.0.borrow_mut().pop_signal() {
        //     if let Some(persistent_func) = self.0.borrow().get_meta(id)
        //         && let Ok(func) = persistent_func.callback.clone().restore(ctx)
        //     {
        //         let res: rquickjs::Result<()> = func.call(());
        //         if let Err(e) = res {
        //             godot_error!("Error inside JS Signal Callback (ID {}): {:?}", id, e);
        //         }
        //     }
        // }
    }
}

impl Deref for ManagerCtxRef {
    type Target = RefCell<JsManagerContext>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Default, Debug)]
pub struct ManagerCtxWeak(Weak<RefCell<JsManagerContext>>);

impl ManagerCtxWeak {
    pub fn upgrade(&self) -> Option<ManagerCtxRef> {
        self.0.upgrade().map(ManagerCtxRef)
    }
}

use crate::re::BSIntrusiveRefCounted::BSIntrusiveRefCounted;
use crate::re::IMessageBoxCallback::{IMessageBoxCallback, IMessageBoxCallbackVtbl, Message};

pub struct MessageBoxCallback {
    __base: IMessageBoxCallback,
    callback_fn: Box<dyn Fn(Message)>,
    // Has myself pinter to avoid double free.
    // - Use `Option`: 1 time drop in C++ Code
    self_ptr: Option<*mut Self>,
}

impl MessageBoxCallback {
    /// Create a new callback and return a raw pointer suitable for passing to C++
    pub fn new(callback_fn: Box<dyn Fn(Message)>) -> *mut Self {
        let __base =
            IMessageBoxCallback { vtable: &VTABLE, __base: BSIntrusiveRefCounted::new(), unk0C: 0 };

        // Save our own raw pointer to drop later
        let raw = Box::into_raw(Box::new(Self { __base, callback_fn, self_ptr: None }));
        unsafe {
            (*raw).self_ptr = Some(raw);
        }
        raw
    }
}

const VTABLE: IMessageBoxCallbackVtbl = IMessageBoxCallbackVtbl {
    CxxDrop: |this| {
        unsafe {
            let this = &mut *this.cast::<MessageBoxCallback>();
            if let Some(_ptr) = this.self_ptr.take() {
                // // Prevent double free
                // if !ptr.is_null() {
                //     drop(Box::from_raw(ptr));
                // }
            }
        }
    },
    Run: |this, msg| {
        let this = unsafe { &mut *this.cast::<MessageBoxCallback>() };
        (this.callback_fn)(msg);
    },
};

use core::ptr::NonNull;

use crate::re::BSIntrusiveRefCounted::BSIntrusiveRefCounted;
use crate::re::IMessageBoxCallback::{IMessageBoxCallback, IMessageBoxCallbackVtbl, Message};
use crate::re::offsets_rtti::RTTI___OldMessageBoxCallback;
use crate::re::offsets_vtable::VTABLE___OldMessageBoxCallback;
use crate::rel::id::VariantID;

pub struct OldMessageBoxCallback {
    __base: IMessageBoxCallback,
    callback: Option<NonNull<fn(Message)>>,
}
const _: () = assert!(core::mem::size_of::<OldMessageBoxCallback>() == 0x18);

impl OldMessageBoxCallback {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI___OldMessageBoxCallback;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE___OldMessageBoxCallback;

    /// Create a new callback and return a raw pointer suitable for passing to C++
    #[inline]
    pub const fn new(callback: fn(Message)) -> Self {
        let __base =
            IMessageBoxCallback { vtable: &VTABLE, __base: BSIntrusiveRefCounted::new(), unk0C: 0 };
        Self {
            __base,
            callback: NonNull::new((&callback) as *const fn(Message) as *mut fn(Message)),
        }
    }
}

static VTABLE: IMessageBoxCallbackVtbl = IMessageBoxCallbackVtbl {
    CxxDrop: |this| unsafe {
        let _this = &mut *this.cast::<OldMessageBoxCallback>();
    },
    Run: |this, msg| {
        let this = unsafe { &mut *this.cast::<OldMessageBoxCallback>() };
        if let Some(callback) = this.callback.as_ref() {
            (unsafe { callback.as_ref() })(msg);
        }
    },
};

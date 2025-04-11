use crate::re::BSIntrusiveRefCounted::BSIntrusiveRefCounted;
use crate::re::offsets_rtti::RTTI_IMessageBoxCallback;
use crate::re::offsets_vtable::VTABLE_IMessageBoxCallback;
use crate::rel::id::VariantID;

#[derive(Debug)]
#[repr(C)]
pub struct IMessageBoxCallback {
    pub vtable: *const IMessageBoxCallbackVtbl,
    pub __base: BSIntrusiveRefCounted, // 0x000
    pub unk0C: u32,
}
const _: () = assert!(core::mem::size_of::<IMessageBoxCallback>() == 0x10);

impl IMessageBoxCallback {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_IMessageBoxCallback;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_IMessageBoxCallback;
}

/// The virtual function table for `INIPrefSettingCollection`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
pub struct IMessageBoxCallbackVtbl {
    /// - C++ destructor: `~IMessageBoxCallback`
    pub CxxDrop: fn(this: *mut IMessageBoxCallback), // 0x0
    pub Run: fn(this: *mut IMessageBoxCallback, msg: Message), // 0x1
}

#[commonlibsse_ng_derive_internal::ffi_enum]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Message {
    Unk0 = 0,
    Unk1 = 1,
    Unk2 = 2,
}

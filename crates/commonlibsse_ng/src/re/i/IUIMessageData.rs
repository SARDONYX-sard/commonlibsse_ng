use crate::re::offsets_rtti::RTTI_IUIMessageData;
use crate::re::offsets_vtable::VTABLE_IUIMessageData;
use crate::rel::id::VariantID;
use core::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IUIMessageData {
    pub vtable: *const IUIMessageDataVtbl, // 0x00
    unk08: u16,                            // 0x08
    unk0A: u16,                            // 0x0A
    unk0C: u32,                            // 0x0C
}
const _: () = assert!(core::mem::size_of::<IUIMessageData>() == 0x10);

impl Default for IUIMessageData {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl IUIMessageData {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_IUIMessageData;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_IUIMessageData;

    /// Creates a new `IUIMessageData` with default values.
    #[inline]
    pub const fn new() -> Self {
        Self { vtable: &HK_BASE_OBJECT_VTBL, unk08: 0, unk0A: 0, unk0C: 0 }
    }
}

pub struct IUIMessageDataVtbl {
    /// Destructor for `IUIMessageData` (represented as a virtual method in C++).
    pub CxxDrop: unsafe extern "C" fn(this: *mut c_void),
}
impl IUIMessageDataVtbl {
    const fn new() -> Self {
        const unsafe extern "C" fn CxxDrop(_this: *mut c_void) {}

        Self { CxxDrop }
    }
}

static HK_BASE_OBJECT_VTBL: IUIMessageDataVtbl = IUIMessageDataVtbl::new();

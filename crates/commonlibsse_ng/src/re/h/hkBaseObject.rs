//! # hkBaseObject
//!
//! This module defines the `hkBaseObject` struct, which represents the base object in the Havok physics system.

use crate::re::offsets_rtti::RTTI_hkBaseObject;
use crate::re::offsets_vtable::VTABLE_hkBaseObject;
use crate::rel::id::VariantID;
use core::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct hkBaseObject {
    pub vtable: *const hkBaseObjectVtbl,
}
const _: () = assert!(core::mem::size_of::<hkBaseObject>() == 0x8);

impl Default for hkBaseObject {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl hkBaseObject {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_hkBaseObject;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_hkBaseObject;

    /// Creates a new `hkBaseObject` with default values.
    #[inline]
    pub const fn new() -> Self {
        Self { vtable: &HK_BASE_OBJECT_VTBL }
    }
}

pub struct hkBaseObjectVtbl {
    /// Destructor for `hkBaseObject` (represented as a virtual method in C++).
    CxxDrop: unsafe extern "C" fn(this: *mut c_void),
}
impl hkBaseObjectVtbl {
    const fn new() -> Self {
        unsafe extern "C" fn CxxDrop(_this: *mut c_void) {}

        Self { CxxDrop }
    }
}

static HK_BASE_OBJECT_VTBL: hkBaseObjectVtbl = hkBaseObjectVtbl::new();

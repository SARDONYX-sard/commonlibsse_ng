//! # bhkAction
//!
//! This module defines the `bhkAction` struct, which inherits from `bhkSerializable`
//! and represents a serializable object in the physics world. It includes virtual function pointers
//! for C++ compatibility and maintains the original memory layout.

use core::ffi::c_void;

use crate::re::bhkSerializable::{bhkSerializable, bhkSerializableVtbl};
use crate::re::offsets_ni_rtti::NiRTTI_bhkAction;
use crate::re::offsets_rtti::RTTI_bhkAction;
use crate::re::offsets_vtable::VTABLE_bhkAction;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct bhkAction {
    /// Base class.
    pub __base: bhkSerializable,
}

const _: () = assert!(core::mem::size_of::<bhkAction>() == 0x20);

impl bhkAction {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_bhkAction;
    /// Address & Offset of the runtime type information (Ni RTTI) identifier.
    pub const NI_RTTI: VariantID = NiRTTI_bhkAction;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_bhkAction;
}

/// The virtual function table for `bhkAction`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
pub struct bhkActionVtbl {
    pub __base: bhkSerializableVtbl,
    Unk_32: fn(this: *mut c_void, *mut c_void),
}

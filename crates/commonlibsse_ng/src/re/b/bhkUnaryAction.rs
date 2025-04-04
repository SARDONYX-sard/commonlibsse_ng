//! # bhkUnaryAction
//!
//! This module defines the `bhkUnaryAction` struct, which inherits from `bhkAction`
//! and represents a serializable object in the physics world. It includes virtual function pointers
//! for C++ compatibility and maintains the original memory layout.

use crate::re::bhkAction::{bhkAction, bhkActionVtbl};
use crate::re::offsets_ni_rtti::NiRTTI_bhkUnaryAction;
use crate::re::offsets_rtti::RTTI_bhkUnaryAction;
use crate::re::offsets_vtable::VTABLE_bhkUnaryAction;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct bhkUnaryAction {
    /// Base class.
    pub __base: bhkAction,
}
const _: () = assert!(core::mem::size_of::<bhkUnaryAction>() == 0x20);

impl bhkUnaryAction {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_bhkUnaryAction;
    /// Address & Offset of the runtime type information (Ni RTTI) identifier.
    pub const NI_RTTI: VariantID = NiRTTI_bhkUnaryAction;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_bhkUnaryAction;
}

/// The virtual function table for `bhkUnaryAction`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
pub struct bhkUnaryActionVtbl {
    pub __base: bhkActionVtbl,
}

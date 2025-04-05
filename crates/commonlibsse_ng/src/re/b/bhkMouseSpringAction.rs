//! # bhkUnaryAction
//!
//! This module defines the `bhkUnaryAction` struct, which inherits from `bhkAction`
//! and represents a serializable object in the physics world. It includes virtual function pointers
//! for C++ compatibility and maintains the original memory layout.

use crate::re::bhkUnaryAction::{bhkUnaryAction, bhkUnaryActionVtbl};
use crate::re::offsets_ni_rtti::NiRTTI_bhkMouseSpringAction;
use crate::re::offsets_rtti::RTTI_bhkMouseSpringAction;
use crate::re::offsets_vtable::VTABLE_bhkMouseSpringAction;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct bhkMouseSpringAction {
    /// Base class.
    pub __base: bhkUnaryAction,
}
const _: () = assert!(core::mem::size_of::<bhkMouseSpringAction>() == 0x20);

impl bhkMouseSpringAction {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_bhkMouseSpringAction;
    /// Address & Offset of the runtime type information (Ni RTTI) identifier.
    pub const NI_RTTI: VariantID = NiRTTI_bhkMouseSpringAction;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_bhkMouseSpringAction;
}

impl crate::re::hkRefPtr::hkRefPtrCounted for bhkMouseSpringAction {
    #[inline]
    fn AddReference(&self) {
        self.__base.__base.__base.__base.referenced_object.AddReference();
    }

    #[inline]
    fn RemoveReference(&self) {
        self.__base.__base.__base.__base.referenced_object.RemoveReference();
    }
}

/// The virtual function table for `bhkMouseSpringAction`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
pub struct bhkMouseSpringActionVtbl {
    pub __base: bhkUnaryActionVtbl,
}

//! # hkbProjectData
//!
//! This module defines the `hkbProjectData` struct, which inherits from `hkReferencedObject`
//! and represents project data in the Havok engine. It includes world-up vector, string data,
//! and default event node information.

use crate::re::hkRefPtr::hkRefPtr;
use crate::re::hkReferencedObject::hkReferencedObject;
use crate::re::hkVector4::hkVector4;
use crate::re::offsets_rtti::RTTI_hkbProjectData;
use crate::re::offsets_vtable::VTABLE_hkbProjectData;
use crate::rel::id::VariantID;
use std::mem;

use super::hkbProjectStringData::hkbProjectStringData;

/// Represents project data in the Havok engine.
#[repr(C)]
pub struct hkbProjectData {
    /// Base class `hkReferencedObject`.
    /// - Offset: `0x0`
    pub __base: hkReferencedObject,

    /// World-up vector in world space.
    /// - Offset: `0x10`
    pub worldUpWS: hkVector4,

    /// Pointer to string data.
    /// - Offset: `0x20`
    pub stringData: hkRefPtr<hkbProjectStringData>,

    /// Default event node (mapped as hkEnum).
    /// - Offset: `0x28`
    pub defaultEventNode: u8,

    /// Padding for memory alignment.
    /// - Offset: `0x29`
    pub _pad29: [u8; 7],
}

/// Ensure the memory layout matches the C++ version.
const _: () = {
    assert!(mem::size_of::<hkbProjectData>() == 0x30);
    assert!(mem::offset_of!(hkbProjectData, __base) == 0x0);
    assert!(mem::offset_of!(hkbProjectData, worldUpWS) == 0x10);
    assert!(mem::offset_of!(hkbProjectData, stringData) == 0x20);
    assert!(mem::offset_of!(hkbProjectData, defaultEventNode) == 0x28);
};
impl crate::re::hkRefPtr::hkRefPtrCounted for hkbProjectData {}

impl Default for hkbProjectData {
    fn default() -> Self {
        Self::new()
    }
}

impl hkbProjectData {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_hkbProjectData;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_hkbProjectData;

    /// Creates a new `hkbProjectData` instance with default values.
    ///
    /// - `worldUpWS`: Default `hkVector4`
    /// - `stringData`: Null `hkRefPtr`
    /// - `defaultEventNode`: Zero
    #[inline]
    pub fn new() -> Self {
        Self {
            __base: hkReferencedObject::new(),
            worldUpWS: hkVector4::new(),
            stringData: hkRefPtr::new(),
            defaultEventNode: 0,
            _pad29: [0; 7],
        }
    }
}

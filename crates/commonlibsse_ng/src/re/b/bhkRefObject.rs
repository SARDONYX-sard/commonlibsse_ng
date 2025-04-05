//! # bhkRefObject
//!
//! This module defines the `bhkRefObject` struct, which inherits from `NiObject` and represents
//! a reference object in the Havok physics system. It includes methods to manage reference counts
//! and holds a reference to another `hkReferencedObject`.

use crate::re::NiObject::{NiObject, NiObjectVtbl};
use crate::re::bhkSerializable::bhkSerializable;
use crate::re::hkRefPtr::hkRefPtr;
use crate::re::hkReferencedObject::hkReferencedObject;
use crate::re::offsets_ni_rtti::NiRTTI_bhkRefObject;
use crate::re::offsets_rtti::RTTI_bhkRefObject;
use crate::re::offsets_vtable::VTABLE_bhkRefObject;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct bhkRefObject {
    /// Base class `NiObject`.
    pub __base: NiObject,

    /// Reference to the referenced object.
    /// - Offset: `0x10`
    pub referenced_object: hkRefPtr<hkReferencedObject>,
}

const _: () = {
    assert!(core::mem::offset_of!(bhkRefObject, __base) == 0x0);
    assert!(core::mem::offset_of!(bhkRefObject, referenced_object) == 0x10);
    assert!(core::mem::size_of::<bhkRefObject>() == 0x18);
};

impl bhkRefObject {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_bhkRefObject;
    /// Address & Offset of the runtime type information (Ni RTTI) identifier.
    pub const NI_RTTI: VariantID = NiRTTI_bhkRefObject;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_bhkRefObject;
}

/// The virtual function table for `bhkRefObject`.
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
pub struct bhkRefObjectVtbl {
    pub __base: NiObjectVtbl,

    /// Function pointer for setting the referenced object.
    pub SetReferencedObject: fn(this: &mut bhkSerializable, a_object: *mut hkReferencedObject),
    /// Function pointer for adjusting the reference count.
    pub AdjustRefCount: fn(this: &mut bhkRefObject, a_increment: bool),
}

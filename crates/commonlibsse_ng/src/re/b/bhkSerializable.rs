//! # bhkSerializable
//!
//! This module defines the `bhkSerializable` struct, which inherits from `bhkRefObject`
//! and represents a serializable object in the physics world. It includes virtual function pointers
//! for C++ compatibility and maintains the original memory layout.

use core::ffi::c_void;

use crate::re::bhkRefObject::bhkRefObject;
use crate::re::bhkWorld::bhkWorld;
use crate::re::offsets_ni_rtti::NiRTTI_bhkSerializable;
use crate::re::offsets_rtti::RTTI_bhkSerializable;
use crate::re::offsets_vtable::VTABLE_bhkSerializable;
use crate::re::{ahkpWorld, hkpWorld};
use crate::rel::id::VariantID;

use super::bhkRefObject::bhkRefObjectVtbl;

/// Represents a serializable object in the physics world.
/// Inherits from `bhkRefObject`.
///
/// # Memory Layout:
/// - `__base`: Base class `bhkRefObject`
/// - `serializable`: Pointer to another `bhkSerializable`
///
#[repr(C)]
pub struct bhkSerializable {
    /// Base class `bhkRefObject`.
    pub __base: bhkRefObject,

    /// Pointer to another `bhkSerializable`.
    pub serializable: *mut bhkSerializable,
}

const _: () = {
    assert!(core::mem::offset_of!(bhkSerializable, __base) == 0x0);
    assert!(core::mem::offset_of!(bhkSerializable, serializable) == 0x18);
    assert!(core::mem::size_of::<bhkSerializable>() == 0x20);
};

impl bhkSerializable {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_bhkSerializable;
    /// Address & Offset of the runtime type information (Ni RTTI) identifier.
    pub const NI_RTTI: VariantID = NiRTTI_bhkSerializable;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_bhkSerializable;
}

/// The virtual function table for `bhkSerializable`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
pub struct bhkSerializableVtbl {
    pub __base: bhkRefObjectVtbl,

    /// - bhkSerializable: return 0
    GetWorld1: fn(this: *mut c_void) -> *mut hkpWorld,
    GetWorld2: fn(this: *mut c_void) -> *mut ahkpWorld,
    MoveToWorld: fn(*mut c_void, world: *mut bhkWorld),
    RemoveFromCurrentWorld: unsafe fn(this: *mut c_void),
    Unk_2B: fn(this: *mut c_void, *mut c_void),
    Unk_2C: fn(this: *mut c_void, *mut c_void), // return 1
    Unk_2D: fn(this: *mut c_void, *mut c_void),
    Unk_2E: fn(this: *mut c_void, *mut c_void), // pure virtual
    Unk_2F: fn(this: *mut c_void, *mut c_void), // pure virtual
    Unk_30: fn(this: *mut c_void, *mut c_void),
    Unk_31: fn(this: *mut c_void, *mut c_void),
}

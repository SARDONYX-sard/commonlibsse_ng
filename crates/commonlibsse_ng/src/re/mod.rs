#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod b;
mod c;
mod e;
mod f;
mod i;
mod m;
mod n;
mod t;

pub mod rtti;

#[doc(hidden)]
#[rustfmt::skip]
pub mod offsets_rtti;
#[doc(hidden)]
#[rustfmt::skip]
pub mod offsets_ni_rtti;
#[doc(hidden)]
#[rustfmt::skip]
pub mod offsets_vtable;

pub use self::b::*;
pub use self::c::*;
pub use self::e::*;
pub use self::f::*;
pub use self::i::*;
pub use self::m::*;
pub use self::n::*;
pub use self::t::*;

use crate::rel::id::VariantID;

/// C++ Virtual Class RTTI & Vtable accessor
pub trait CxxVirtClass {
    /// Gets the runtime information address ID reference.
    fn rtti() -> &'static VariantID;
    /// Gets the virtual function table address reference.
    fn vtable() -> &'static [VariantID];
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// dummies

pub struct GFxMovieView;
pub struct GFxValue;

pub struct VMHandle(pub u64);
pub struct FormID(pub u32);

pub struct TesWaterForm;

#[derive(Debug)]
pub struct BSAnimationGraphEvent;
#[derive(Debug)]
pub struct IAnimationGraphManagerHolder {
    pub opaque: [u8; 2],
}

pub enum ItemRemoveReason {
    Remove,
    Steal,
    Selling,
    Dropping,
    StoreInContainer,
    StoreInTeammate,
}
pub struct ObjectHandle;
pub struct NiExtraData;
pub struct NiTimeController;

impl NiSmartPointer::RefCountable for NiTimeController {
    fn inc_ref_count(&self) {
        todo!()
    }

    fn dec_ref_count(&mut self) {
        todo!()
    }
}

#[repr(C)]
pub struct NiNode;

#[repr(C)]
pub struct NiSwitchNode;

#[repr(C)]
pub struct BSFadeNode;

#[repr(C)]
pub struct BSMultiBoundNode;

#[repr(C)]
pub struct BSGeometry;

#[repr(C)]
pub struct NiTriStrips;

#[repr(C)]
pub struct BSTriShape;

#[repr(C)]
pub struct BSSegmentedTriShape;

#[repr(C)]
pub struct BSSubIndexTriShape;

#[repr(C)]
pub struct BSDynamicTriShape;

#[repr(C)]
pub struct NiGeometry;

#[repr(C)]
pub struct NiTriBasedGeom;

#[repr(C)]
pub struct NiTriShape;

#[repr(C)]
pub struct NiParticles;

#[repr(C)]
pub struct BSLines;

#[repr(C)]
pub struct bhkNiCollisionObject;

#[repr(C)]
pub struct bhkBlendCollisionObject;

#[repr(C)]
pub struct bhkAttachmentCollisionObject;

#[repr(C)]
pub struct bhkRigidBody;

#[repr(C)]
pub struct bhkLimitedHingeConstraint;

#[repr(C)]
pub struct NiCloningProcess;
pub struct NiStream;
pub struct NiObjectGroup;
pub struct NiControllerManager;
pub struct bhkCollisionObject;

pub struct BGSLocation;
pub struct TESFaction;

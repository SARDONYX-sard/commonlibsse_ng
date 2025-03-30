#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod a;
mod b;
mod c;
mod e;
mod f;
mod g;
mod i;
mod m;
mod n;
mod p;
mod s;
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

pub use self::a::*;
pub use self::b::*;
pub use self::c::*;
pub use self::e::*;
pub use self::f::*;
pub use self::g::*;
pub use self::i::*;
pub use self::m::*;
pub use self::n::*;
pub use self::p::*;
pub use self::s::*;
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
impl NiSmartPointer::RefCountable for NiNode {
    fn inc_ref_count(&self) {
        todo!()
    }
    fn dec_ref_count(&mut self) {
        todo!()
    }
}

#[repr(C)]
pub struct NiSwitchNode;

#[repr(C)]
pub struct BSFadeNode;
impl NiSmartPointer::RefCountable for BSFadeNode {
    fn inc_ref_count(&self) {
        todo!()
    }
    fn dec_ref_count(&mut self) {
        todo!()
    }
}

#[repr(C)]
pub struct BSMultiBoundNode;

#[repr(C)]
pub struct BSGeometry;

#[repr(C)]
pub struct NiTriStrips;

#[repr(C)]
pub struct BSTriShape;
impl NiSmartPointer::RefCountable for BSTriShape {
    fn inc_ref_count(&self) {
        todo!()
    }
    fn dec_ref_count(&mut self) {
        todo!()
    }
}

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
pub struct NiBillboardNode;
impl NiSmartPointer::RefCountable for NiBillboardNode {
    fn inc_ref_count(&self) {
        todo!()
    }
    fn dec_ref_count(&mut self) {
        todo!()
    }
}

#[repr(C)]
pub struct TESTopicInfo;
#[repr(C)]
pub struct BGSAnimationSequencer;
#[repr(C)]
pub struct BGSKeyword;
#[repr(C)]
pub struct TESPackage;
#[repr(C)]
pub struct BGSScene;
#[repr(C)]
pub struct DialogueResponse;
#[repr(C)]
pub struct BGSDialogueBranch;
#[repr(C)]
pub struct ActorCause;
#[repr(C)]
pub struct MagicCaster;
#[repr(C)]
pub struct MagicTarget;
#[repr(C)]
pub struct TESActorBase;
#[repr(C)]
pub struct BSFaceGenNiNode;
#[repr(C)]
pub struct BSFaceGenAnimationData;
#[repr(C)]
pub struct TrapData;
#[repr(C)]
pub struct TrapEntry;
#[repr(C)]
pub struct TargetEntry;
#[repr(C)]
pub struct BSAnimationUpdateData;
#[repr(C)]
pub struct BipedAnim;
#[repr(C)]
pub struct BSTSmartPointer<T>(pub *mut T);

#[repr(C)]
pub enum MagicSystem {
    CastingSource,
}

pub enum ITEM_REMOVE_REASON {}

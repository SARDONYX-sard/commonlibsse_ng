use crate::re::NiRTTI::NiRTTI;
use crate::re::NiRefObject::NiRefObject;
use crate::re::offsets_ni_rtti::NiRTTI_NiObject;
use crate::re::offsets_rtti::RTTI_NiObject;
use crate::re::offsets_vtable::VTABLE_NiObject;
use crate::re::{
    BSDynamicTriShape, BSFadeNode, BSGeometry, BSLines, BSMultiBoundNode, BSSegmentedTriShape,
    BSSubIndexTriShape, BSTriShape, NiCloningProcess, NiControllerManager, NiGeometry, NiNode,
    NiObjectGroup, NiParticles, NiStream, NiSwitchNode, NiTriBasedGeom, NiTriShape, NiTriStrips,
    bhkAttachmentCollisionObject, bhkBlendCollisionObject, bhkLimitedHingeConstraint,
    bhkNiCollisionObject, bhkRigidBody,
};
use crate::rel::id::VariantID;

use super::NiRefObject::NiRefObjectVtbl;

#[repr(C)]
pub struct NiObject {
    pub __base: NiRefObject,
    // pub vtable: *const NiObjectVtbl, // default virtual doesn't have vtable.
}
const _: () = assert!(core::mem::size_of::<NiObject>() == 0x10);

impl NiObject {
    pub const RTTI: VariantID = RTTI_NiObject;
    pub const NI_RTTI: VariantID = NiRTTI_NiObject;
    pub const VTABLE: [VariantID; 1] = VTABLE_NiObject;
}

impl crate::re::NiSmartPointer::RefCountable for NiObject {
    #[inline]
    fn inc_ref_count(&self) {
        self.__base.inc_ref_count();
    }

    #[inline]
    fn dec_ref_count(&self) {
        self.__base.dec_ref_count();
    }
}

// Dummy structs for missing classes

/// # Virtual member functions info
/// - fn count: 37
/// - offset: 37 * 8 = 288(0x128)
#[repr(C)]
pub struct NiObjectVtbl {
    //                                                                                                  | Method count |
    /// - overrides
    ///  - destructor
    pub _base: NiRefObjectVtbl,

    // additional methods
    pub GetRtti: unsafe extern "C" fn(this: *const NiObject) -> *const NiRTTI, // 0x02
    pub AsNode: unsafe extern "C" fn(this: *mut NiObject) -> *mut NiNode,      // 0x03
    pub AsSwitchNode: unsafe extern "C" fn(this: *mut NiObject) -> *mut NiSwitchNode, // 0x04
    pub AsFadeNode: unsafe extern "C" fn(this: *mut NiObject) -> *mut BSFadeNode, // 0x05
    pub AsMultiBoundNode: unsafe extern "C" fn(this: *mut NiObject) -> *mut BSMultiBoundNode, // 0x06
    pub AsGeometry: unsafe extern "C" fn(this: *mut NiObject) -> *mut BSGeometry, // 0x07
    pub AsTriStrips: unsafe extern "C" fn(this: *mut NiObject) -> *mut NiTriStrips, // 0x08
    pub AsTriShape: unsafe extern "C" fn(this: *mut NiObject) -> *mut BSTriShape, // 0x09
    pub AsSegmentedTriShape: unsafe extern "C" fn(this: *mut NiObject) -> *mut BSSegmentedTriShape, // 0x0A
    pub AsSubIndexTriShape: unsafe extern "C" fn(this: *mut NiObject) -> *mut BSSubIndexTriShape, // 0x0B
    pub AsDynamicTriShape: unsafe extern "C" fn(this: *mut NiObject) -> *mut BSDynamicTriShape, // 0x0C
    pub AsNiGeometry: unsafe extern "C" fn(this: *mut NiObject) -> *mut NiGeometry, // 0x0D
    pub AsNiTriBasedGeom: unsafe extern "C" fn(this: *mut NiObject) -> *mut NiTriBasedGeom, // 0x0E
    pub AsNiTriShape: unsafe extern "C" fn(this: *mut NiObject) -> *mut NiTriShape, // 0x0F
    pub AsParticlesGeom: unsafe extern "C" fn(this: *mut NiObject) -> *mut NiParticles, // 0x10
    pub AsLinesGeom: unsafe extern "C" fn(this: *mut NiObject) -> *mut BSLines,     // 0x11
    pub AsBhkNiCollisionObject:
        unsafe extern "C" fn(this: *mut NiObject) -> *mut bhkNiCollisionObject, // 0x12
    pub AsBhkBlendCollisionObject:
        unsafe extern "C" fn(this: *mut NiObject) -> *mut bhkBlendCollisionObject, // 0x13
    pub AsBhkAttachmentCollisionObject:
        unsafe extern "C" fn(this: *mut NiObject) -> *mut bhkAttachmentCollisionObject, // 0x14
    pub AsBhkRigidBody: unsafe extern "C" fn(this: *mut NiObject) -> *mut bhkRigidBody, // 0x15
    pub AsBhkLimitedHingeConstraint:
        unsafe extern "C" fn(this: *mut NiObject) -> *mut bhkLimitedHingeConstraint, // 0x16
    pub CreateClone:
        unsafe extern "C" fn(this: *mut NiObject, cloning: *mut NiCloningProcess) -> *mut NiObject, // 0x17
    pub LoadBinary: unsafe extern "C" fn(this: *mut NiObject, stream: *mut NiStream), // 0x18
    pub LinkObject: unsafe extern "C" fn(this: *mut NiObject, stream: *mut NiStream), // 0x19
    pub RegisterStreamables:
        unsafe extern "C" fn(this: *mut NiObject, stream: *mut NiStream) -> bool, // 0x1A
    pub SaveBinary: unsafe extern "C" fn(this: *mut NiObject, stream: *mut NiStream), // 0x1B
    pub IsEqual: unsafe extern "C" fn(this: *mut NiObject, other: *mut NiObject) -> bool, // 0x1C
    pub ProcessClone: unsafe extern "C" fn(this: *mut NiObject, cloning: *mut NiCloningProcess), // 0x1D
    pub PostLinkObject: unsafe extern "C" fn(this: *mut NiObject, stream: *mut NiStream), // 0x1E
    pub StreamCanSkip: unsafe extern "C" fn(this: *mut NiObject) -> bool,                 // 0x1F
    pub GetStreamableRtti: unsafe extern "C" fn(this: *const NiObject) -> *const NiRTTI,  // 0x20
    pub GetBlockAllocationSize: unsafe extern "C" fn(this: *const NiObject) -> u32,       // 0x21
    pub GetGroup: unsafe extern "C" fn(this: *const NiObject) -> *mut NiObjectGroup,      // 0x22
    pub SetGroup: unsafe extern "C" fn(this: *mut NiObject, group: *mut NiObjectGroup),   // 0x23

    pub AsNiControllerManager:
        unsafe extern "C" fn(this: *mut NiObject) -> *mut NiControllerManager, // 0x24
}
const _: () = {
    const FN_COUNT: usize = 37 * core::mem::size_of::<usize>();
    assert!(core::mem::size_of::<NiObjectVtbl>() == FN_COUNT);
};

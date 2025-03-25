use crate::re::NiRTTI::NiRTTI;
use crate::re::NiRefObject::NiRefObject;
use crate::re::NiRefObject::NiRefObjectVtbl;
use crate::re::NiSmartPointer::NiPointer;
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
use crate::rel::ResolvableAddress as _;
use crate::rel::id::{DataBaseError, VariantID};
use crate::rel::relocation::Relocation;
use core::ffi::CStr;
use core::ptr::NonNull;

#[repr(C)]
pub struct NiObject {
    pub __base: NiRefObject,
}
const _: () = assert!(core::mem::size_of::<NiObject>() == 0x10);

impl NiObject {
    pub const RTTI: VariantID = RTTI_NiObject;
    pub const NI_RTTI: VariantID = NiRTTI_NiObject;
    pub const VTABLE: [VariantID; 1] = VTABLE_NiObject;

    /// # Errors
    pub fn get_rtti(&self) -> Result<NonNull<NiRTTI>, DataBaseError> {
        let rel = Relocation::new(Self::NI_RTTI.address()?);
        Ok(rel.cast::<NiRTTI>())
    }

    /// # Safety
    pub unsafe fn is_equal(&self, other: *mut Self) -> bool {
        if other.is_null() {};

        let name = match self.get_rtti() {
            Ok(rtti) => unsafe { rtti.as_ref().get_name() },
            Err(_) => return false,
        };
        let rtti = match unsafe { other.as_ref() } {
            Some(rtti) => rtti,
            None => return false,
        };

        let rtti = match rtti.get_rtti() {
            Ok(rtti) => rtti,
            Err(_) => return false,
        };
        let other_name = unsafe { rtti.as_ref().get_name() };
        let self_name = unsafe { CStr::from_ptr(name) };
        let other_name = unsafe { CStr::from_ptr(other_name) };
        self_name == other_name
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 68838, ae_id = 70190)]
    pub unsafe fn process_clone(&self, cloning: &NiCloningProcess) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 68839, ae_id = 70191)]
    pub unsafe fn create_deep_copy(&self, cloning: &NiPointer<NiObject>) {}
}

impl crate::re::NiSmartPointer::RefCountable for NiObject {
    #[inline]
    fn inc_ref_count(&self) {
        self.__base.inc_ref_count();
    }

    #[inline]
    fn dec_ref_count(&mut self) {
        self.__base.dec_ref_count();
    }
}

/// # Virtual member functions info
/// - fn count: 37
/// - offset: 37 * 8 = 288(0x128)
#[repr(C)]
pub struct NiObjectVtbl {
    //                                                                                                  | Method count |
    /// - overrides
    ///  - destructor
    pub __base: NiRefObjectVtbl,

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

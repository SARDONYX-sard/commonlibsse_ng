use crate::re::NiAVObject::NiAVObject;
use crate::re::NiObject::{NiObject, NiObjectVtbl};
use crate::re::offsets_ni_rtti::NiRTTI_NiCollisionObject;
use crate::re::offsets_rtti::RTTI_NiCollisionObject;
use crate::re::offsets_vtable::VTABLE_NiCollisionObject;
use crate::rel::id::VariantID;

#[repr(C)]
pub struct NiCollisionObject {
    pub __base: NiObject,
    pub sceneObject: *mut NiAVObject,
}
const _: () = assert!(core::mem::size_of::<NiCollisionObject>() == 0x18);

impl NiCollisionObject {
    pub const RTTI: VariantID = RTTI_NiCollisionObject;
    pub const NI_RTTI: VariantID = NiRTTI_NiCollisionObject;
    pub const VTABLE: [VariantID; 1] = VTABLE_NiCollisionObject;
}

impl crate::re::NiSmartPointer::RefCountable for NiCollisionObject {
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
#[repr(C)]
pub struct NiCollisionObjectVtbl {
    pub __base: NiObjectVtbl,

    // additional methods
    pub Unk_25: unsafe extern "C" fn(this: *mut NiCollisionObject), // 0x25
    pub Unk_26: unsafe extern "C" fn(this: *mut NiCollisionObject), // 0x26
    pub Unk_27: unsafe extern "C" fn(this: *mut NiCollisionObject), // 0x27
    pub Unk_28: unsafe extern "C" fn(this: *mut NiCollisionObject), // 0x28
    pub Unk_29: unsafe extern "C" fn(this: *mut NiCollisionObject), // 0x29
}
const _: () = {
    const FN_COUNT: usize = (1 + 0x29) * core::mem::size_of::<usize>();
    assert!(core::mem::size_of::<NiCollisionObjectVtbl>() == FN_COUNT);
};

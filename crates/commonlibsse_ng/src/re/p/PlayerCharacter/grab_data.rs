use crate::re::{
    BSPointerHandle::ObjectRefHandle, BSTArray::BSTSmallArray,
    bhkMouseSpringAction::bhkMouseSpringAction, hkRefPtr::hkRefPtr,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GrabbingType {
    #[default]
    None = 0,
    Normal = 1,
    Telekinesis = 2,
}

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GrabData {
    grabSpring: BSTSmallArray<hkRefPtr<bhkMouseSpringAction>, 4>,
    grabbedObject: ObjectRefHandle,
    grabObjectWeight: f32,
    grabDistance: f32,
    unk004: f32,
    unk008: u64,
}
const _: () = {
    assert!(core::mem::offset_of!(GrabData, grabSpring) == 0x0);
    assert!(core::mem::offset_of!(GrabData, grabbedObject) == 0x30);
    assert!(core::mem::offset_of!(GrabData, grabObjectWeight) == 0x34);
    assert!(core::mem::offset_of!(GrabData, grabDistance) == 0x38);
    assert!(core::mem::offset_of!(GrabData, unk004) == 0x3c);
    assert!(core::mem::offset_of!(GrabData, unk008) == 0x40);

    assert!(core::mem::size_of::<GrabData>() == 0x48);
};

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct VRGrabData {
    /// - `0x8(hkRefPtr) * 4(N) = 32 = 0x20`
    grabSpring: BSTSmallArray<hkRefPtr<bhkMouseSpringAction>, 0x20>,
    grabbedObject: ObjectRefHandle,
    grabObjectWeight: f32,
    grabType: GrabbingType,
    grabDistance: f32,
    unk40: f64,
    unk48: u64,
    unk50: f64,
    unk58: u64,
    unk60: u32,
    unk64Flags: u32,
}

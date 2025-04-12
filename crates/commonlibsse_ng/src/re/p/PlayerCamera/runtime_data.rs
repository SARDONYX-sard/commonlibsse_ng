use crate::re::BSAtomic::BSSpinLock;
use crate::re::BSCoreTypes::RefHandle;
use crate::re::BSTArray::BSTSmallArray;
use crate::re::BSTSmartPointer::BSTSmartPointer;
use crate::re::CrosshairPickData::bhkSimpleShapePhantom;
use crate::re::NiSmartPointer::NiPointer;
use crate::re::TESCameraState::TESCameraState;
use crate::re::bhkRigidBody;

use super::{CameraStateSE_CEnum, CameraStateVR_CEnum};

#[repr(C)]
#[derive(Debug)]
pub struct Unk120 {
    pub unk00: NiPointer<bhkSimpleShapePhantom>,
    pub unk08: NiPointer<bhkSimpleShapePhantom>,
}
const _: () = assert!(core::mem::size_of::<Unk120>() == 0x10);

const TEMP_RETURN_STATES_TOTAL: usize =
    core::mem::size_of::<*mut TESCameraState>() * CameraStateSE_CEnum::count();
const CAMERA_STATES_SIZE: usize = CameraStateSE_CEnum::count();

#[repr(C)]
#[derive(Debug)]
pub struct RUNTIME_DATA {
    pub tempReturnStates: BSTSmallArray<*mut TESCameraState, TEMP_RETURN_STATES_TOTAL>, // 0x040, VR 0x040
    pub cameraStates: [BSTSmartPointer<TESCameraState>; CAMERA_STATES_SIZE], // 0x0B8, VR 0x0C0
    pub unk120: *mut Unk120,                                                 // 0x120,
    pub rigidBody: NiPointer<bhkRigidBody>,                                  // 0x128, VR 0x130
    pub objectFadeHandle: RefHandle,                                         // 0x130, VR 0x138
    pub lock: BSSpinLock,                                                    // 0x134, VR 0x13c
}
const _: () = {
    assert!(core::mem::offset_of!(RUNTIME_DATA, cameraStates) == 0x78);
    assert!(core::mem::offset_of!(RUNTIME_DATA, rigidBody) == 0xE8);
    assert!(core::mem::offset_of!(RUNTIME_DATA, objectFadeHandle) == 0xF0);
    assert!(core::mem::offset_of!(RUNTIME_DATA, lock) == 0xF4);

    assert!(core::mem::size_of::<RUNTIME_DATA>() == 0x100);
};

const VR_TEMP_RETURN_STATES_TOTAL: usize =
    core::mem::size_of::<*mut TESCameraState>() * CameraStateVR_CEnum::count();
const VR_CAMERA_STATES_SIZE: usize = CameraStateVR_CEnum::count();

#[repr(C)]
#[derive(Debug)]
pub struct VR_RUNTIME_DATA {
    pub tempReturnStates: BSTSmallArray<*mut TESCameraState, VR_TEMP_RETURN_STATES_TOTAL>, // 0x040, VR 0x040
    pub cameraStates: [BSTSmartPointer<TESCameraState>; VR_CAMERA_STATES_SIZE], // 0x0B8, VR 0x0C0
    pub rigidBody: NiPointer<bhkRigidBody>,                                     // 0x128, VR 0x130
    pub objectFadeHandle: RefHandle,                                            // 0x130, VR 0x138
    pub lock: BSSpinLock,                                                       // 0x134, VR 0x13c
    pub VR_pad144: [u8; 14],
}
const _: () = {
    assert!(core::mem::offset_of!(VR_RUNTIME_DATA, cameraStates) == 0x80);
    assert!(core::mem::offset_of!(VR_RUNTIME_DATA, rigidBody) == 0xF0);
    assert!(core::mem::offset_of!(VR_RUNTIME_DATA, objectFadeHandle) == 0xF8);
    assert!(core::mem::offset_of!(VR_RUNTIME_DATA, lock) == 0xFC);

    assert!(core::mem::size_of::<VR_RUNTIME_DATA>() == 0x118);
};

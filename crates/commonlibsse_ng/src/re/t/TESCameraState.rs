use crate::re::BGSLoadFormBuffer::BGSLoadFormBuffer;
use crate::re::BGSSaveFormBuffer::BGSSaveFormBuffer;
use crate::re::BSIntrusiveRefCounted::BSIntrusiveRefCounted;
use crate::re::BSTSmartPointer::BSTSmartPointer;
use crate::re::NiPoint3::NiPoint3;
use crate::re::NiQuaternion::NiQuaternion;
use crate::re::PlayerCamera::CameraState;
use crate::re::TESCamera::TESCamera;
use crate::re::offsets_rtti::RTTI_TESCameraState;
use crate::re::offsets_vtable::VTABLE_TESCameraState;
use crate::rel::id::VariantID;
use core::ffi::c_void;

#[repr(C)]
#[derive(Debug)]
pub struct TESCameraState {
    pub vtable: *const TESCameraStateVtbl, // 0x00
    pub __base: BSIntrusiveRefCounted,     // 0x08
    pub pad0C: u8,                         // 0x0C
    pub camera: *mut TESCamera,            // 0x10
    pub id: CameraState,                   // 0x18
    pub pad34: u32,                        // 0x1C
}
const _: () = assert!(std::mem::size_of::<TESCameraState>() == 0x20);

impl TESCameraState {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_TESCameraState;

    /// Address & offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_TESCameraState;
}

impl crate::re::BSIntrusiveRefCounted::BSIntrusiveRefCountedTrait for TESCameraState {
    #[inline]
    fn inc_ref(&self) -> u32 {
        self.__base.inc_ref()
    }

    #[inline]
    fn dec_ref(&self) -> u32 {
        self.__base.dec_ref()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TESCameraStateVtbl {
    /// C++ Destructor `~TESCamera`
    pub CxxDrop: fn(this: *mut c_void), // 0x00
    pub Begin: fn(this: *mut c_void), // 0x01
    pub End: fn(this: *mut c_void),   // 0x02

    pub Unk_03: fn(this: *mut c_void), // 0x03 - VR only

    pub Update: fn(this: *mut c_void, nextState: &BSTSmartPointer<TESCameraState>), // 0x03
    pub GetRotation: fn(this: *mut c_void, rotation: &NiQuaternion),                // 0x04
    pub GetTranslation: fn(this: *mut c_void, translation: &NiPoint3),              // 0x05
    pub SaveGame: fn(this: *mut c_void, buf: &BGSSaveFormBuffer),                   // 0x06
    pub LoadGame: fn(this: *mut c_void, buf: *mut BGSLoadFormBuffer),               // 0x07
    pub Revert: fn(this: *mut c_void, buf: *mut BGSLoadFormBuffer),                 // 0x08
}
const _: () = {
    const VFUNC_COUNT: usize = 0x9 + 1;

    const EXPECTED_SIZE: usize = VFUNC_COUNT * core::mem::size_of::<usize>();
    assert!(core::mem::size_of::<TESCameraStateVtbl>() == EXPECTED_SIZE);
};

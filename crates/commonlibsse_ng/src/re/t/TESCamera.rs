use crate::re::BSTPoint::{BSTPoint2, BSTPoint3};
use crate::re::BSTSmartPointer::BSTSmartPointer;
use crate::re::NiNode;
use crate::re::NiSmartPointer::NiPointer;
use crate::re::TESCameraState::TESCameraState;
use crate::re::offsets_rtti::RTTI_TESCamera;
use crate::re::offsets_vtable::VTABLE_TESCamera;
use crate::rel::id::VariantID;
use core::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct TESCamera {
    pub vtable: *const TESCameraVtbl,                  // 0x00
    pub rotationInput: BSTPoint2<f32>,                 // 0x08
    pub translationInput: BSTPoint3<f32>,              // 0x10
    pub zoomInput: f32,                                // 0x1C
    pub cameraRoot: NiPointer<NiNode>,                 // 0x20
    pub currentState: BSTSmartPointer<TESCameraState>, // 0x28
    pub enabled: bool,                                 // 0x30
    pub pad31: u8,                                     // 0x31
    pub pad32: u16,                                    // 0x32
    pub pad34: u32,                                    // 0x34
}
const _: () = assert!(std::mem::size_of::<TESCamera>() == 0x38);

impl TESCamera {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_TESCamera;

    /// Address & offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_TESCamera;

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 32290, ae_id = 33026)]
    #[inline]
    pub fn set_state(&mut self, state: *mut TESCameraState) {}
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TESCameraVtbl {
    /// C++ Destructor `~TESCamera`
    pub CxxDrop: fn(this: *mut c_void), // 0x00
    pub SetCameraRoot: fn(this: *mut c_void, root: NiPointer<NiNode>), // 0x01 - { if (cameraRoot != a_root) cameraRoot }
    pub Update: fn(this: *mut c_void),                                 // 0x02
}
const _: () = {
    const VFUNC_COUNT: usize = 0x3;

    const EXPECTED_SIZE: usize = VFUNC_COUNT * core::mem::size_of::<usize>();
    assert!(core::mem::size_of::<TESCameraVtbl>() == EXPECTED_SIZE);
};

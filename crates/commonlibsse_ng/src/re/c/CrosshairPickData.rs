use crate::re::BSPointerHandle::ObjectRefHandle;
use crate::re::NiPoint3::NiPoint3;
use crate::re::NiSmartPointer::NiPointer;

#[derive(Debug, Clone, PartialEq)]
pub struct bhkSimpleShapePhantom;
impl crate::re::NiSmartPointer::RefCountable for bhkSimpleShapePhantom {
    fn inc_ref_count(&self) {
        todo!()
    }

    fn dec_ref_count(&mut self) {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum VR_DEVICE {
    LeftController,
    RightController,
    Headset, // Can be kGamepad when in gamepad mode
}

impl VR_DEVICE {
    pub const TOTAL: usize = 3;
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrosshairPickData {
    pad00: u32,                                       // 00
    target: [ObjectRefHandle; VR_DEVICE::TOTAL],      // 04
    targetActor: [ObjectRefHandle; VR_DEVICE::TOTAL], // 10
    grabPickRef: [ObjectRefHandle; VR_DEVICE::TOTAL], // 1C
    collisionPoint: [NiPoint3; VR_DEVICE::TOTAL],     // 28
    pad4C: u32,                                       // 4C
    unk50: [u64; VR_DEVICE::TOTAL],                   // 50
    unk68: f32,                                       // 68
    unk6C: f32,                                       // 68
    unk70: u32,                                       // 70
    unk74: u32,                                       // 74
    unk78: NiPointer<bhkSimpleShapePhantom>,          // 78
    unk80: u32,                                       // 80
    unk84: u16,                                       // 84
    unk86: u8,                                        // 86
}
const _: () = assert!(core::mem::size_of::<CrosshairPickData>() == 0x88);

impl CrosshairPickData {
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut *mut CrosshairPickData",
        default = "None",
        deref_once,
        id(se = 515446, ae = 401585)
    )]
    pub fn get_singleton() -> Option<&'static CrosshairPickData> {
        |deref_type: DerefType| unsafe { deref_type.as_ref() }
    }
}

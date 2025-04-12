#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NiQuaternion {
    pub w: f32, // 0x0
    pub x: f32, // 0x4
    pub y: f32, // 0x8
    pub z: f32, // 0xC
}
const _: () = assert!(core::mem::size_of::<NiQuaternion>() == 0x10);

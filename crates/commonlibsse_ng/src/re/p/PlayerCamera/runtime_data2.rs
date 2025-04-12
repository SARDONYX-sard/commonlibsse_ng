use crate::re::NiPoint3::NiPoint3;

#[derive(Debug)]
#[repr(C)]
pub struct RUNTIME_DATA2 {
    pub worldFOV: f32,             // 0x13C, VR: 0x158
    pub firstPersonFOV: f32,       // 0x140, VR: 0x15c
    pub pos: NiPoint3,             // 0x144, VR: 0x160 - ?
    pub idleTimer: f32,            // 0x150, VR: 0x16c - ?
    pub yaw: f32,                  // 0x154, VR: 0x170 - ? - in radians
    pub unk158: u32,               // 0x158 - ?
    pub unk15C: u32,               // 0x15C - ?
    pub allowAutoVanityMode: bool, // 0x160, VR: 0x17c
    pub bowZoomedIn: bool,         // 0x161, VR: 0x17d
    pub isWeapSheathed: bool,      // 0x162, VR: 0x17e - ?
    pub isProcessed: bool,         // 0x163, VR: 0x17f - ?
    pub unk164: u8,                // 0x164
    pub unk165: u8,                // 0x165
    pub pad166: u16,               // 0x166
}
const _: () = assert!(core::mem::size_of::<RUNTIME_DATA2>() == 0x2C);

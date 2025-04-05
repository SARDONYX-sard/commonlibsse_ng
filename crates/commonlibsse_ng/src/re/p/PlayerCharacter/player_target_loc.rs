use crate::re::BSCoreTypes::RefHandle;
use crate::re::NiPoint3::NiPoint3;
use crate::re::TESObjectCELL::TESObjectCELL;
use crate::re::TESWorldSpace::TESWorldSpace;

#[repr(C)]
#[derive(Debug)]
pub struct PLAYER_TARGET_LOC {
    world: *mut TESWorldSpace,       // 00
    interior: *mut TESObjectCELL,    // 08
    location: NiPoint3,              // 10
    angle: NiPoint3,                 // 1C
    arrivalFunc: extern "C" fn(i64), // 28
    arrivalFuncData: i64,            // 30
    furnitureRef: RefHandle,         // 38
    fastTravelMarker: RefHandle,     // 3C
    resetWeather: bool,              // 40
    allowAutoSave: bool,             // 41
    isValid: bool,                   // 42
    pad43: u8,                       // 43
    pad44: u32,                      // 44
}
const _: () = {
    assert!(core::mem::offset_of!(PLAYER_TARGET_LOC, world) == 0x00);
    assert!(core::mem::offset_of!(PLAYER_TARGET_LOC, interior) == 0x08);
    assert!(core::mem::offset_of!(PLAYER_TARGET_LOC, location) == 0x10);
    assert!(core::mem::offset_of!(PLAYER_TARGET_LOC, angle) == 0x1c);
    assert!(core::mem::offset_of!(PLAYER_TARGET_LOC, arrivalFunc) == 0x28);
    assert!(core::mem::offset_of!(PLAYER_TARGET_LOC, arrivalFuncData) == 0x30);
    assert!(core::mem::offset_of!(PLAYER_TARGET_LOC, furnitureRef) == 0x38);
    assert!(core::mem::offset_of!(PLAYER_TARGET_LOC, fastTravelMarker) == 0x3C);
    assert!(core::mem::offset_of!(PLAYER_TARGET_LOC, resetWeather) == 0x40);
    assert!(core::mem::offset_of!(PLAYER_TARGET_LOC, allowAutoSave) == 0x41);
    assert!(core::mem::offset_of!(PLAYER_TARGET_LOC, isValid) == 0x42);
    assert!(core::mem::offset_of!(PLAYER_TARGET_LOC, pad43) == 0x43);
    assert!(core::mem::offset_of!(PLAYER_TARGET_LOC, pad44) == 0x44);

    assert!(core::mem::size_of::<PLAYER_TARGET_LOC>() == 0x48);
};

#[repr(C)]
#[derive(Debug)]
pub struct VR_PLAYER_TARGET_LOC {
    world: *mut TESWorldSpace,    // 0x00
    interior: *mut TESObjectCELL, // 0x08
    location: NiPoint3,           // 0x10
    angle: NiPoint3,              // 0x1C
    arrivalFunc: fn(i64),         // 0x28
    arrivalFuncData: i64,         // 0x30
    furnitureRef: RefHandle,      // 0x38
    fastTravelMarker: RefHandle,  // 0x3C
    unk_40: f32,                  // 0x40 - New in VR, always 0.0 in vanilla
    unk44: u8,                    // 0x44
    resetWeather: bool,           // 0x45
    allowAutoSave: u8,            // 0x46
    isValid: bool,                // 0x47
    unk48: u8,                    // 0x48
    unk49: u8,                    // 0x49
    unk4A: u8,                    // 0x4A
    unk4B: u8,                    // 0x4B
    unk4C: u32,                   // 0x4C
}
const _: () = assert!(core::mem::size_of::<VR_PLAYER_TARGET_LOC>() == 0x50);

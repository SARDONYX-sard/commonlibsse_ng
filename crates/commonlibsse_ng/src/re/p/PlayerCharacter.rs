pub mod vr;

use crate::re::{
    BSCoreTypes::RefHandle, Character::Character, NiPoint3::NiPoint3, TESObjectCELL::TESObjectCELL,
    TESWorldSpace::TESWorldSpace,
};

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum PLAYER_ACTION {
    #[default]
    None = 0,
    SwingMeleeWeapon,
    CastProjectileSpell,
    ShootBow,
    ZKeyObject,
    Jumping,
    KnockingOverObjects,
    StandOnTableChair,
    IronSights,
    DestroyObject,
    LockedObject,
    Pickpocket,
    CastSelfSpell,
    Shout,
    ActorCollision,

    Total,
    InvalidMarker,
}

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CrimeGoldStruct {
    violentCur: f32,       // 00
    nonViolentCur: f32,    // 04
    nonViolentInfamy: f32, // 08
    violentInfamy: f32,    // 0C
}
const _: () = {
    assert!(core::mem::offset_of!(CrimeGoldStruct, violentCur) == 0x0);
    assert!(core::mem::offset_of!(CrimeGoldStruct, nonViolentCur) == 0x4);
    assert!(core::mem::offset_of!(CrimeGoldStruct, nonViolentInfamy) == 0x8);
    assert!(core::mem::offset_of!(CrimeGoldStruct, violentInfamy) == 0xc);
    assert!(core::mem::size_of::<CrimeGoldStruct>() == 0x10);
};

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StolenItemValueStruct {
    unwitnessed: i32, // 0
    witnessed: i32,   // 4
}
const _: () = {
    assert!(core::mem::offset_of!(StolenItemValueStruct, unwitnessed) == 0x0);
    assert!(core::mem::offset_of!(StolenItemValueStruct, witnessed) == 0x4);
    assert!(core::mem::size_of::<StolenItemValueStruct>() == 0x8);
};

#[repr(C)]
pub struct FriendshipFactionsStruct {
    friend_counts: [u16; 4], // 0
}
const _: () = assert!(core::mem::size_of::<FriendshipFactionsStruct>() == 0x8);

#[repr(C)]
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
pub struct VR_PLAYER_TARGET_LOC {
    world: *mut TESWorldSpace,               // 00
    interior: *mut TESObjectCELL,            // 08
    location: NiPoint3,                      // 10
    angle: NiPoint3,                         // 1C
    arrivalFunc: Option<extern "C" fn(i64)>, // 28
    arrivalFuncData: i64,                    // 30
    furnitureRef: RefHandle,                 // 38
    fastTravelMarker: RefHandle,             // 3C
    unk_40: f32,                             // 40 - New in VR, always 0.0 in vanilla
    unk44: u8,                               // 44
    resetWeather: bool,                      // 45
    allowAutoSave: u8,                       // 46
    isValid: bool,                           // 47
    unk48: u8,                               // 48
    unk49: u8,                               // 49
    unk4A: u8,                               // 4A
    unk4B: u8,                               // 4B
    unk4C: u32,                              // 4C
}
const _: () = assert!(core::mem::size_of::<VR_PLAYER_TARGET_LOC>() == 0x50);

#[repr(C)]
pub struct PlayerActionObject {
    timer: f32,          // 0
    refObj: RefHandle,   // 4
    next: PLAYER_ACTION, // 8
}
const _: () = assert!(core::mem::size_of::<PlayerActionObject>() == 0x0C);

#[repr(C)]
pub struct PlayerCharacter {
    pub __base: Character,
}

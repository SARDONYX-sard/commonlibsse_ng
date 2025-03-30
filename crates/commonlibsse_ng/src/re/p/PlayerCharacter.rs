mod vr_node;

pub use self::vr_node::VRNodeData;
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

#[commonlibsse_ng_derive_internal::ffi_enum]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VR_Bow_State {
    #[default]
    None,
    NoAmmo,
    Idle,
    ArrowKnocked,
}

#[repr(C)]
pub struct CrimeGoldStruct {
    violent_cur: f32,        // 00
    non_violent_cur: f32,    // 04
    non_violent_infamy: f32, // 08
    violent_infamy: f32,     // 0C
}

#[repr(C)]
pub struct StolenItemValueStruct {
    unwitnessed: i32, // 0
    witnessed: i32,   // 4
}

#[repr(C)]
pub struct FriendshipFactionsStruct {
    friend_counts: [u16; 4], // 0
}

#[repr(C)]
pub struct PLAYER_TARGET_LOC {
    world: *mut TESWorldSpace,                // 00
    interior: *mut TESObjectCELL,             // 08
    location: NiPoint3,                       // 10
    angle: NiPoint3,                          // 1C
    arrival_func: Option<extern "C" fn(i64)>, // 28
    arrival_func_data: i64,                   // 30
    furniture_ref: RefHandle,                 // 38
    fast_travel_marker: RefHandle,            // 3C
    reset_weather: bool,                      // 40
    allow_auto_save: bool,                    // 41
    is_valid: bool,                           // 42
    pad43: u8,                                // 43
    pad44: u32,                               // 44
}

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

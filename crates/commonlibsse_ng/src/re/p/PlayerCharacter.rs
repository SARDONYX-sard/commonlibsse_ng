pub mod vr;

use crate::re::BGSActorCellEvent::BGSActorCellEvent;
use crate::re::BSCoreTypes::RefHandle;
use crate::re::BSPointerHandle::ObjectRefHandle;
use crate::re::BSTArray::BSTSmallArray;
use crate::re::BSTEvent::{BSTEventSink, BSTEventSource};
use crate::re::Character::Character;
use crate::re::FormTypes::FormType;
use crate::re::MenuModeChangeEvent::MenuModeChangeEvent;
use crate::re::MenuOpenCloseEvent::MenuOpenCloseEvent;
use crate::re::NiPoint3::NiPoint3;
use crate::re::PositionPlayerEvent::PositionPlayerEvent;
use crate::re::TESObjectCELL::TESObjectCELL;
use crate::re::TESTrackedStatsEvent::TESTrackedStatsEvent;
use crate::re::TESWorldSpace::TESWorldSpace;
use crate::re::UserEventEnabledEvent;
use crate::re::hkRefPtr::hkRefPtr;
use crate::re::offsets_rtti::RTTI_PlayerCharacter;
use crate::re::offsets_vtable::VTABLE_PlayerCharacter;
use crate::re::{BGSActorDeathEvent, TESObjectWEAP, bhkMouseSpringAction};
use crate::rel::id::VariantID;

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
pub struct PlayerActionObject {
    timer: f32,          // 0
    refObj: RefHandle,   // 4
    next: PLAYER_ACTION, // 8
}
const _: () = assert!(core::mem::size_of::<PlayerActionObject>() == 0x0C);

#[repr(C)]
#[derive(Debug)]
pub struct PlayerCharacter {
    pub __base: Character,                            // 000
    pub __base1: BSTEventSource<BGSActorCellEvent>,   // SE: 0x2D0, AE: 0x2D8, VR: 0x2E8
    pub __base2: BSTEventSource<BGSActorDeathEvent>,  // SE: 0x328, AE: 0x330, VR: 0x340
    pub __base3: BSTEventSource<PositionPlayerEvent>, // SE: 0x380, AE: 0x388, VR: 0x398
    pub __base4: BSTEventSink<MenuOpenCloseEvent>,    // SE,VR: 0x2B0, AE: 0x2B8
    pub __base5: BSTEventSink<MenuModeChangeEvent>,   // SE,VR: 0x2B8, AE: 0x2C0
    pub __base6: BSTEventSink<UserEventEnabledEvent>, // SE,VR: 0x2C0, AE: 0x2C8
    pub __base7: BSTEventSink<TESTrackedStatsEvent>,  // SE,VR: 0x2C8, AE: 0x2D0
}

impl PlayerCharacter {
    pub const RTTI: VariantID = RTTI_PlayerCharacter;
    pub const VTABLE: [VariantID; 17] = VTABLE_PlayerCharacter;
    pub const FORM_TYPE: FormType = FormType::ActorCharacter;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventType {
    Thief = 3,
    Container = 5,
    DeadBody = 6,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GrabbingType {
    #[default]
    None = 0,
    Normal = 1,
    Telekinesis = 2,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ByCharGenFlag {
    #[default]
    None = 0,
    DisableSaving = 1 << 0,
    HandsBound = 1 << 2,
}

#[repr(C)]
#[derive(Debug)]
pub struct GrabData {
    grabSpring: BSTSmallArray<hkRefPtr<bhkMouseSpringAction>, 4>,
    grabbedObject: ObjectRefHandle,
    grabObjectWeight: f32,
    grabDistance: f32,
    unk004: f32,
    unk008: u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct VRGrabData {
    grabSpring: BSTSmallArray<hkRefPtr<bhkMouseSpringAction>, 4>,
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

#[repr(C)]
#[derive(Debug, Default)]
pub struct PlayerFlags {
    travelUseDoor: bool,
    fastTraveling: bool,
    overAutoAimTarget: bool,
    showQuestItems: bool,
    unk0_4: bool,
    hasQueuedEquipAnim: bool,
    escaping: bool,
    forceQuestTargetRepath: bool,
    unk1_0: bool,
    unk1_1: bool,
    sleeping: bool,
    unk1_3: bool,
    unk1_4: bool,
    unk1_5: bool,
    greetingPlayer: bool,
    unk1_7: bool,
    unk2_0: bool,
    aiControlledToPos: bool,
    aiControlledFromPos: bool,
    aiControlledPackage: bool,
    returnToLastKnownGoodPosition: bool,
    isBeingChased: bool,
    unk2_6: bool,
    unk2_7: bool,
    isInThirdPersonMode: bool,
    unk3_1: bool,
    unk3_2: bool,
    unk3_3: bool,
    target3DDistant: bool,
    isInCombat: bool,
    attemptedYieldInCurrentCombat: bool,
    unk3_7: bool,
    isLoading: bool,
    shouldUpdateCrosshair: bool,
    unk4_2: bool,
    healthTutorialShown: bool,
    magickaTutorialShown: bool,
    staminaTutorialShown: bool,
    goToJailQueued: bool,
    unk4_7: bool,
    isSprinting: bool,
    isSungazing: bool,
    dragonRideTargetLocked: bool,
    everModded: bool,
    servingJailTime: bool,
    extra_flags: [bool; 16], // Placeholder for additional flags
}

#[repr(C)]
#[derive(Debug)]
pub struct QueuedWeapon {
    rightHandWeapon: *mut TESObjectWEAP,
    leftHandWeapon: *mut TESObjectWEAP,
}

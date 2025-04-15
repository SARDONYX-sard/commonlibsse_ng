use crate::re::ActorValues::{ActorValue_CEnum, ActorValue_u8};

use super::RangedData;

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum WEAPON_HIT_BEHAVIOR {
    Normal = 0,
    DismemberOnly = 1,
    ExplodeOnly = 2,
    NoDismemberOrExplode = 3,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum WEAPON_TYPE {
    HandToHandMelee = 0,
    OneHandSword = 1,
    OneHandDagger = 2,
    OneHandAxe = 3,
    OneHandMace = 4,
    TwoHandSword = 5,
    TwoHandAxe = 6,
    Bow = 7,
    Staff = 8,
    Crossbow = 9,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum Flag2 {
    None = 0,
    PlayerOnly = 1 << 0,
    NPCsUseAmmo = 1 << 1,
    NoJamAfterReload = 1 << 2, // unused
    MinorCrime = 1 << 4,
    RangeFixed = 1 << 5,
    NotUsedInNormalCombat = 1 << 6,
    DontUse3rdPersonISAnim = 1 << 8, // unused
    BurstShot = 1 << 9,
    RumbleAlternate = 1 << 10,
    LongBursts = 1 << 11,
    NonHostile = 1 << 12,
    BoundWeapon = 1 << 13,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum AttackAnimation {
    AttackLeft = 26,
    AttackRight = 32,
    Attack3 = 38,
    Attack4 = 44,
    Attack5 = 50,
    Attack7 = 62,
    Attack8 = 68,
    AttackLoop = 74,
    AttackSpin = 80,
    AttackSpin2 = 86,
    PlaceMine = 97,
    PlaceMine2 = 103,
    AttackThrow = 109,
    AttackThrow2 = 115,
    AttackThrow3 = 121,
    AttackThrow4 = 127,
    AttackThrow5 = 133,
    Default = 255,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Flag {
    None = 0,
    IgnoresNormalWeaponResistance = 1 << 0,
    Automatic = 1 << 1, // unused
    HasScope = 1 << 2,  // unused
    CantDrop = 1 << 3,
    HideBackpack = 1 << 4,             // unused
    EmbeddedWeapon = 1 << 5,           // unused
    DontUseFirstPersonISAnim = 1 << 6, // unused
    NonPlayable = 1 << 7,
}

#[repr(C)]
#[derive(Debug)]
pub struct Data {
    pub rangedData: *mut RangedData,      // 0x00
    pub speed: f32,                       // 0x08
    pub reach: f32,                       // 0x0C
    pub minRange: f32,                    // 0x10
    pub maxRange: f32,                    // 0x14
    pub animationAttackMult: f32,         // 0x18
    pub unk1C: f32,                       // 0x1C
    pub staggerValue: f32,                // 0x20
    pub hitBehavior: WEAPON_HIT_BEHAVIOR, // 0x24
    pub skill: ActorValue_CEnum,          // 0x28
    pub resistance: ActorValue_CEnum,     // 0x2C
    pub flags2: Flag2,                    // 0x30
    pub baseVATSToHitChance: u8,          // 0x32
    pub attackAnimation: AttackAnimation, // 0x33
    pub embeddedWeaponAV: ActorValue_u8,  // 0x34 - unused
    pub animationType: WEAPON_TYPE,       // 0x35
    pub flags: Flag,                      // 0x36
    pub unk37: u8,                        // 0x37
}
const _: () = assert!(core::mem::size_of::<Data>() == 0x38);

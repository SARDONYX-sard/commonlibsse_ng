use crate::re::BSTArray::BSTArray;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct CachedValueData {
    value: f32,    // 0x0
    invalid: bool, // 0x4
    pad5: u8,      // 0x5
    pad6: u16,     // 0x6
}
const _: () = assert!(core::mem::size_of::<CachedValueData>() == 0x8);

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum Flags {
    None = 0,
    Radius = 1 << 0,
    Width = 1 << 1,
    Length = 1 << 2,
    DPS = 1 << 3,
    MedicineEffectivenessMult = 1 << 4,
    EyeLevel = 1 << 9,
    ConditionPreventsRun = 1 << 10,
    ForwardLength = 1 << 11,
    ActorIsGhost = 1 << 20,
    HealthDamaged = 1 << 21,
    MagickaPointsDamaged = 1 << 22,
    StaminaDamaged = 1 << 23,
    OwnerIsNPC = 1 << 25,
    OwnerIsUndead = 1 << 26,
    OwnerIsInCombatantFaction = 1 << 27,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum BooleanValue {
    None = 0,
    ConditionPreventsRun = 1 << 0,
    OwnerIsNPC = 1 << 1,
    OwnerIsUndead = 1 << 2,
    OwnerIsInCombatantFaction = 1 << 3,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct CachedValues {
    cachedRadius: f32,                             // 0x00
    cachedWidth: f32,                              // 0x04
    cachedLength: f32,                             // 0x08
    cachedForwardLength: f32,                      // 0x0C
    cachedDPS: f32,                                // 0x10
    cachedEyeLevel: f32,                           // 0x14
    cachedWalkSpeed: f32,                          // 0x18
    cachedRunSpeed: f32,                           // 0x1C
    cachedJogSpeed: f32,                           // 0x20
    cachedFastWalkSpeed: f32,                      // 0x24
    booleanValues: BooleanValue,                   // 0x28
    flags: Flags,                                  // 0x2C
    actorValueCache: BSTArray<CachedValueData>,    // 0x30
    maxActorValueCache: BSTArray<CachedValueData>, // 0x48
}
const _: () = assert!(core::mem::size_of::<CachedValues>() == 0x60);

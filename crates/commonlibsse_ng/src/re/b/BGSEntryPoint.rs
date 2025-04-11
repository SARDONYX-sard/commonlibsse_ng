use core::{ffi::c_char, ptr::NonNull};

#[commonlibsse_ng_derive_internal::ffi_enum]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ENTRY_POINT {
    CalculateWeaponDamage = 0,
    CalculateMyCriticalHitChance = 1,
    CalculateMyCriticalHitDamage = 2,
    CalculateMineExplodeChance = 3,
    AdjustLimbDamage = 4,
    AdjustBookSkillPoints = 5,
    ModRecoveredHealth = 6,
    GetShouldAttack = 7,
    ModBuyPrices = 8,
    AddLeveledListOnDeath = 9,
    GetMaxCarryWeight = 10,
    ModAddictionChance = 11,
    ModAddictionDuration = 12,
    ModPositiveChemDuration = 13,
    Activate = 14,
    IgnoreRunningDuringDetection = 15,
    IgnoreBrokenLock = 16,
    ModEnemyCriticalHitChance = 17,
    ModSneakAttackMult = 18,
    ModMaxPlaceableMines = 19,
    ModBowZoom = 20,
    ModRecoverArrowChance = 21,
    ModSkillUse = 22,
    ModTelekinesisDistance = 23,
    ModTelekinesisDamageMult = 24,
    ModTelekinesisDamage = 25,
    ModBashingDamage = 26,
    ModPowerAttackStamina = 27,
    ModPowerAttackDamage = 28,
    ModSpellMagnitude = 29,
    ModSpellDuration = 30,
    ModSecondaryValueWeight = 31,
    ModArmorWeight = 32,
    ModIncomingStagger = 33,
    ModTargetStagger = 34,
    ModAttackDamage = 35,
    ModIncomingDamage = 36,
    ModTargetDamageResistance = 37,
    ModSpellCost = 38,
    ModPercentBlocked = 39,
    ModShieldDeflectArrowChance = 40,
    ModIncomingSpellMagnitude = 41,
    ModIncomingSpellDuration = 42,
    ModPlayerIntimidation = 43,
    ModPlayerReputation = 44,
    ModFavorPoints = 45,
    ModBribeAmount = 46,
    ModDetectionLight = 47,
    ModDetectionMovement = 48,
    ModSoulGemRecharge = 49,
    SetSweepAttack = 50,
    ApplyCombatHitSpell = 51,
    ApplyBashingSpell = 52,
    ApplyReanimateSpell = 53,
    SetBooleanGraphVariable = 54,
    ModSpellCastingSoundEvent = 55,
    ModPickpocketChance = 56,
    ModDetectionSneakSkill = 57,
    ModFallingDamage = 58,
    ModLockpickSweetSpot = 59,
    ModSellPrices = 60,
    CanPickpocketEquippedItem = 61,
    ModLockpickLevelAllowed = 62,
    SetLockpickStartingArc = 63,
    SetProgressionPicking = 64,
    MakeLockpicksUnbreakable = 65,
    ModAlchemyEffectiveness = 66,
    ApplyWeaponSwingSpell = 67,
    ModCommandedActorLimit = 68,
    ApplySneakingSpell = 69,
    ModPlayerMagicSlowdown = 70,
    ModWardMagickaAbsorptionPct = 71,
    ModInitialIngredientEffectsLearned = 72,
    PurifyAlchemyIngredients = 73,
    FilterActivation = 74,
    CanDualCastSpell = 75,
    ModTemperingHealth = 76,
    ModEnchantmentPower = 77,
    ModSoulPctCapturedToWeapon = 78,
    ModSoulGemEnchanting = 79,
    ModNumberAppliedEnchantmentsAllowed = 80,
    SetActivateLabel = 81,
    ModShoutOK = 82,
    ModPoisonDoseCount = 83,
    ShouldApplyPlacedItem = 84,
    ModArmorRating = 85,
    ModLockpickingCrimeChance = 86,
    ModIngredientsHarvested = 87,
    ModSpellRange_TargetLoc = 88,
    ModPotionsCreated = 89,
    ModLockpickingKeyRewardChance = 90,
    AllowMountActor = 91,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntryPointParameter {
    pub name: *const c_char, // 0x00
    pub nonActor: bool,      // 0x08
    pub pad09: u8,           // 0x09
    pub pad0A: u16,          // 0x0A
    pub pad0C: u32,          // 0x0C
}
const _: () = assert!(core::mem::size_of::<EntryPointParameter>() == 0x10);

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntryPointParameters {
    pub count: u32,                     // 0x00
    pub pad04: u32,                     // 0x04
    pub data: *mut EntryPointParameter, // 0x08
}
const _: () = assert!(core::mem::size_of::<EntryPointParameters>() == 0x10);

// This is a placeholder. You will need to define this enum appropriately.
#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntryPointFunctionType {
    // Example variant(s)
    Example = 0,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntryPoint {
    pub name: *const c_char,                  // 0x00
    pub parameters: EntryPointParameters,     // 0x08
    pub functionType: EntryPointFunctionType, // 0x18
    pub pad1C: u32,                           // 0x1C
}
const _: () = assert!(core::mem::size_of::<EntryPoint>() == 0x20);

// Equivalent to: static EntryPoint* GetEntryPoint(...)
pub fn get_entry_point(entry_point: ENTRY_POINT) -> Option<NonNull<EntryPoint>> {
    if (entry_point as u32) < ENTRY_POINT_CEnum::count() as u32 {
        entry_points(entry_point)
    } else {
        None
    }
}

#[commonlibsse_ng_derive_internal::relocate(
    cast_as = "*mut EntryPoint",
    default = "None",
    id(se = 675707, ae = 368994)
)]
#[inline]
fn entry_points(entry_point: ENTRY_POINT) -> Option<NonNull<EntryPoint>> {
    |as_type| unsafe { NonNull::new(as_type.add(entry_point as usize)) }
}

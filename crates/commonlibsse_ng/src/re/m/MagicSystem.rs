use crate::re::BGSSoundDescriptorForm::BGSSoundDescriptorForm;
use crate::re::BSString::BSString;
use crate::re::MagicItem::MagicItem;

#[commonlibsse_ng_derive_internal::ffi_enum]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CannotCastReason {
    OK = 0,
    Magicka = 1,
    PowerUsed = 2,
    RangedUnderWater = 3,
    MultipleCast = 4,
    ItemCharge = 5,
    CastWhileShouting = 6,
    ShoutWhileCasting = 7,
    ShoutWhileRecovering = 8,
    CustomReasonNoStart = 100,
}

#[commonlibsse_ng_derive_internal::ffi_enum]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CastingSource {
    LeftHand = 0,
    RightHand = 1,
    Other = 2,
    Instant = 3,
}

#[commonlibsse_ng_derive_internal::ffi_enum]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CastingType {
    ConstantEffect = 0,
    FireAndForget = 1,
    Concentration = 2,
    Scroll = 3,
}

#[commonlibsse_ng_derive_internal::ffi_enum]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Delivery {
    Self_ = 0, // `Self` is a reserved keyword
    Touch = 1,
    Aimed = 2,
    TargetActor = 3,
    TargetLocation = 4,
    Total = 5,
}

#[commonlibsse_ng_derive_internal::ffi_enum]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SoundID {
    DrawSheatheLPM = 0,
    Charge = 1,
    ReadyLoop = 2,
    Release = 3,
    CastLoop = 4,
    Hit = 5,
}

/// The following discriminates are bitflags because they have the same value
/// - Potion & Alchemy,
/// - WortCraft & Ingredient
#[commonlibsse_ng_derive_internal::to_bitflags]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpellType {
    Spell = 0,
    Disease = 1,
    Power = 2,
    LesserPower = 3,
    Ability = 4,
    Poison = 5,
    Enchantment = 6,
    Potion = 7,
    Alchemy = 7,
    WortCraft = 8,
    Ingredient = 8,
    LeveledSpell = 9,
    Addiction = 10,
    VoicePower = 11,
    StaffEnchantment = 12,
    Scroll = 13,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WardState {
    None = 0,
    Absorb = 1,
    Break = 2,
    Total = 3,
}

#[commonlibsse_ng_derive_internal::relocate_fn(se_id = 11295, ae_id = 11423)]
pub fn GetCannotCastString(reason: CannotCastReason) -> *const u8 {}

#[commonlibsse_ng_derive_internal::relocate_fn(se_id = 11294, ae_id = 11422)]
pub fn GetMagicCasterTargetUpdateInterval() -> f32 {}

#[commonlibsse_ng_derive_internal::relocate_fn(se_id = 11286, ae_id = 11411)]
pub fn GetMagicFailureSound(ty: SpellType) -> *mut BGSSoundDescriptorForm {}

#[commonlibsse_ng_derive_internal::relocate_fn(se_id = 11299, ae_id = 11427)]
pub fn GetMagicItemDescription(
    out: *mut BSString,
    magic_item: *mut MagicItem,
    begin_tag_format: *const u8,
    end_tag_format: *const u8,
) {
}

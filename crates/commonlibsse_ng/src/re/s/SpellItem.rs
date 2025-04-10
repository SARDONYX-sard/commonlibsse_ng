use crate::re::BGSEquipType::{BGSEquipType, BGSEquipTypeVtbl};
use crate::re::BGSMenuDisplayObject::{BGSMenuDisplayObject, BGSMenuDisplayObjectVtbl};
use crate::re::BGSPerk::BGSPerk;
use crate::re::FormTypes::FormType;
use crate::re::MagicItem::{MagicItem, MagicItemVtbl};
use crate::re::MagicSystem;
use crate::re::TESDescription::{TESDescription, TESDescriptionVtbl};
use crate::re::offsets_rtti::RTTI_SpellItem;
use crate::re::offsets_vtable::VTABLE_SpellItem;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct SpellItem {
    pub __base: MagicItem,             // 0x00
    pub __base1: BGSEquipType,         // 0x90
    pub __base2: BGSMenuDisplayObject, // 0xA0
    pub __base3: TESDescription,       // 0xB0
    pub data: Data,                    // 0xC0
}
const _: () = assert!(core::mem::size_of::<SpellItem>() == 0xE8);

impl SpellItem {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_SpellItem;

    /// Address & offset of the virtual function table.
    ///
    /// The number of tables is the same as the number of classes with inherited virtual functions.
    pub const VTABLE: [VariantID; 6] = VTABLE_SpellItem;

    /// The `FormType` value for SpellItem.
    pub const FORM_TYPE: FormType = FormType::Spell;
}

#[repr(C)]
pub struct SpellItemVtbl {
    pub __base: MagicItemVtbl,
    pub __base1: BGSEquipTypeVtbl,
    pub __base2: BGSMenuDisplayObjectVtbl,
    pub __base3: TESDescriptionVtbl,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Data {
    costOverride: i32,                     // 0x00
    flags: SpellFlag,                      // 0x04
    spellType: MagicSystem::SpellType,     // 0x08
    chargeTime: f32,                       // 0x0C
    castingType: MagicSystem::CastingType, // 0x10
    delivery: MagicSystem::Delivery,       // 0x14
    castDuration: f32,                     // 0x18
    range: f32,                            // 0x1C
    castingPerk: *mut BGSPerk,             // 0x20
}
const _: () = assert!(core::mem::size_of::<Data>() == 0x28);

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum SpellFlag {
    None = 0,
    CostOverride = 1 << 0,
    FoodItem = 1 << 1,
    ExtendDuration = 1 << 3,
    PCStartSpell = 1 << 17,
    InstantCast = 1 << 18,
    IgnoreLOSCheck = 1 << 19,
    IgnoreResistance = 1 << 20,
    NoAbsorb = 1 << 21,
    NoDualCastMods = 1 << 23,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum RecordFlag {
    Deleted = 1 << 5,
    Ignored = 1 << 12,
}

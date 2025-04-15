use crate::re::BGSListForm::BGSListForm;
use crate::re::FormTypes::FormType;
use crate::re::MagicItem::{MagicItem, MagicItemVtbl};
use crate::re::MagicSystem;
use crate::re::offsets_rtti::RTTI_EnchantmentItem;
use crate::re::offsets_vtable::VTABLE_EnchantmentItem;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct EnchantmentItem {
    pub __base: MagicItem, // 0x00
    pub data: Data,
}
const _: () = assert!(core::mem::size_of::<EnchantmentItem>() == 0xC0);

impl EnchantmentItem {
    pub const RTTI: VariantID = RTTI_EnchantmentItem;
    pub const VTABLE: [VariantID; 3] = VTABLE_EnchantmentItem;
    pub const FORM_TYPE: FormType = FormType::Enchantment;
}

#[repr(C)]
pub struct EnchantmentItemVtbl {
    pub __base: MagicItemVtbl,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum EnchantmentFlag {
    None = 0,
    CostOverride = 1 << 0,
    FoodItem = 1 << 1,
    ExtendDuration = 1 << 3,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum RecordFlag {
    Deleted = 1 << 5,
    Ignored = 1 << 12,
}

#[repr(C)]
#[derive(Debug)]
pub struct Data {
    pub costOverride: i32,                     // 0x00
    pub flags: EnchantmentFlag,                // 0x04
    pub castingType: MagicSystem::CastingType, // 0x08
    pub chargeOverride: i32,                   // 0x0C
    pub delivery: MagicSystem::Delivery,       // 0x10
    pub spellType: MagicSystem::SpellType,     // 0x14
    pub chargeTime: f32,                       // 0x18
    pub pad1C: u32,                            // 0x1C
    pub baseEnchantment: *mut EnchantmentItem, // 0x20
    pub wornRestrictions: *mut BGSListForm,    // 0x28
}
const _: () = assert!(core::mem::size_of::<Data>() == 0x30);

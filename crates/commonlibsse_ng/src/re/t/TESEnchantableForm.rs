use crate::re::BaseFormComponent::{BaseFormComponent, BaseFormComponentVtbl};
use crate::re::EnchantmentItem::EnchantmentItem;
use crate::re::MagicSystem::CastingType_CEnum;
use crate::re::offsets_rtti::RTTI_TESEnchantableForm;
use crate::re::offsets_vtable::VTABLE_TESEnchantableForm;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct TESEnchantableForm {
    pub __base: BaseFormComponent,            // 0x00
    pub formEnchanting: *mut EnchantmentItem, // 0x08
    pub castingType: CastingType_CEnum,       // 0x10
    pub amountOfEnchantment: u16,             // 0x12
    pub pad14: u32,                           // 0x14
}
const _: () = assert!(core::mem::size_of::<TESEnchantableForm>() == 0x18);

impl TESEnchantableForm {
    pub const RTTI: VariantID = RTTI_TESEnchantableForm;
    pub const VTABLE: [VariantID; 1] = VTABLE_TESEnchantableForm;
}

#[repr(C)]
#[derive(Debug)]
pub struct TESEnchantableFormVtbl {
    pub __base: BaseFormComponentVtbl, // 0x00
}
const _: () = {
    const VFUNC_COUNT: usize = 0x4;

    const EXPECTED_SIZE: usize = VFUNC_COUNT * core::mem::size_of::<usize>();
    assert!(core::mem::size_of::<TESEnchantableFormVtbl>() == EXPECTED_SIZE);
};

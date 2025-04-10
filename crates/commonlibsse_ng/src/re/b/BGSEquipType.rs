use core::ptr::NonNull;

use crate::re::BGSEquipSlot::BGSEquipSlot;
use crate::re::BaseFormComponent::{BaseFormComponent, BaseFormComponentVtbl};
use crate::re::offsets_rtti::RTTI_BGSEquipType;
use crate::re::offsets_vtable::VTABLE_BGSEquipType;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct BGSEquipType {
    pub __base: BaseFormComponent,                // 0x0
    pub equipSlot: Option<NonNull<BGSEquipSlot>>, // 0x8
}
const _: () = assert!(core::mem::size_of::<BGSEquipType>() == 0x10);

impl BGSEquipType {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_BGSEquipType;

    /// Address & offset of the virtual function table.
    ///
    /// The number of tables is the same as the number of classes with inherited virtual functions.
    pub const VTABLE: [VariantID; 1] = VTABLE_BGSEquipType;
}

pub struct BGSEquipTypeVtbl {
    pub __base: BaseFormComponentVtbl,
    pub GetEquipSlot: fn(this: &BGSEquipType),
    pub SetEquipSlot: fn(this: &mut BGSEquipType, slot: *mut BGSEquipSlot),
}

#[commonlibsse_ng_derive_internal::ffi_enum]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EQUIPPED_ITEM_TYPE {
    Spell = 24,
    Shield = 25,
    Torch = 26,
}

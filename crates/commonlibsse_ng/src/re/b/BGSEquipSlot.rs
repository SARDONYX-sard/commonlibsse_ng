use crate::re::BSTArray::BSTArray;
use crate::re::TESForm::{TESForm, TESFormVtbl};
use crate::re::offsets_rtti::RTTI_BGSEquipSlot;
use crate::re::offsets_vtable::VTABLE_BGSEquipSlot;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct BGSEquipSlot {
    pub __base: TESForm,                          // 0x0
    pub parentSlots: BSTArray<*mut BGSEquipSlot>, // 0x20
    pub flags: Flag,                              // 0x38
    pub pad3C: u32,                               // 0x3C
}
const _: () = assert!(core::mem::size_of::<BGSEquipSlot>() == 0x40);

impl BGSEquipSlot {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_BGSEquipSlot;

    /// Address & offset of the virtual function table.
    ///
    /// The number of tables is the same as the number of classes with inherited virtual functions.
    pub const VTABLE: [VariantID; 1] = VTABLE_BGSEquipSlot;
}

pub struct BGSEquipSlotVtbl {
    pub __base: TESFormVtbl,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Flag {
    #[default]
    None = 0,
    UseAllParents = 1 << 0,
    ParentsOptional = 1 << 1,
    ItemSlot = 1 << 2,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum RecordFlag {
    Deleted = 1 << 5,
    Ignored = 1 << 12,
}

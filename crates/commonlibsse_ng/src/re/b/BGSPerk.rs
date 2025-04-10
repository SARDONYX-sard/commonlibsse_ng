use crate::re::BGSPerkEntry::BGSPerkEntry;
use crate::re::BSTArray::BSTArray;
use crate::re::FormTypes::FormType;
use crate::re::TESCondition::TESCondition;
use crate::re::TESDescription::{TESDescription, TESDescriptionVtbl};
use crate::re::TESForm::{TESForm, TESFormVtbl};
use crate::re::TESFullName::{TESFullName, TESFullNameVtbl};
use crate::re::TESIcon::{TESIcon, TESIconVtbl};
use crate::re::offsets_rtti::RTTI_BGSPerk;
use crate::re::offsets_vtable::VTABLE_BGSPerk;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct BGSPerk {
    pub __base: TESForm,                          // 0x00
    pub __base1: TESFullName,                     // 0x20
    pub __base2: TESDescription,                  // 0x30
    pub __base3: TESIcon,                         // 0x40
    pub data: PerkData,                           // 0x50
    pub pad55: u8,                                // 0x55
    pub pad56: u16,                               // 0x56
    pub perkConditions: TESCondition,             // 0x58
    pub perkEntries: BSTArray<*mut BGSPerkEntry>, // 0x60
    pub nextPerk: *mut BGSPerk,                   // 0x78 - NNAM
}
const _: () = assert!(core::mem::size_of::<BGSPerk>() == 0x80);

impl BGSPerk {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_BGSPerk;

    /// Address & offset of the virtual function table.
    ///
    /// The number of tables is the same as the number of classes with inherited virtual functions.
    pub const VTABLE: [VariantID; 4] = VTABLE_BGSPerk;

    /// The `FormType` value for BGSPerk.
    pub const FORM_TYPE: FormType = FormType::Perk;
}

#[repr(C)]
pub struct BGSPerkVtbl {
    pub __base: TESFormVtbl,
    pub __base1: TESFullNameVtbl,    // 0x20
    pub __base2: TESDescriptionVtbl, // 0x30
    pub __base3: TESIconVtbl,        // 0x40
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct PerkData {
    trait_: bool,   // 0x0
    level: i8,      // 0x1
    numRanks: i8,   // 0x2
    playable: bool, // 0x3
    hidden: bool,   // 0x4
}
const _: () = assert!(core::mem::size_of::<PerkData>() == 0x5);

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum RecordFlag {
    NonPlayable = 1 << 2,
    Deleted = 1 << 5,
    Ignored = 1 << 12,
}

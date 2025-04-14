use crate::re::BGSEquipType::{BGSEquipType, BGSEquipTypeVtbl};
use crate::re::BGSMenuDisplayObject::{BGSMenuDisplayObject, BGSMenuDisplayObjectVtbl};
use crate::re::FormTypes::FormType;
use crate::re::SpellItem::SpellItem;
use crate::re::TESDescription::{TESDescription, TESDescriptionVtbl};
use crate::re::TESForm::TESForm;
use crate::re::TESForm::TESFormVtbl;
use crate::re::TESFullName::{TESFullName, TESFullNameVtbl};
use crate::re::TESWordOfPower::TESWordOfPower;
use crate::re::offsets_rtti::RTTI_TESShout;
use crate::re::offsets_vtable::VTABLE_TESShout;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct TESShout {
    pub __base: TESForm,
    pub __base1: TESFullName,
    pub __base2: BGSMenuDisplayObject,
    pub __base3: BGSEquipType,
    pub __base4: TESDescription,
    pub variations: [Variation; 3], // 3: VariationID::count()
}
const _: () = assert!(core::mem::size_of::<TESShout>() == 0xA8);

impl TESShout {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_TESShout;

    /// Address & offset of the virtual function table.
    ///
    /// The number of tables is the same as the number of classes with inherited virtual functions.
    pub const VTABLE: [VariantID; 5] = VTABLE_TESShout;

    /// The `FormType` value for TESShout.
    pub const FORM_TYPE: FormType = FormType::Shout;
}

pub struct TESShoutVtbl {
    pub __base: TESFormVtbl,
    pub __base1: TESFullNameVtbl,
    pub __base2: BGSMenuDisplayObjectVtbl,
    pub __base3: BGSEquipTypeVtbl,
    pub __base4: TESDescriptionVtbl,
}
const _: () = {
    const VTABLE_SIZE: usize = core::mem::size_of::<TESShoutVtbl>();
    const EXPECTED_SIZE: usize = (0x36 + 1) * core::mem::size_of::<usize>();
    assert!(VTABLE_SIZE == EXPECTED_SIZE);
};

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum RecordFlag {
    Deleted = 1 << 5,
    TreatSpellsAsPowers = 1 << 7,
    Ignored = 1 << 12,
}

#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum VariationID {
    One = 0,
    Two,
    Three,
}

impl VariationID {
    pub const NONE: u32 = u32::MAX;
}
const _: () = assert!(VariationID_CEnum::count() == 3);

#[derive(Debug)]
pub struct Variation {
    pub word: *mut TESWordOfPower, // 0x00
    pub spell: *mut SpellItem,     // 0x08
    pub recoveryTime: f32,         // 0x10
    pub pad14: u32,                // 0x14
}

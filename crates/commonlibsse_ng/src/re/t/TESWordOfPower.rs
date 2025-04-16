use crate::re::BSFixedString::BSFixedString;
use crate::re::FormTypes::FormType;
use crate::re::TESForm::TESForm;
use crate::re::TESForm::TESFormVtbl;
use crate::re::TESFullName::{TESFullName, TESFullNameVtbl};
use crate::re::offsets_rtti::RTTI_TESWordOfPower;
use crate::re::offsets_vtable::VTABLE_TESWordOfPower;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct TESWordOfPower {
    pub __base: TESForm,            // 0x00
    pub __base1: TESFullName,       // 0x20
    pub translation: BSFixedString, // 0x30
}
const _: () = assert!(core::mem::size_of::<TESWordOfPower>() == 0x38);

impl TESWordOfPower {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_TESWordOfPower;

    /// Address & offset of the virtual function table.
    ///
    /// The number of tables is the same as the number of classes with inherited virtual functions.
    pub const VTABLE: [VariantID; 2] = VTABLE_TESWordOfPower;

    /// The `FormType` value for TESWordOfPower.
    pub const FORM_TYPE: FormType = FormType::WordOfPower;
}

#[repr(C)]
pub struct TESWordOfPowerVtbl {
    pub __base: TESFormVtbl,
    pub __base1: TESFullNameVtbl,
}
// const _: () = {
//     const VTABLE_SIZE: usize = core::mem::size_of::<TESWordOfPowerVtbl>();
//     const EXPECTED_SIZE: usize = (0x26 + 1) * core::mem::size_of::<usize>();
//     assert!(VTABLE_SIZE == EXPECTED_SIZE);
// };

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum RecordFlag {
    Deleted = 1 << 5,
    Ignored = 1 << 12,
}

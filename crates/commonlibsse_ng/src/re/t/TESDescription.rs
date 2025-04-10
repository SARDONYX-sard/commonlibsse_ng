use crate::re::BGSLocalizedStringDL::BGSLocalizedStringDL;
use crate::re::BSString::BSString;
use crate::re::BaseFormComponent::{BaseFormComponent, BaseFormComponentVtbl};
use crate::re::TESForm::TESForm;
use crate::re::offsets_rtti::RTTI_TESDescription;
use crate::re::offsets_vtable::VTABLE_TESDescription;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct TESDescription {
    pub __base: BaseFormComponent,
    pub fileOffset: u32,                       // 0x08
    pub descriptionText: BGSLocalizedStringDL, // 0x0C
}
const _: () = assert!(core::mem::size_of::<TESDescription>() == 0x10);

impl TESDescription {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_TESDescription;

    /// Address & offset of the virtual function table.
    ///
    /// The number of tables is the same as the number of classes with inherited virtual functions.
    pub const VTABLE: [VariantID; 1] = VTABLE_TESDescription;
}

pub struct TESDescriptionVtbl {
    pub __base: BaseFormComponentVtbl,
    /// - default file_type = `"CSED"`: 1129530692(0x43534544)
    pub GetDescription:
        fn(this: &mut TESDescription, out: &mut BSString, parent: *mut TESForm, file_type: u32),
}

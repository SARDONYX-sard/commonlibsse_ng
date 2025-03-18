use crate::re::BSCoreTypes::FormID;
use crate::re::BSTArray::BSStaticArray;
use crate::re::BaseFormComponent::BaseFormComponent;
use crate::re::TESFile::TESFile;
use crate::re::offsets_rtti::RTTI_TESForm;
use crate::re::offsets_vtable::VTABLE_TESForm;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
struct TESFileArray {
    _base: BSStaticArray<TESFile>,
}
const_assert_eq!(core::mem::size_of::<TESFileArray>(), 0x10);

#[repr(C)]
#[derive(Debug)]
pub struct TESFileContainer {
    array: *mut TESFileArray,
}
const_assert_eq!(core::mem::size_of::<TESFileContainer>(), 0x8);

#[repr(C)]
#[derive(Debug)]
pub struct TESForm {
    pub _base: BaseFormComponent,
    pub sourceFiles: TESFileContainer,
    pub formFlags: u32,
    pub formID: FormID,
    pub inGameFormFlags: u16,
    pub formType: u8,
    pub pad1B: u8,
    pub pad1C: u32,
}

impl TESForm {
    pub const RTTI: VariantID = RTTI_TESForm;
    pub const VTABLE: [VariantID; 1] = VTABLE_TESForm;
}

use crate::re::BGSKeyword::BGSKeyword;
use crate::re::BaseFormComponent::BaseFormComponent;
use crate::re::BaseFormComponent::BaseFormComponentVtbl;
use crate::re::offsets_rtti::RTTI_BGSKeywordForm;
use crate::re::offsets_vtable::VTABLE_BGSKeywordForm;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct BGSKeywordForm {
    pub __base: BaseFormComponent,      // 0x00
    pub keywords: *mut *mut BGSKeyword, // 0x08
    pub numKeywords: u32,               // 0x10
    pub pad14: u32,                     // 0x14
}
const _: () = assert!(core::mem::size_of::<BGSKeywordForm>() == 0x18);

impl BGSKeywordForm {
    pub const RTTI: VariantID = RTTI_BGSKeywordForm;
    pub const VTABLE: [VariantID; 1] = VTABLE_BGSKeywordForm;
}

#[repr(C)]
pub struct BGSKeywordFormVtbl {
    pub base: BaseFormComponentVtbl,
    pub HasKeyword: extern "C" fn(this: *const BGSKeywordForm, keyword: *const BGSKeyword) -> bool, // 0x04
    pub GetDefaultKeyword: extern "C" fn(this: *const BGSKeywordForm) -> *mut BGSKeyword, // 0x05
}

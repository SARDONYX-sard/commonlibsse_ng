use crate::re::BSFixedString::BSFixedString;
use crate::re::FormTypes::FormType;
use crate::re::TESFile::TESFile;
use crate::re::TESForm::{TESForm, TESFormVtbl};
use crate::re::offsets_rtti::RTTI_BGSKeyword;
use crate::re::offsets_vtable::VTABLE_BGSKeyword;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct BGSKeyword {
    pub __base: TESForm,             // 0x00
    pub formEditorID: BSFixedString, // 0x20
}
const _: () = assert!(core::mem::size_of::<BGSKeyword>() == 0x28);

impl BGSKeyword {
    pub const RTTI: VariantID = RTTI_BGSKeyword;
    pub const VTABLE: [VariantID; 1] = VTABLE_BGSKeyword;
    pub const FORM_TYPE: FormType = FormType::Keyword;
}

#[repr(C)]
pub struct BGSKeywordVtbl {
    pub __base: TESFormVtbl,
    pub Load: extern "C" fn(this: *mut BGSKeyword, mod_: *mut TESFile) -> bool, // 0x06
    pub GetFormEditorID: extern "C" fn(this: *const BGSKeyword) -> *const u8,   // 0x32
    pub SetFormEditorID: extern "C" fn(this: *mut BGSKeyword, str_: *const u8) -> bool, // 0x33
}

#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RecordFlags {
    Deleted = 1 << 5,
    Ignored = 1 << 12,
}

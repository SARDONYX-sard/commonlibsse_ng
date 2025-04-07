use crate::re::FormTypes::FormType;
use crate::re::TESForm::TESForm;
use crate::re::TESFullName::TESFullName;
use crate::re::offsets_rtti::RTTI_BGSSoundCategory;
use crate::re::offsets_vtable::VTABLE_BGSSoundCategory;
use crate::rel::id::VariantID;

use super::BSISoundCategory::BSISoundCategory;

#[repr(C)]
#[derive(Debug)]
pub struct BGSSoundCategory {
    pub __base: TESForm,                       // 0x00
    pub __base1: TESFullName,                  // 0x20
    pub __base2: BSISoundCategory,             // 0x30
    pub flags: BGSSoundCategoryFlag,           // 0x38
    pub unk3C: u32,                            // 0x3C
    pub parentCategory: *mut BGSSoundCategory, // 0x40
    pub unk48: u16,                            // 0x48
    pub attenuation: u16,                      // 0x4A
    pub staticMult: u16,                       // 0x4C
    pub defaultMenuValue: u16,                 // 0x4E
    pub volumeMult: f32,                       // 0x50
    pub frequencyMult: f32,                    // 0x54
}
const _: () = assert!(core::mem::size_of::<BGSSoundCategory>() == 0x58);

impl BGSSoundCategory {
    pub const RTTI: VariantID = RTTI_BGSSoundCategory;
    pub const VTABLE: [VariantID; 3] = VTABLE_BGSSoundCategory;
    pub const FORM_TYPE: FormType = FormType::SoundCategory;
}

#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum BGSSoundCategoryFlag {
    #[default]
    None = 0,
    MuteWhenSubmerged = 1 << 0,
    ShouldAppearOnMenu = 1 << 1,
}

#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RecordFlags {
    Deleted = 1 << 5,
    Ignored = 1 << 12,
}

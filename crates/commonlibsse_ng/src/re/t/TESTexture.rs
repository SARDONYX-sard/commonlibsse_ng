use crate::re::BSFixedString::BSFixedString;
use crate::re::BSString::BSString;
use crate::re::BaseFormComponent::{BaseFormComponent, BaseFormComponentVtbl};
use crate::re::offsets_rtti::RTTI_TESTexture;
use crate::re::offsets_vtable::VTABLE_TESTexture;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct TESTexture {
    pub __base: BaseFormComponent,  // 00
    pub textureName: BSFixedString, // 08 - ICON
}
const _: () = assert!(core::mem::size_of::<TESTexture>() == 0x10);

impl TESTexture {
    pub const RTTI: VariantID = RTTI_TESTexture;
    pub const VTABLE: [VariantID; 1] = VTABLE_TESTexture;
}

#[repr(C)]
pub struct TESTextureVtbl {
    pub __base: BaseFormComponentVtbl,
    pub GetMaxAllowedSize: extern "C" fn(this: *const TESTexture) -> u32, // 0x04
    pub GetAsNormalFile: extern "C" fn(this: *const TESTexture, out: *mut BSString) -> *const u8, // 0x05
    pub GetDefaultPath: extern "C" fn(this: *const TESTexture) -> *const u8, // 0x06
}

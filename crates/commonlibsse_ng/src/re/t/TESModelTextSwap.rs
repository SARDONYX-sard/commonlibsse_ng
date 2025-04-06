use crate::re::BSFixedString::BSFixedString;
use crate::re::TESModel::{TESModel, TESModelVtbl};
use crate::re::b::BGSTextureSet::BGSTextureSet;
use crate::re::offsets_rtti::RTTI_TESModelTextureSwap;
use crate::re::offsets_vtable::VTABLE_TESModelTextureSwap;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct TESModelTextureSwap {
    pub __base: TESModel,                         // 0x0
    pub alternateTextures: *mut AlternateTexture, // 0x28
    pub numAlternateTextures: u32,                // 0x30
    pub pad34: u32,                               // 0x34
}
const _: () = assert!(std::mem::size_of::<TESModelTextureSwap>() == 0x38);

#[repr(C)]
#[derive(Debug)]
pub struct AlternateTexture {
    pub textureSet: *mut BGSTextureSet, // 0x00
    pub index3d: u32,                   // 0x08
    pub unk0C: u32,                     // 0x0C
    pub name3d: BSFixedString,          // 0x10
}
const _: () = assert!(std::mem::size_of::<AlternateTexture>() == 0x18);

impl TESModelTextureSwap {
    pub const RTTI: VariantID = RTTI_TESModelTextureSwap;
    pub const VTABLE: [VariantID; 1] = VTABLE_TESModelTextureSwap;
}

#[repr(C)]
pub struct TESModelTextureSwapVtbl {
    pub __base: TESModelVtbl,
}

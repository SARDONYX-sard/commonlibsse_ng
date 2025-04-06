use core::ffi::c_char;

use crate::re::BSFixedString::BSFixedString;
use crate::re::BSResource;
use crate::re::BaseFormComponent::{BaseFormComponent, BaseFormComponentVtbl};
use crate::re::TESModelTextSwap::TESModelTextureSwap;
use crate::re::offsets_rtti::RTTI_TESModel;
use crate::re::offsets_vtable::VTABLE_TESModel;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct TESModel {
    pub __base: BaseFormComponent,
    pub model: BSFixedString,        // 0x08
    pub textures: *mut BSResource::ID, // 0x10
    pub addons: *mut u32,            // 0x18
    pub numTextures: u16,            // 0x20
    pub numAddons: u16,              // 0x22
    pub pad24: u32,                  // 0x24
}
const _: () = assert!(std::mem::size_of::<TESModel>() == 0x28);

impl TESModel {
    pub const RTTI: VariantID = RTTI_TESModel;
    pub const VTABLE: [VariantID; 1] = VTABLE_TESModel;
}

#[repr(C)]
pub struct TESModelVtbl {
    pub __base: BaseFormComponentVtbl,
    pub GetModel: extern "C" fn(this: *const TESModel) -> *const c_char, // 04
    pub SetModel: extern "C" fn(this: *mut TESModel, model: *const c_char), // 05
    pub GetAsModelTextureSwap: extern "C" fn(this: *mut TESModel) -> *mut TESModelTextureSwap, // 06
}

use core::ffi::c_char;

use crate::re::NiObject::{NiObject, NiObjectVtbl};
use crate::re::NiSourceTexture::NiSourceTexture;
use crate::re::offsets_ni_rtti::NiRTTI_BSTextureSet;
use crate::re::offsets_rtti::RTTI_BSTextureSet;
use crate::re::offsets_vtable::VTABLE_BSTextureSet;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct BSTextureSet {
    pub __base: NiObject,
}
const _: () = assert!(core::mem::size_of::<BSTextureSet>() == 0x10);

impl BSTextureSet {
    pub const RTTI: VariantID = RTTI_BSTextureSet;
    pub const NI_RTTI: VariantID = NiRTTI_BSTextureSet;
    pub const VTABLE: [VariantID; 1] = VTABLE_BSTextureSet;
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Texture {
    Diffuse = 0,
    Normal = 1,
    Gloss = 1,
    EnvironmentMask = 2,
    SubsurfaceTint = 2,
    GlowMap = 3,
    DetailMap = 3,
    Height = 4,
    Environment = 5,
    Multilayer = 6,
    BacklightMask = 7,
    Specular = 7,
    Unused08 = 8,
    UsedTotal = 8,
}

impl Texture {
    /// enum length.
    pub const TOTAL: usize = 9;
}

#[repr(C)]
pub struct BSTextureSetVtbl {
    pub __base: NiObjectVtbl,
    pub GetTexturePath:
        unsafe extern "C" fn(this: *const BSTextureSet, texture: Texture) -> *const c_char, // 0x25
    pub SetTexture: unsafe extern "C" fn(
        this: *mut BSTextureSet,
        texture: Texture,
        src_texture: *mut NiSourceTexture,
    ), // 0x26
    pub SetTexturePath:
        unsafe extern "C" fn(this: *mut BSTextureSet, texture: Texture, path: *const c_char), // 0x27
}

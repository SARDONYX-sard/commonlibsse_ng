use crate::re::BSTextureSet::BSTextureSet;
use crate::re::DecalData::DecalData;
use crate::re::BSResource;
use crate::re::TESBoundObject::TESBoundObject;
use crate::re::TESTexture::TESTexture;

#[repr(C)]
pub struct BGSTextureSet {
    pub __base: TESBoundObject,
    pub __base1: BSTextureSet,
    /// - 8: BSTextureSet::Texture::UsedTotal
    pub textures: [TESTexture; 8], // 0x040
    pub decalData: *mut DecalData, // 0x0C0
    pub flags: Flag,               // 0x0C8
    pub pad0CA: u16,               // 0x0CA
    /// - 8: BSTextureSet::Texture::UsedTotal
    pub textureFileIDs: [BSResource::ID; 8], // 0x0CC
    pub pad12C: u32,               // 0x12C
}
const _: () = assert!(std::mem::size_of::<BGSTextureSet>() == 0x130);

#[commonlibsse_ng_derive_internal::to_bitflags]
#[repr(u16)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Flag {
    #[default]
    None = 0,
    NoSpecularMap = 1 << 0,
    FacegenTextures = 1 << 1,
    HasModelSpaceNormalMap = 1 << 2,
}

#[commonlibsse_ng_derive_internal::ffi_enum]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RecordFlag {
    Deleted = 1 << 5,
    Ignored = 1 << 12,
}

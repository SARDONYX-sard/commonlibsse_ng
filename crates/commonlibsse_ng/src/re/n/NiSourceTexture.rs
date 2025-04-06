use crate::re::NiTexture::{NiTexture, NiTextureVtbl};
use crate::re::offsets_ni_rtti::NiRTTI_NiSourceTexture;
use crate::re::offsets_rtti::RTTI_NiSourceTexture;
use crate::re::offsets_vtable::VTABLE_NiSourceTexture;
use crate::rel::id::VariantID;

use windows::Win32::Graphics::Direct3D11::{ID3D11ShaderResourceView, ID3D11Texture2D};

#[repr(C)]
#[derive(Debug)]
pub struct BSGraphicsTexture {
    pub texture: *mut ID3D11Texture2D,               // 0x00
    pub unk08: u64,                                  // 0x08
    pub resourceView: *mut ID3D11ShaderResourceView, // 0x10
}
const _: () = assert!(core::mem::size_of::<BSGraphicsTexture>() == 0x18);

#[repr(C)]
#[derive(Debug)]
pub struct NiSourceTexture {
    pub __base: NiTexture,                       // 0x00 - 40
    pub unk40: *mut BSResourceStream,            // 0x40
    pub rendererTexture: *mut BSGraphicsTexture, // 0x48
    pub flags: u8,                               // 0x50
    pub pad51: u8,                               // 0x51
    pub pad52: u16,                              // 0x52
    pub pad54: u32,                              // 0x54
}
const _: () = assert!(core::mem::size_of::<NiSourceTexture>() == 0x58);

impl NiSourceTexture {
    pub const RTTI: VariantID = RTTI_NiSourceTexture;
    pub const NI_RTTI: VariantID = NiRTTI_NiSourceTexture;
    pub const VTABLE: [VariantID; 1] = VTABLE_NiSourceTexture;
}

#[repr(C)]
pub struct NiSourceTextureVtbl {
    pub __base: NiTextureVtbl,

    // virtual-s from 0x25 to 0x2A
    pub Unk25: extern "C" fn(this: *const NiSourceTexture) -> u32, // 25
    pub Unk26: extern "C" fn(this: *const NiSourceTexture) -> u32, // 26
    pub Unk27: extern "C" fn(this: *const NiSourceTexture),        // 27
    pub Unk28: extern "C" fn(this: *const NiSourceTexture),        // 28
    pub Unk29: extern "C" fn(this: *const NiSourceTexture),        // 29
    pub Unk2A: extern "C" fn(this: *const NiSourceTexture),        // 2A
}

// FFI declarations

#[repr(C)]
pub struct BSResourceStream;

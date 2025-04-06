use crate::re::BSFixedString::BSFixedString;
use crate::re::NiObject::{NiObject, NiObjectVtbl};
use crate::re::offsets_ni_rtti::NiRTTI_NiTexture;
use crate::re::offsets_rtti::RTTI_NiTexture;
use crate::re::offsets_vtable::VTABLE_NiTexture;
use crate::rel::id::VariantID;

use windows::Win32::Graphics::Direct3D11::{ID3D11ShaderResourceView, ID3D11Texture2D};

#[repr(C)]
#[derive(Debug)]
pub struct NiTexture {
    pub __base: NiObject,         // 0x00
    pub formatPrefs: FormatPrefs, // 0x10
    pub name: BSFixedString,      // 0x20
    pub unk28: u32,               // 0x28
    pub unk2C: u32,               // 0x2C
    pub prev: *mut NiTexture,     // 0x30
    pub next: *mut NiTexture,     // 0x38
}
const _: () = assert!(core::mem::size_of::<NiTexture>() == 0x40);

impl NiTexture {
    pub const RTTI: VariantID = RTTI_NiTexture;
    pub const NI_RTTI: VariantID = NiRTTI_NiTexture;
    pub const VTABLE: [VariantID; 1] = VTABLE_NiTexture;
}

#[repr(C)]
#[derive(Debug)]
pub struct FormatPrefs {
    pub pixelLayout: PixelLayout, // 00
    pub alphaFormat: AlphaFormat, // 04
    pub mipMapped: MipFlag,       // 08
    pub pad0C: u32,               // 0C
}
const _: () = assert!(core::mem::size_of::<FormatPrefs>() == 0x10);

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum PixelLayout {
    Palettized8 = 0,
    HighColor16,
    TrueColor32,
    Compressed,
    Bumpmap,
    Palettized4,
    Default,
    SingleColor8,
    SingleColor16,
    SingleColor32,
    DoubleColor32,
    DoubleColor64,
    FloatColor32,
    FloatColor64,
    FloatColor128,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum AlphaFormat {
    None = 0,
    Binary,
    Smooth,
    Default,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum MipFlag {
    No = 0,
    Yes = 1,
    Default = 2,
}

#[repr(C)]
#[derive(Debug)]
pub struct RendererData {
    pub texture: *mut ID3D11Texture2D,               // 00
    pub unk08: u64,                                  // 08
    pub resourceView: *mut ID3D11ShaderResourceView, // 10
    pub width: u16,                                  // 18
    pub height: u16,                                 // 1A
    pub unk1C: u8,                                   // 1C
    pub unk1D: u8,                                   // 1D
    pub unk1E: u16,                                  // 1E
    pub unk20: u32,                                  // 20
    pub unk24: u32,                                  // 24
}
const _: () = assert!(core::mem::size_of::<RendererData>() == 0x28);

#[repr(C)]
pub struct NiTextureVtbl {
    pub __base: NiObjectVtbl,

    pub Unk25: extern "C" fn(this: *const NiTexture) -> u32, // 25
    pub Unk26: extern "C" fn(this: *const NiTexture) -> u32, // 26
    pub Unk27: extern "C" fn(this: *const NiTexture) -> *const u8, // 27 - returns C-string?
    pub Unk28: extern "C" fn(this: *const NiTexture),        // 28
    pub Unk29: extern "C" fn(this: *const NiTexture) -> u32, // 29
    pub Unk2A: extern "C" fn(this: *const NiTexture) -> u32, // 2A
}

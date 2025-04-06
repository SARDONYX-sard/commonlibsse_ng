use crate::re::Color::Color;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flag {
    None = 0,
    Parallax = 1 << 0,
    AlphaBlending = 1 << 1,
    AlphaTesting = 1 << 2,
    NoSubtextures = 1 << 3,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct DECAL_DATA_DATA {
    pub decalMinWidth: f32,  // 0x00
    pub decalMaxWidth: f32,  // 0x04
    pub decalMinHeight: f32, // 0x08
    pub decalMaxHeight: f32, // 0x0C
    pub depth: f32,          // 0x10
    pub shininess: f32,      // 0x14
    pub parallaxScale: f32,  // 0x18
    pub parallaxPasses: i8,  // 0x1C
    pub flags: u8,           // 0x1D (Flag as u8, or use newtype wrapper)
    pub pad1E: u16,          // 0x1E
    pub color: Color,        // 0x20
}
const _: () = assert!(std::mem::size_of::<DECAL_DATA_DATA>() == 0x24);

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct DecalData {
    pub data: DECAL_DATA_DATA, // 00
}
const _: () = assert!(std::mem::size_of::<DecalData>() == 0x24);

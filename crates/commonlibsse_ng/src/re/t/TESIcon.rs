use crate::re::TESTexture::TESTexture;
use crate::re::TESTexture::TESTextureVtbl;
use crate::re::offsets_rtti::RTTI_TESIcon;
use crate::re::offsets_vtable::VTABLE_TESIcon;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct TESIcon {
    pub __base: TESTexture, // 0x00
}
const _: () = assert!(core::mem::size_of::<TESIcon>() == 0x10);

impl TESIcon {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_TESIcon;

    /// Address & offset of the virtual function table.
    ///
    /// The number of tables is the same as the number of classes with inherited virtual functions.
    pub const VTABLE: [VariantID; 1] = VTABLE_TESIcon;
}

#[repr(C)]
pub struct TESIconVtbl {
    pub __base: TESTextureVtbl,
    pub GetDefaultPath: fn(this: &TESIcon),
}

use crate::re::offsets_rtti::RTTI_BGSLoadFormBuffer;
use crate::re::offsets_vtable::VTABLE_BGSLoadFormBuffer;
use crate::rel::id::VariantID;

use super::BGSLoadFormData::BGSLoadFormData;
use super::BGSLoadGameBuffer::{BGSLoadGameBuffer, BGSLoadGameBufferVtbl};

#[repr(C)]
#[derive(Debug)]
pub struct BGSLoadFormBuffer {
    pub __base: BGSLoadGameBuffer, // 0x00
    pub __base1: BGSLoadFormData,  // 0x28
}
const _: () = assert!(std::mem::size_of::<BGSLoadFormBuffer>() == 0x50);

impl BGSLoadFormBuffer {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_BGSLoadFormBuffer;

    /// Address & offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_BGSLoadFormBuffer;
}

#[repr(C)]
#[derive(Debug)]
pub struct BGSLoadFormBufferVtbl {
    pub __base: BGSLoadGameBufferVtbl, // 0x00
}
const _: () = {
    const VFUNC_COUNT: usize = 0x2;

    const EXPECTED_SIZE: usize = VFUNC_COUNT * core::mem::size_of::<usize>();
    assert!(core::mem::size_of::<BGSLoadFormBufferVtbl>() == EXPECTED_SIZE);
};

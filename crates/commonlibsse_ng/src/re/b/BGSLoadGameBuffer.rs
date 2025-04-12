use crate::re::offsets_rtti::RTTI_BGSLoadGameBuffer;
use crate::re::offsets_vtable::VTABLE_BGSLoadGameBuffer;
use crate::rel::id::VariantID;
use core::ffi::c_void;

#[repr(C)]
#[derive(Debug)]
pub struct BGSLoadGameBuffer {
    pub vtable: *const BGSLoadGameBufferVtbl, // 0x00
    pub buffer: *mut c_void,                  // 0x08
    pub unk10: u64,                           // 0x10
    pub unk18: u32,                           // 0x18
    pub unk1C: u32,                           // 0x1C
    pub unk20: u32,                           // 0x20
    pub bufferPosition: u32,                  // 0x24
}
const _: () = assert!(std::mem::size_of::<BGSLoadGameBuffer>() == 0x28);

impl BGSLoadGameBuffer {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_BGSLoadGameBuffer;

    /// Address & offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_BGSLoadGameBuffer;

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 35112, ae_id = 36005)]
    #[inline]
    pub fn load_data_endian(&mut self, data: *mut c_void, offset: u32, size: u32) {}
}

#[repr(C)]
#[derive(Debug)]
pub struct BGSLoadGameBufferVtbl {
    /// C++ Destructor `~BGSLoadGameBuffer`
    pub CxxDrop: fn(this: *mut BGSLoadGameBuffer), // 0x00
    pub GetVersion: fn(this: *mut BGSLoadGameBuffer, arg1: c_void), // 0x01
}
const _: () = {
    const VFUNC_COUNT: usize = 0x2;

    const EXPECTED_SIZE: usize = VFUNC_COUNT * core::mem::size_of::<usize>();
    assert!(core::mem::size_of::<BGSLoadGameBufferVtbl>() == EXPECTED_SIZE);
};

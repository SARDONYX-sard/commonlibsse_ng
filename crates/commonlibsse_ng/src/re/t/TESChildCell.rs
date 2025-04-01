use crate::re::offsets_rtti::RTTI_TESChildCell;
use crate::re::offsets_vtable::VTABLE_TESChildCell;
use crate::rel::id::VariantID;
use core::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TESChildCell {
    pub vtable: *const TESChildFullVtbl, // 00
}
const _: () = assert!(std::mem::size_of::<TESChildCell>() == 0x8);

impl TESChildCell {
    pub const RTTI: VariantID = RTTI_TESChildCell;
    pub const VTABLE: [VariantID; 1] = VTABLE_TESChildCell;
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TESChildFullVtbl {
    pub CxxDrop: fn(this: *mut c_void),           // 0x00
    pub GetSaveParentCell: fn(this: *mut c_void), // 0x01
}
const _: () = {
    const VFUNC_COUNT: usize = 0x2;

    const EXPECTED_SIZE: usize = VFUNC_COUNT * core::mem::size_of::<usize>();
    assert!(core::mem::size_of::<TESChildFullVtbl>() == EXPECTED_SIZE);
};

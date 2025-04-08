use crate::re::offsets_rtti::RTTI_BSISoundCategory;
use crate::re::offsets_vtable::VTABLE_BSISoundCategory;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct BSISoundCategory {
    vtable: *const BSISoundCategoryVtbl,
}

impl BSISoundCategory {
    pub const RTTI: VariantID = RTTI_BSISoundCategory;
    pub const VTABLE: [VariantID; 1] = VTABLE_BSISoundCategory;
}

#[repr(C)]
pub struct BSISoundCategoryVtbl {
    /// C++ Destructor. `~BSISoundCategory`
    pub CxxDrop: extern "C" fn(this: *mut BSISoundCategory), // 0x00

    pub Matches:
        extern "C" fn(this: *const BSISoundCategory, category: *const BSISoundCategory) -> bool, // 0x01
    pub GetCategoryVolume: extern "C" fn(this: *const BSISoundCategory) -> f32, // 0x02
    pub SetCategoryVolume: extern "C" fn(this: *mut BSISoundCategory, value: f32), // 0x03
    pub GetCategoryFrequency: extern "C" fn(this: *const BSISoundCategory) -> f32, // 0x04
    pub SetCategoryFrequency: extern "C" fn(this: *mut BSISoundCategory, value: f32), // 0x05
    pub GetCategoryAttenuation: extern "C" fn(this: *const BSISoundCategory) -> u16, // 0x06
    pub SetCategoryAttenuation: extern "C" fn(this: *mut BSISoundCategory, value: u16), // 0x07
    pub Unk_08: extern "C" fn(this: *mut BSISoundCategory),                     // 0x08
    pub Unk_09: extern "C" fn(this: *mut BSISoundCategory),                     // 0x09
    pub Unk_0a: extern "C" fn(this: *mut BSISoundCategory),                     // 0x0A
    pub Unk_0b: extern "C" fn(this: *mut BSISoundCategory),                     // 0x0B
}
const _: () = {
    const ACTUAL_SIZE: usize = core::mem::size_of::<BSISoundCategoryVtbl>();
    const EXPECTED_SIZE: usize = (0x0B + 1) * core::mem::size_of::<usize>();
    assert!(ACTUAL_SIZE == EXPECTED_SIZE);
};

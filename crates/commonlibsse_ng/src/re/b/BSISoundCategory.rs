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
    pub Matches: extern "C" fn(this: *const BSISoundCategory, category: *const BSISoundCategory) -> bool,
    pub GetCategoryVolume: extern "C" fn(this: *const BSISoundCategory) -> f32,
    pub SetCategoryVolume: extern "C" fn(this: *const BSISoundCategory, value: f32),
    pub GetCategoryFrequency: extern "C" fn(this: *const BSISoundCategory) -> f32,
    pub SetCategoryFrequency: extern "C" fn(this: *const BSISoundCategory, value: f32),
    pub GetCategoryAttenuation: extern "C" fn(this: *const BSISoundCategory) -> u16,
    pub SetCategoryAttenuation: extern "C" fn(this: *const BSISoundCategory, value: u16),
    pub Unk_08: extern "C" fn(this: *const BSISoundCategory),
    pub Unk_09: extern "C" fn(this: *const BSISoundCategory),
    pub Unk_0a: extern "C" fn(this: *const BSISoundCategory),
    pub Unk_0b: extern "C" fn(this: *const BSISoundCategory),
}

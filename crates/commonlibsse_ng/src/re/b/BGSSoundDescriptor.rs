use crate::re::BGSSoundCategory::BGSSoundCategory;
use crate::re::BSCoreTypes::FormID;
use crate::re::BSISoundDescriptor::BSISoundDescriptor;
use crate::re::BSISoundDescriptor::BSISoundDescriptorVtbl;
use crate::re::TESFile::TESFile;
use crate::re::TESForm::TESForm;
use crate::re::offsets_rtti::RTTI_BGSSoundDescriptor;
use crate::re::offsets_vtable::VTABLE_BGSSoundDescriptor;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct BGSSoundDescriptor {
    pub __base: BSISoundDescriptor,      // 0x00
    pub category: *mut BGSSoundCategory, // 0x08
    pub alternateSoundFormId: FormID,    // 0x10
    pub pad14: u32,                      // 0x14
}
const _: () = assert!(core::mem::size_of::<BGSSoundDescriptor>() == 0x18);

impl BGSSoundDescriptor {
    pub const RTTI: VariantID = RTTI_BGSSoundDescriptor;
    pub const VTABLE: [VariantID; 1] = VTABLE_BGSSoundDescriptor;
}

#[repr(C)]
pub struct BGSSoundDescriptorVtbl {
    pub __base: BSISoundDescriptorVtbl, // 0x00
    pub InitSound: extern "C" fn(this: *mut BGSSoundDescriptor, src: *mut TESForm), // 0x03
    pub LoadSound: extern "C" fn(this: *mut BGSSoundDescriptor, mod_: *mut TESFile) -> bool, // 0x04
    pub GetType: extern "C" fn(this: *const BGSSoundDescriptor) -> u32, // 0x05
    pub Unk_06: extern "C" fn(this: *mut BGSSoundDescriptor), // 0x06
}

use crate::re::BGSSoundDescriptor::BGSSoundDescriptor;
use crate::re::BSISoundDescriptor::{BSISoundDescriptor, BSISoundDescriptorVtbl};
use crate::re::FormTypes::FormType;
use crate::re::TESForm::{TESForm, TESFormVtbl};
use crate::re::offsets_rtti::RTTI_BGSSoundDescriptorForm;
use crate::re::offsets_vtable::VTABLE_BGSSoundDescriptorForm;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct BGSSoundDescriptorForm {
    pub __base: TESForm,                          // 0x00
    pub __base1: BSISoundDescriptor,              // 0x20
    pub soundDescriptor: *mut BGSSoundDescriptor, // 0x28
}
const _: () = assert!(core::mem::size_of::<BGSSoundDescriptorForm>() == 0x30);

impl BGSSoundDescriptorForm {
    pub const RTTI: VariantID = RTTI_BGSSoundDescriptorForm;
    pub const VTABLE: [VariantID; 2] = VTABLE_BGSSoundDescriptorForm;
    pub const FORM_TYPE: FormType = FormType::SoundRecord;
}

#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RecordFlags {
    Deleted = 1 << 5,
    Ignored = 1 << 12,
}

#[repr(C)]
pub struct BGSSoundDescriptorFormVtbl {
    pub __base: TESFormVtbl,             // 0x00
    pub __base1: BSISoundDescriptorVtbl, // 0x20
    pub GetDescriptorType: extern "C" fn(this: *const BGSSoundDescriptorForm) -> u32, // 0x3B
}

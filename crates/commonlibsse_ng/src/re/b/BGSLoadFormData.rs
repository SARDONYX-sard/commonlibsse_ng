use crate::re::BSCoreTypes::FormID;
use crate::re::TESForm::TESForm;
use crate::re::offsets_rtti::RTTI_BGSLoadFormData;
use crate::rel::id::VariantID;

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum Flags {
    SkipForm = 1 << 0,
    FormPending = 1 << 1,
    RevertOnly = 1 << 2,
    ConstructedForm = 1 << 3,
    CellChanged = 1 << 4,
    LoadingPackageFromExtraData = 1 << 5,
}

#[repr(C)]
#[derive(Debug)]
pub struct BGSLoadFormData {
    pub formID: FormID,        // 0x00
    pub size: u32,             // 0x04
    pub uncompressedSize: u32, // 0x08
    pub pad0C: u32,            // 0x0C
    pub form: *mut TESForm,    // 0x10
    pub changeFlags: u32,      // 0x18
    pub oldChangeFlags: u32,   // 0x1C
    pub flags: Flags,          // 0x20
    pub pad22: u8,             // 0x22
    pub version: u8,           // 0x23
}
const _: () = assert!(std::mem::size_of::<BGSLoadFormData>() == 0x28);

impl BGSLoadFormData {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_BGSLoadFormData;
}

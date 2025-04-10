use crate::re::FormTypes::FormType;
use crate::re::offsets_rtti::RTTI_BGSPerkEntry;
use crate::re::offsets_vtable::VTABLE_BGSPerkEntry;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BGSPerkEntry {
    pub vtable: *const BGSPerkEntryVtbl, // 0x0
    pub header: Header,                  // 0x8
}
const _: () = assert!(core::mem::size_of::<BGSPerkEntry>() == 0x10);

impl BGSPerkEntry {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_BGSPerkEntry;

    /// Address & offset of the virtual function table.
    ///
    /// The number of tables is the same as the number of classes with inherited virtual functions.
    pub const VTABLE: [VariantID; 1] = VTABLE_BGSPerkEntry;

    /// The `FormType` value for BGSPerkEntry.
    pub const FORM_TYPE: FormType = FormType::Perk;
}

#[repr(C)]
pub struct BGSPerkEntryVtbl {
    GetType: fn(this: &BGSPerkEntry),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Header {
    rank: u8,     // 0x1
    priority: u8, // 0x2
    unk2: u16,    // 0x3
    unk4: u32,    // 0x4
}
const _: () = assert!(core::mem::size_of::<Header>() == 0x8);

#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum PERK_ENTRY_TYPE {
    Quest = 0,
    Ability = 1,
    EntryPoint = 2,
}

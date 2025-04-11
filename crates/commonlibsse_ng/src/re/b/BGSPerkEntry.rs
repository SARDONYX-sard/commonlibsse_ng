use core::ffi::c_void;

use crate::re::Actor::Actor;
use crate::re::BGSEntryPoint::EntryPoint;
use crate::re::FormTypes::FormType;
use crate::re::TESFile::TESFile;
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

    #[inline]
    pub const fn get_rank(&self) -> u8 {
        self.header.rank
    }

    #[inline]
    pub const fn get_priority(&self) -> u8 {
        self.header.priority
    }
}

#[repr(C)]
pub struct BGSPerkEntryVtbl {
    CheckConditionFilters: fn(this: &mut BGSPerkEntry, num_args: u32, args: *mut c_void) -> bool, // 00 - { return false; }
    GetFunction: fn(this: &mut BGSPerkEntry) -> EntryPoint, // 01 - { return 0; }
    GetFunctionData: fn(this: &BGSPerkEntry) -> *mut c_void, // 02 - { return 0; }
    CxxDrop: fn(this: &mut BGSPerkEntry),
    GetType: fn(this: &BGSPerkEntry) -> PERK_ENTRY_TYPE_CEnum, // 0x04
    ClearData: fn(this: &mut BGSPerkEntry),                    // 0x05 - { return; }
    InitItem: fn(this: &mut BGSPerkEntry, owner: *mut TESFile), // 0x06 - { return; }
    Load: fn(this: &mut BGSPerkEntry, file: *mut TESFile) -> bool, // 0x07 - { return true; }
    SetParent: fn(this: &mut BGSPerkEntry),                    // 0x08 - { return; }
    GetID: fn(this: &BGSPerkEntry) -> u16,                     // 0x09 - { return 0xFFFF; }
    ApplyPerkEntry: fn(this: &mut BGSPerkEntry, actor: *mut Actor), // 0x0A
    RemovePerkEntry: fn(this: &mut BGSPerkEntry, actor: *mut Actor), // 0x0B
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

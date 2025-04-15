use crate::re::BSCoreTypes::RefHandle;
use crate::re::InventoryEntryData::InventoryEntryData;
use crate::re::offsets_rtti::RTTI_StandardItemData;
use crate::re::offsets_vtable::VTABLE_StandardItemData;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct StandardItemData {
    pub vtable: *const StandardItemDataVtbl,
    pub objDesc: *mut InventoryEntryData,
    pub owner: RefHandle,
    pub pad14: u32,
}

const _: () = {
    assert!(core::mem::offset_of!(StandardItemData, vtable) == 0x0);
    assert!(core::mem::offset_of!(StandardItemData, objDesc) == 0x8);
    assert!(core::mem::offset_of!(StandardItemData, owner) == 0x10);
    assert!(core::mem::offset_of!(StandardItemData, pad14) == 0x14);
    assert!(core::mem::size_of::<StandardItemData>() == 0x18);
};

impl StandardItemData {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_StandardItemData;

    /// Address & offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_StandardItemData;
}

/// The virtual function table for `StandardItemData`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
pub struct StandardItemDataVtbl {
    GetName: fn(this: &mut StandardItemData) -> *const char, // 01 - { return objDesc->GenerateName(); }
    GetCount: fn(this: &mut StandardItemData) -> u32,        // 02 - { return objDesc->GetCount(); }
    GetEquipState: fn(this: &mut StandardItemData) -> u32,   // 03
    GetFilterFlag: fn(this: &mut StandardItemData) -> u32,   // 04
    GetFavorite: fn(this: &mut StandardItemData) -> u32,     // 05
    GetEnabled: fn(this: &mut StandardItemData) -> bool,     // 06 - { return true; }
}

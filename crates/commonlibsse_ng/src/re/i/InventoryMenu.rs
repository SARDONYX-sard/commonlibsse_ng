use core::ffi::c_void;

use crate::re::BSTArray::BSTArray;
use crate::re::BottomBar::BottomBar;
use crate::re::GFxValue;
use crate::re::IMenu::{IMenu, IMenuVtbl};
use crate::re::ItemCard::ItemCard;
use crate::re::ItemList::ItemList;
use crate::re::UI::HasMenuName;
use crate::re::offsets_rtti::RTTI_InventoryMenu;
use crate::rel::id::VariantID;
use crate::rel::relocation::{RelocationError, relocate_member, relocate_member_mut};

/// - menuDepth: `0`
/// - flags: `PausesGame | DisablePauseMenu | UpdateUsesCursor | InventoryItemMenu | CustomRendering`
/// - context: `None`
#[repr(C)]
#[derive(Debug)]
pub struct InventoryMenu {
    pub __base: IMenu, // 0x000
}
const _: () = assert!(core::mem::size_of::<InventoryMenu>() == 0x30);

impl InventoryMenu {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_InventoryMenu;

    /// Gets fields whose offset is determined at runtime.
    ///
    /// # Errors
    /// This function may return an error if the module's runtime is not available or if any error occurs while fetching the runtime state.
    /// Specifically, it calls `ModuleState::map_active`, which could result in an error.
    #[inline]
    pub fn get_runtime_data(&self) -> Result<&RUNTIME_DATA, RelocationError> {
        relocate_member(self, 0x30, 0x40)
    }

    /// Gets mutable fields whose offset is determined at runtime.
    ///
    /// # Errors
    /// This function may return an error if the module's runtime is not available or if any error occurs while fetching the runtime state.
    /// Specifically, it calls `ModuleState::map_active_mut`, which could result in an error.
    #[inline]
    pub fn get_runtime_data_mut(&mut self) -> Result<&mut RUNTIME_DATA, RelocationError> {
        relocate_member_mut(self, 0x30, 0x40)
    }
}

impl HasMenuName for InventoryMenu {
    const MENU_NAME: &'static core::ffi::CStr = c"InventoryMenu";
}

#[repr(C)]
pub struct InventoryMenuVtbl {
    pub __base: IMenuVtbl,
}

#[derive(Debug)]
#[repr(C)]
pub struct RUNTIME_DATA {
    pub root: GFxValue,               // 0x00 - kDisplayObject - "_level0.Menu_mc"
    pub itemList: *mut ItemList,      // 0x18
    pub itemCard: *mut ItemCard,      // 0x20
    pub bottomBar: *mut BottomBar,    // 0x28
    pub unk60: BSTArray<*mut c_void>, // 0x30
    pub unk78: u8,                    // 0x48
    pub pad79: u8,                    // 0x49
    pub pad7A: u16,                   // 0x4A
    pub unk7C: u32,                   // 0x4C
    pub pcControlsReady: bool,        // 0x50
    pub unk81: u8,                    // 0x51
    pub pad82: u16,                   // 0x52
    pub pad84: u32,                   // 0x54
}
const _: () = assert!(core::mem::size_of::<RUNTIME_DATA>() == 0x58);

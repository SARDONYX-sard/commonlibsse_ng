//! # GameSettingCollection
//!
//! This module defines the `GameSettingCollection` struct, inheriting from `SettingCollectionMap<Setting>`.
//! It represents the game's settings collection, including methods for interacting with settings and handling.

use crate::re::Setting::Setting;
use crate::re::SettingCollectionMap::SettingCollectionMap;
use crate::re::offsets_rtti::RTTI_GameSettingCollection;
use crate::re::offsets_vtable::VTABLE_GameSettingCollection;
use crate::rel::id::VariantID;

/// Represents the game's settings collection.
///
/// Inherits from `SettingCollectionMap<Setting>`.
///
/// # Memory Layout:
/// - `__base`: Base class `SettingCollectionMap<Setting>`
/// - `handle`: Handle used for settings management
#[repr(C)]
pub struct GameSettingCollection {
    /// Base class `SettingCollectionMap<Setting>`.
    pub __base: SettingCollectionMap<Setting>,

    /// Handle used for managing settings (0x138).
    /// - Offset: `0x138`
    pub handle: u64,
}

const _: () = {
    assert!(core::mem::offset_of!(GameSettingCollection, __base) == 0x0);
    // assert!(core::mem::offset_of!(GameSettingCollection, handle) == 0x138);
    // assert!(core::mem::size_of::<GameSettingCollection>() == 0x140);
};

impl GameSettingCollection {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_GameSettingCollection;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_GameSettingCollection;

    /// Retrieves the singleton instance of `GameSettingCollection`.
    ///
    /// # Returns
    /// A reference to the singleton instance.
    #[inline]
    pub fn get_singleton() -> *mut Self {
        static mut SINGLETON: *mut GameSettingCollection = std::ptr::null_mut();
        unsafe { SINGLETON }
    }

    /// Retrieves a setting by name.
    ///
    /// # Arguments
    /// - `a_name`: The name of the setting to retrieve.
    ///
    /// # Returns
    /// - `Some(&mut Setting)` if the setting exists.
    /// - `None` if the setting is not found.
    #[inline]
    pub fn get_setting(&mut self, a_name: &str) -> Option<&mut Setting> {
        let _ = a_name;
        // self.__base.settings.get_mut(a_name)
        todo!()
    }

    /// Writes a setting.
    ///
    /// # Arguments
    /// - `a_setting`: The setting to write.
    ///
    /// # Returns
    /// - `false` (this function always returns `false`).
    #[inline]
    pub fn write_setting(&mut self, _a_setting: &Setting) -> bool {
        false
    }

    /// Reads a setting.
    ///
    /// # Arguments
    /// - `a_setting`: The setting to read.
    ///
    /// # Returns
    /// - `true` if reading was successful, `false` otherwise.
    #[inline]
    pub fn read_setting(&mut self, _a_setting: &mut Setting) -> bool {
        true
    }

    /// Opens the handle.
    ///
    /// # Arguments
    /// - `a_create`: Whether to create the handle.
    ///
    /// # Returns
    /// - `true` if the handle is valid.
    #[inline]
    pub fn open_handle(&mut self, _a_create: bool) -> bool {
        self.handle != 0
    }

    /// Closes the handle.
    ///
    /// # Returns
    /// - `true` on success.
    #[inline]
    pub fn close_handle(&mut self) -> bool {
        self.handle = 0;
        true
    }

    /// Unknown virtual function `0A`.
    #[inline]
    pub fn unk_0a(&mut self) {
        // Stub function, add behavior if needed.
    }
}

/// The virtual function table for `GameSettingCollection`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
pub struct GameSettingCollectionVtbl {
    /// Destructor function pointer.
    pub CxxDrop: fn(this: &mut GameSettingCollection),

    /// Function pointer for writing a setting.
    pub WriteSetting: fn(this: &mut GameSettingCollection, setting: &Setting) -> bool,

    /// Function pointer for reading a setting.
    pub ReadSetting: fn(this: &mut GameSettingCollection, setting: &mut Setting) -> bool,

    /// Function pointer for opening the handle.
    pub OpenHandle: fn(this: &mut GameSettingCollection, create: bool) -> bool,

    /// Function pointer for closing the handle.
    pub CloseHandle: fn(this: &mut GameSettingCollection) -> bool,

    /// Function pointer for the unknown function `0A`.
    pub Unk_0A: fn(this: &mut GameSettingCollection),
}

impl Default for GameSettingCollectionVtbl {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl GameSettingCollectionVtbl {
    /// Creates a new default virtual table with stubbed functions.
    #[inline]
    pub const fn new() -> Self {
        const fn CxxDrop(_this: &mut GameSettingCollection) {}

        const fn WriteSetting(_this: &mut GameSettingCollection, _setting: &Setting) -> bool {
            false
        }

        const fn ReadSetting(_this: &mut GameSettingCollection, _setting: &mut Setting) -> bool {
            true
        }

        const fn OpenHandle(_this: &mut GameSettingCollection, _create: bool) -> bool {
            true
        }

        const fn CloseHandle(_this: &mut GameSettingCollection) -> bool {
            true
        }

        const fn Unk_0A(_this: &mut GameSettingCollection) {}

        Self { CxxDrop, WriteSetting, ReadSetting, OpenHandle, CloseHandle, Unk_0A }
    }
}

//! # SettingCollectionMap
//!
//! This module defines the `SettingCollectionMap<T>` struct, representing a collection of settings
//! with case-insensitive string keys. It inherits from `SettingCollection<T>` and overrides
//! the relevant virtual functions.

use crate::re::BSTCaseInsensitiveStringMap::BSTCaseInsensitiveStringMap;
use crate::re::Setting::Setting;
use crate::re::SettingCollection::SettingCollection;

/// Represents a map-based collection of settings.
///
/// Inherits from `SettingCollection<T>`.
///
/// # Memory Layout:
/// - `sub_key`: Sub-key array (0x104)
/// - `handle`: Handle pointer (0x110)
/// - `settings`: Case-insensitive map of settings (0x118)
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettingCollectionMap<T> {
    /// Base `SettingCollection<T>` struct.
    pub __base: SettingCollection<T>,

    /// Map of case-insensitive strings to settings.
    pub settings: BSTCaseInsensitiveStringMap<*mut T>,
}
const _: () = {
    assert!(core::mem::offset_of!(SettingCollectionMap::<Setting>, __base) == 0x00);
    assert!(core::mem::offset_of!(SettingCollectionMap::<Setting>, settings) == 0x118);
    assert!(core::mem::size_of::<SettingCollectionMap::<Setting>>() == 0x140);
};

/// The virtual function table for `SettingCollectionMap<T>`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
pub struct SettingCollectionMapVtbl<T> {
    /// Destructor function pointer.
    pub CxxDrop: fn(this: &mut SettingCollectionMap<T>),

    /// Function pointer for inserting a setting.
    pub InsertSetting: fn(this: &mut SettingCollectionMap<T>, setting: &mut T),

    /// Function pointer for removing a setting.
    pub RemoveSetting: fn(this: &mut SettingCollectionMap<T>, setting: &mut T),

    /// Function pointer for writing all settings.
    pub WriteAllSettings: fn(this: &mut SettingCollectionMap<T>),

    /// Function pointer for reading all settings.
    pub ReadAllSettings: fn(this: &mut SettingCollectionMap<T>),
}

impl<T> Default for SettingCollectionMapVtbl<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SettingCollectionMapVtbl<T> {
    /// Creates a new default virtual table with stubbed functions.
    #[inline]
    pub const fn new() -> Self {
        const fn CxxDrop<T>(_this: &mut SettingCollectionMap<T>) {}

        const fn InsertSetting<T>(_this: &mut SettingCollectionMap<T>, _setting: &mut T) {}

        const fn RemoveSetting<T>(_this: &mut SettingCollectionMap<T>, _setting: &mut T) {}

        const fn WriteAllSettings<T>(_this: &mut SettingCollectionMap<T>) {}

        const fn ReadAllSettings<T>(_this: &mut SettingCollectionMap<T>) {}

        Self { CxxDrop, InsertSetting, RemoveSetting, WriteAllSettings, ReadAllSettings }
    }
}

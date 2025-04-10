//! # GameSettingCollection
//!
//! This module defines the `GameSettingCollection` struct, inheriting from `SettingCollectionMap<Setting>`.
//! It represents the game's settings collection, including methods for interacting with settings and handling.

use core::fmt;
use std::collections::HashMap;

use crate::re::Setting::{Setting, SettingValue};
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
#[derive(Clone, PartialEq)]
pub struct GameSettingCollection {
    pub __base: SettingCollectionMap<Setting>, // 0x000
}
const _: () = {
    assert!(core::mem::offset_of!(GameSettingCollection, __base) == 0x0);
    assert!(core::mem::size_of::<GameSettingCollection>() == 0x140);
};

impl fmt::Debug for GameSettingCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = f.debug_map();
        for (key, value) in self.__base.settings.__base.__base.__base.iter() {
            let key_str = unsafe {
                if key.is_null() {
                    "<null>"
                } else {
                    core::ffi::CStr::from_ptr(*key).to_str().unwrap_or("<invalid utf8>")
                }
            };
            let value = unsafe { value.as_ref().map(|p| p.get_value()).unwrap_or_default() };
            map.entry(&key_str, &value);
        }
        map.finish()
    }
}

impl GameSettingCollection {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_GameSettingCollection;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_GameSettingCollection;

    /// Gets the singleton instance of `GameSettingCollection`.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut GameSettingCollection",
        default = "None",
        id(se = 514622, ae = 400782)
    )]
    pub fn get_singleton() -> Option<&'static GameSettingCollection> {
        |as_type: AsType| unsafe { as_type.as_ref() }
    }

    pub fn to_hashmap(&self) -> HashMap<&str, SettingValue<'_>> {
        let mut map = std::collections::HashMap::new();
        for (key, value) in self.__base.settings.__base.__base.__base.iter() {
            let key_str = unsafe {
                if key.is_null() {
                    "<null>"
                } else {
                    core::ffi::CStr::from_ptr(*key).to_str().unwrap_or("<invalid utf8>")
                }
            };
            let value = unsafe { value.as_ref().map(|p| p.get_value()).unwrap_or_default() };
            map.insert(key_str, value);
        }
        map
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

//! # ExtraTextDisplayData
//!
//! Represents extra data for text display, including a name, temper factor, and owner quest.
//!
//! Inherits from `BSExtraData` and uses `BSFixedString` for display names.
//!
//! # Memory Layout:
//! - `__base`: Base class `BSExtraData`
//! - `display_name`: Display name string (0x10)
//! - `display_name_text`: Pointer to `BGSMessage` (0x18)
//! - `owner_quest`: Pointer to `TESQuest` (0x20)
//! - `owner_instance`: Display data type (0x28)
//! - `temper_factor`: Temper factor (0x2C)
//! - `custom_name_length`: Length of the custom name (0x30)
//! - `pad32`: Padding for alignment (0x32)
//! - `pad34`: Padding for alignment (0x34)

use crate::re::BGSMessage::BGSMessage;
use crate::re::BSExtraData::{BSExtraData, DerivedBSExtraData};
use crate::re::BSFixedString::BSFixedString;
use crate::re::ExtraDataType::ExtraDataType;
use crate::re::TESBoundObject::TESBoundObject;
use crate::re::TESQuest::TESQuest;
use crate::re::offsets_rtti::RTTI_ExtraTextDisplayData;
use crate::re::offsets_vtable::VTABLE_ExtraTextDisplayData;
use crate::rel::id::VariantID;
use core::ffi::{CStr, c_char, c_float};
use std::ptr;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct ExtraTextDisplayData {
    /// Base class `BSExtraData`.
    pub __base: BSExtraData,

    /// Display name string.
    /// Offset: `0x10`
    pub display_name: BSFixedString,

    /// Pointer to a `BGSMessage`.
    /// Offset: `0x18`
    pub display_name_text: *mut BGSMessage,

    /// Pointer to the owner quest.
    /// Offset: `0x20`
    pub owner_quest: *mut TESQuest,

    /// Display data type.
    /// Offset: `0x28`
    pub owner_instance: DisplayDataType,

    /// Temper factor.
    /// Offset: `0x2C`
    pub temper_factor: f32,

    /// Length of the custom name.
    /// Offset: `0x30`
    pub custom_name_length: u16,

    /// Padding for alignment.
    /// Offset: `0x32`
    pub pad32: u16,

    /// Padding for alignment.
    /// Offset: `0x34`
    pub pad34: u32,
}

const _: () = {
    assert!(core::mem::offset_of!(ExtraTextDisplayData, __base) == 0x0);
    assert!(core::mem::offset_of!(ExtraTextDisplayData, display_name) == 0x10);
    assert!(core::mem::offset_of!(ExtraTextDisplayData, display_name_text) == 0x18);
    assert!(core::mem::offset_of!(ExtraTextDisplayData, owner_quest) == 0x20);
    assert!(core::mem::offset_of!(ExtraTextDisplayData, owner_instance) == 0x28);
    assert!(core::mem::offset_of!(ExtraTextDisplayData, temper_factor) == 0x2C);
    assert!(core::mem::offset_of!(ExtraTextDisplayData, custom_name_length) == 0x30);
    assert!(core::mem::offset_of!(ExtraTextDisplayData, pad32) == 0x32);
    assert!(core::mem::offset_of!(ExtraTextDisplayData, pad34) == 0x34);
    assert!(core::mem::size_of::<ExtraTextDisplayData>() == 0x38);
};

impl Default for ExtraTextDisplayData {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl DerivedBSExtraData for ExtraTextDisplayData {
    #[inline]
    fn get_extra_data(&self) -> &BSExtraData {
        &self.__base
    }

    #[inline]
    fn get_extra_data_type() -> ExtraDataType {
        Self::EXTRA_DATA_TYPE
    }
}

impl ExtraTextDisplayData {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_ExtraTextDisplayData;

    /// Address & offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_ExtraTextDisplayData;

    /// The `ExtraDataType` value for text display data.
    pub const EXTRA_DATA_TYPE: ExtraDataType = ExtraDataType::TextDisplayData;

    /// Creates a new `ExtraTextDisplayData` instance with default values.
    #[inline]
    pub const fn new() -> Self {
        Self {
            __base: BSExtraData::new(),
            display_name: BSFixedString::DEFAULT,
            display_name_text: ptr::null_mut(),
            owner_quest: ptr::null_mut(),
            owner_instance: DisplayDataType::Uninitialized,
            temper_factor: 1.0,
            custom_name_length: 0,
            pad32: 0,
            pad34: 0,
        }
    }

    /// Creates a new `ExtraTextDisplayData` instance with a specified name.
    #[inline]
    pub fn from_name(name: &CStr) -> Self {
        let mut instance = Self::new();
        instance.set_name(name);
        instance
    }

    /// Creates a new `ExtraTextDisplayData` with a form and temper factor.
    /// # Safety
    /// form is a valid ptr.
    #[inline]
    pub unsafe fn from_form(form: *mut TESBoundObject, temper_factor: f32) -> Self {
        let mut instance = Self::new();
        unsafe { instance.get_display_name(form, temper_factor) };
        instance
    }

    /// Gets the extra data type, always returning `ExtraDataType::TextDisplayData`.
    #[inline]
    pub const fn get_type(&self) -> ExtraDataType {
        ExtraDataType::TextDisplayData
    }

    /// Gets the display name based on the form and temper factor.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 12626, ae_id = 12768)]
    pub unsafe fn get_display_name(
        &mut self,
        form: *mut TESBoundObject,
        temper_factor: c_float,
    ) -> *const c_char {
    }

    /// Checks if the display data is player-set.
    #[inline]
    pub fn is_player_set(&self) -> bool {
        self.owner_instance == DisplayDataType::CustomName
    }

    /// Sets the display name.
    #[inline]
    pub fn set_name(&mut self, name: &CStr) {
        if !self.display_name_text.is_null() {
            return;
        }

        self.display_name = BSFixedString::new(name);
        self.custom_name_length = name.count_bytes() as u16;
        self.owner_instance = DisplayDataType::CustomName;
        self.temper_factor = 1.0;
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DisplayDataType: i32 {
        const Uninitialized = -1;
        const CustomName = -2;
    }
}

/// The virtual function table for `ExtraTextDisplayData`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
#[derive(Debug)]
pub struct ExtraTextDisplayDataVtbl {
    /// Destructor function pointer.
    pub CxxDrop: fn(this: &mut ExtraTextDisplayData),

    /// Function pointer for retrieving the extra data type.
    pub GetType: fn(this: &ExtraTextDisplayData) -> ExtraDataType,
}

impl Default for ExtraTextDisplayDataVtbl {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl ExtraTextDisplayDataVtbl {
    /// Creates a new default virtual table with stubbed functions.
    pub const fn new() -> Self {
        const fn CxxDrop(_this: &mut ExtraTextDisplayData) {}

        const fn GetType(_this: &ExtraTextDisplayData) -> ExtraDataType {
            ExtraTextDisplayData::EXTRA_DATA_TYPE
        }

        Self { CxxDrop, GetType }
    }
}

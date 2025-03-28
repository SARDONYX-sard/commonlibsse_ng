//! # Setting
//!
//! This module defines the `Setting` struct, representing various game settings with multiple data types.
//! It supports boolean, float, integer, color, string, and unsigned integer types.

use crate::re::Color::Color;
use crate::re::offsets_rtti::RTTI_Setting;
use crate::re::offsets_vtable::VTABLE_Setting;
use crate::rel::id::VariantID;
use core::ffi::{CStr, c_char};
use core::{ptr, str};

/// Represents a game setting with multiple data types.
///
/// # Memory Layout:
/// - `data`: Union containing the setting value (0x08)
/// - `name`: Pointer to the setting's name (0x10)
#[repr(C)]
pub struct Setting {
    pub vtable: *const SettingVtbl,

    /// Union holding the setting data.
    pub data: SettingData,

    /// Pointer to the setting name.
    pub name: *mut c_char,
}

const _: () = {
    assert!(core::mem::offset_of!(Setting, data) == 0x08);
    assert!(core::mem::offset_of!(Setting, name) == 0x10);
    assert!(core::mem::size_of::<Setting>() == 0x18);
};

impl Default for Setting {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

/// Represents the types of values in a `Setting`.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingType {
    Unknown = 0,
    Bool,
    Float,
    SignedInteger,
    Color,
    String,
    UnsignedInteger,
}

/// Represents the union holding the setting's value.
///
/// # Memory Layout:
/// - `b`: Boolean value (0x00)
/// - `f`: Float value (0x00)
/// - `i`: Signed integer (0x00)
/// - `r`: Color value (0x00)
/// - `s`: Pointer to a C string (0x00)
/// - `u`: Unsigned integer (0x00)
#[repr(C)]
#[derive(Clone, Copy)]
pub union SettingData {
    pub b: bool,
    pub f: f32,
    pub i: i32,
    pub r: Color,
    pub s: *mut i8,
    pub u: u32,
}

const _: () = {
    assert!(core::mem::size_of::<SettingData>() == 0x8);
};

impl Setting {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_Setting;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_Setting;

    /// Creates a new `Setting` with default values.
    ///
    /// - `data`: Default-initialized union.
    /// - `name`: Null pointer.
    #[inline]
    pub const fn new() -> Self {
        Self { vtable: ptr::null(), data: SettingData { u: 0 }, name: ptr::null_mut() }
    }

    /// Retrieves the name of the setting.
    ///
    /// # Returns
    /// - `Some(&str)` if the name is valid.
    /// - `None` if the name pointer is null.
    #[inline]
    pub fn get_name(&self) -> Option<&str> {
        if self.name.is_null() {
            return None;
        }

        unsafe { CStr::from_ptr(self.name).to_str().ok() }
    }

    /// Retrieves the setting type.
    ///
    /// # Returns
    /// - The `SettingType` corresponding to the setting.
    #[inline]
    pub const fn get_type(&self) -> SettingType {
        // Placeholder logic; replace with actual type retrieval logic if necessary.
        SettingType::Unknown
    }

    /// Checks if the setting is managed.
    ///
    /// # Returns
    /// - `true` if the setting is managed.
    /// - `false` otherwise.
    #[inline]
    pub const fn is_managed(&self) -> bool {
        // Placeholder: implement the correct logic here.
        false
    }

    /// Retrieves the boolean value.
    ///
    /// # Returns
    /// - `true` or `false`.
    #[inline]
    pub const fn get_bool(&self) -> bool {
        unsafe { self.data.b }
    }

    /// Retrieves the float value.
    ///
    /// # Returns
    /// - `f32` value.
    #[inline]
    pub const fn get_float(&self) -> f32 {
        unsafe { self.data.f }
    }

    /// Retrieves the signed integer value.
    ///
    /// # Returns
    /// - `i32` value.
    #[inline]
    pub const fn get_sint(&self) -> i32 {
        unsafe { self.data.i }
    }

    /// Retrieves the color value.
    ///
    /// # Returns
    /// - `Color` value.
    #[inline]
    pub const fn get_color(&self) -> Color {
        unsafe { self.data.r }
    }

    /// Retrieves the string value.
    ///
    /// # Returns
    /// - `Some(&str)` if valid.
    /// - `None` if the pointer is null or invalid.
    #[inline]
    pub fn get_string(&self) -> Option<&str> {
        if unsafe { self.data.s.is_null() } {
            return None;
        }

        unsafe {
            let c_str = CStr::from_ptr(self.data.s);
            c_str.to_str().ok()
        }
    }

    /// Retrieves the unsigned integer value.
    ///
    /// # Returns
    /// - `u32` value.
    #[inline]
    pub const fn get_uint(&self) -> u32 {
        unsafe { self.data.u }
    }
}

/// The virtual function table for `Setting`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
pub struct SettingVtbl {
    /// Destructor function pointer.
    pub CxxDrop: fn(this: &mut Setting),

    /// Unknown function pointer `0x01`.
    pub Unk_01: fn(this: &mut Setting) -> bool,
}

impl Default for SettingVtbl {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl SettingVtbl {
    /// Creates a new default virtual table with stubbed functions.
    #[inline]
    pub const fn new() -> Self {
        const fn CxxDrop(_this: &mut Setting) {}

        const fn Unk_01(_this: &mut Setting) -> bool {
            false
        }

        Self { CxxDrop, Unk_01 }
    }
}

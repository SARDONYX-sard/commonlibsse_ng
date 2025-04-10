//! # Setting
//!
//! This module defines the `Setting` struct, representing various game settings
//! with multiple data types such as boolean, float, integer, color, string, and unsigned integer.
//! It safely encapsulates a union for value storage and provides a typed API to interact with it.

use crate::re::Color::Color;
use crate::re::MemoryManager::free;
use crate::re::offsets_rtti::RTTI_Setting;
use crate::re::offsets_vtable::VTABLE_Setting;
use crate::rel::id::VariantID;
use core::ffi::{CStr, c_char};
use core::{fmt, ptr};

/// Represents a game setting with multiple data types.
///
/// # Memory Layout:
/// - `data`: Union containing the setting value (0x08)
/// - `name`: Pointer to the setting's name (0x10)
#[repr(C)]
#[derive(Clone)]
pub struct Setting {
    pub vtable: *const SettingVtbl,
    data: Data,
    name: *mut c_char,
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

impl fmt::Debug for Setting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Setting")
            .field("vtable", &self.vtable)
            .field("name", &self.get_name().map(|name| name.to_str().unwrap_or("Invalid Name")))
            .field("data", &self.get_value())
            .finish()
    }
}

impl PartialEq for Setting {
    fn eq(&self, other: &Self) -> bool {
        self.get_type() == other.get_type() && self.get_value() == other.get_value()
    }
}

/// Represents the value type of a `Setting`.
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    #[default]
    Unknown = 0,
    Bool,
    Float,
    SignedInteger,
    Color,
    String,
    UnsignedInteger,
}

/// A safe wrapper for accessing the value in a `Setting`.
#[derive(Default, Clone, PartialEq)]
pub enum SettingValue<'a> {
    Bool(bool),
    Float(f32),
    SignedInteger(i32),
    Color(Color),
    /// Maybe utf-8
    String(&'a CStr),
    UnsignedInteger(u32),
    #[default]
    Unknown,
}

impl fmt::Debug for SettingValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(arg0) => f.debug_tuple("Bool").field(arg0).finish(),
            Self::Float(arg0) => f.debug_tuple("Float").field(arg0).finish(),
            Self::SignedInteger(arg0) => f.debug_tuple("SignedInteger").field(arg0).finish(),
            Self::Color(arg0) => f.debug_tuple("Color").field(arg0).finish(),
            Self::String(arg0) => f.debug_tuple("String").field(&arg0.to_str()).finish(),
            Self::UnsignedInteger(arg0) => f.debug_tuple("UnsignedInteger").field(arg0).finish(),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Union representing various setting values.
///
/// field name => value type
#[repr(C)]
#[derive(Clone, Copy)]
union Data {
    pub b: bool,
    pub f: f32,
    pub i: i32,
    pub r: Color,
    pub s: *const c_char,
    pub u: u32,
}
const _: () = assert!(core::mem::size_of::<Data>() == 0x8);

impl Setting {
    /// Runtime type info offset for this type.
    pub const RTTI: VariantID = RTTI_Setting;

    /// Virtual table offset.
    pub const VTABLE: [VariantID; 1] = VTABLE_Setting;

    /// Checks whether the setting is managed (i.e., dynamically allocated).
    #[inline]
    pub const fn is_managed(&self) -> bool {
        !self.name.is_null() && unsafe { self.name.read() } == b'S' as i8
    }

    /// Returns the type of the setting based on the name prefix.
    #[inline]
    pub const fn get_type(&self) -> Type {
        match unsafe { self.name.read() } as u8 {
            b'b' => Type::Bool,
            b'f' => Type::Float,
            b'i' => Type::SignedInteger,
            b'r' => Type::Color,
            b's' | b'S' => Type::String,
            b'u' => Type::UnsignedInteger,
            _ => Type::Unknown,
        }
    }

    /// Returns the name as `&CStr` if available.
    ///
    /// # Errors
    /// Returns `None` if the pointer is null.
    #[inline]
    pub const fn get_name(&self) -> Option<&CStr> {
        if self.name.is_null() {
            return None;
        }
        unsafe { Some(CStr::from_ptr(self.name)) }
    }

    /// Returns the value as a typed enum.
    #[inline]
    pub const fn get_value(&self) -> SettingValue<'_> {
        if self.name.is_null() {
            return SettingValue::Unknown;
        }

        unsafe {
            match self.get_type() {
                Type::Bool => SettingValue::Bool(self.data.b),
                Type::Float => SettingValue::Float(self.data.f),
                Type::SignedInteger => SettingValue::SignedInteger(self.data.i),
                Type::Color => SettingValue::Color(self.data.r),
                Type::String => {
                    let s = self.data.s;
                    if s.is_null() {
                        SettingValue::Unknown
                    } else {
                        SettingValue::String(CStr::from_ptr(s))
                    }
                }
                Type::UnsignedInteger => SettingValue::UnsignedInteger(self.data.u),
                Type::Unknown => SettingValue::Unknown,
            }
        }
    }
}

impl Drop for Setting {
    fn drop(&mut self) {
        ((unsafe { &*self.vtable }).CxxDrop)(self);
    }
}

/// Virtual function table for `Setting`.
#[repr(C)]
pub struct SettingVtbl {
    pub CxxDrop: fn(this: &mut Setting),
    pub Unk_01: fn(this: &mut Setting) -> bool,
}

impl Default for SettingVtbl {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl SettingVtbl {
    #[inline]
    pub const fn new() -> Self {
        fn CxxDrop(this: &mut Setting) {
            if this.is_managed() {
                unsafe { free(this.name.cast()) };
                this.name = ptr::null_mut();
            }
        }
        const fn Unk_01(_this: &mut Setting) -> bool {
            false
        }
        Self { CxxDrop, Unk_01 }
    }
}

//! # hkStringPtr
//!
//! This module defines the `hkStringPtr` struct, representing a managed string pointer
//! in the Havok engine. It includes methods for string access, capacity checking, and
//! compatibility with C++ memory layout.

use core::ffi::{CStr, c_char};
use core::mem;
use core::ptr;

/// Represents a managed string pointer in the Havok engine.
#[repr(C)]
#[derive(Debug)]
pub struct hkStringPtr {
    /// Pointer to the string data.
    /// - Offset: `0x0`
    pub _data: *const c_char,
}

/// Ensure the memory layout matches the C++ version.
const _: () = {
    assert!(mem::size_of::<hkStringPtr>() == 0x8);
    assert!(mem::align_of::<hkStringPtr>() == mem::align_of::<*const c_char>());
};

impl hkStringPtr {
    /// Managed flag mask.
    const MANAGED: usize = 1 << 0;

    /// Creates a new `hkStringPtr` with the given string data.
    ///
    /// # Arguments
    /// - `data`: The C string pointer.
    ///
    /// # Returns
    /// - `hkStringPtr` instance.
    #[inline]
    pub const fn new(data: *const c_char) -> Self {
        Self { _data: data }
    }

    /// Returns the raw string data, removing the managed flag.
    ///
    /// # Returns
    /// - `*const c_char`: Pointer to the string data.
    #[inline]
    pub fn as_ptr(&self) -> *const c_char {
        (self._data as usize & !Self::MANAGED) as *const c_char
    }

    /// Returns the C-style string.
    ///
    /// # Returns
    /// - `&CStr`: Reference to the string data.
    #[inline]
    pub fn as_c_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_ptr()) }
    }

    /// Checks if the string is empty.
    ///
    /// - C++: `empty`
    ///
    /// # Returns
    /// - `bool`: `true` if empty, otherwise `false`.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self._data.is_null() || unsafe { *self.as_ptr() == 0 }
    }

    /// Returns the size of the string.
    ///
    /// - C++: Equivalent `size()`/`length()` method
    ///
    /// # Returns
    /// - `i32`: Length of the string.
    #[inline]
    pub fn len(&self) -> usize {
        if self.is_empty() { 0 } else { unsafe { CStr::from_ptr(self._data).count_bytes() } }
    }
}

impl Default for hkStringPtr {
    #[inline]
    fn default() -> Self {
        Self::new(ptr::null())
    }
}

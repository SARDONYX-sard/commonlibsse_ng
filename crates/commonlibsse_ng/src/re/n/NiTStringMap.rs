//! # NiTStringMap
//!
//! This module defines the `NiTStringMap<T>` and `NiTStringTemplateMap<Parent, T>` structs,
//! simulating the base map and its template inheritance behavior.

use crate::re::NiTMap::NiTMap;
use core::{
    ffi::c_char,
    hash::{Hash as _, Hasher as _},
};

/// Represents a string template map.
///
/// Simulates the `NiTStringTemplateMap` C++ class.
#[repr(C)]
pub struct NiTStringTemplateMap<Parent, T> {
    /// Base map.
    pub __base: Parent,

    /// Copy flag.
    pub copy: bool,

    // Padding to align with C++ memory layout.
    pub pad21: u8,
    pub pad22: u16,
    pub pad24: u32,

    marker: core::marker::PhantomData<T>,
}

const _: () = {
    type ParentType = [u8; 0x20];
    assert!(core::mem::offset_of!(NiTStringTemplateMap::<ParentType, ()>, __base) == 0x00);
    assert!(core::mem::offset_of!(NiTStringTemplateMap::<ParentType, ()>, copy) == 0x20);
    assert!(core::mem::offset_of!(NiTStringTemplateMap::<ParentType, ()>, pad21) == 0x21);
    assert!(core::mem::offset_of!(NiTStringTemplateMap::<ParentType, ()>, pad22) == 0x22);
    assert!(core::mem::offset_of!(NiTStringTemplateMap::<ParentType, ()>, pad24) == 0x24);
    assert!(core::mem::size_of::<NiTStringTemplateMap::<ParentType, ()>>() == 0x28);
};

/// Represents a case-sensitive string map.
///
/// Inherits from `NiTStringTemplateMap`.
#[repr(C)]
pub struct NiTStringMap<T> {
    /// Base `NiTStringTemplateMap`.
    pub __base: NiTStringTemplateMap<NiTMap<*const c_char, T>, T>,
}

const _: () = {
    assert!(core::mem::offset_of!(NiTStringMap::<()>, __base) == 0x00);
    assert!(core::mem::size_of::<NiTStringMap::<()>>() == 0x28);
};

/// Trait representing the `NiTStringMap` behavior.
pub trait NiTStringMapTrait<T> {
    /// Hashes a string key.
    ///
    /// # Arguments
    /// - `key`: The string key.
    ///
    /// # Returns
    /// - The hash value.
    fn hash_function(&self, key: &str) -> u32;

    /// Checks if two keys are equal.
    ///
    /// # Arguments
    /// - `lhs`: The left-hand side key.
    /// - `rhs`: The right-hand side key.
    ///
    /// # Returns
    /// - `true` if equal, `false` otherwise.
    fn key_eq(&self, lhs: &str, rhs: &str) -> bool;

    /// Assigns a value to the map.
    ///
    /// # Arguments
    /// - `key`: The key.
    /// - `value`: The mapped value.
    fn assign_value(&mut self, key: &str, value: T);

    /// Clears a value from the map.
    ///
    /// # Arguments
    /// - `key`: The key to remove.
    fn clear_value(&mut self, key: &str);
}

impl<T> NiTStringMapTrait<T> for NiTStringMap<T> {
    #[inline]
    fn hash_function(&self, key: &str) -> u32 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as u32
    }

    #[inline]
    fn key_eq(&self, lhs: &str, rhs: &str) -> bool {
        lhs == rhs
    }

    #[inline]
    fn assign_value(&mut self, key: &str, value: T) {
        let c_key = std::ffi::CString::new(key).unwrap();
        self.__base.__base.insert(c_key.as_ptr(), value);
    }

    #[inline]
    fn clear_value(&mut self, key: &str) {
        let c_key = std::ffi::CString::new(key).unwrap();
        self.__base.__base.remove(&c_key.as_ptr());
    }
}

//! # BSTCaseInsensitiveStringMap
//!
//! This module defines the `BSTCaseInsensitiveStringMap<T>` and `NiTStringMap<T>` structs,
//! simulating the case-insensitive string map and its base map functionality.

use std::hash::{Hash, Hasher};

use crate::re::NiTStringMap::NiTStringMap;

/// Represents a case-insensitive string map.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BSTCaseInsensitiveStringMap<V> {
    pub __base: NiTStringMap<V>,
}

const _: () = {
    type ParentType = [u8; 0x20];
    assert!(core::mem::offset_of!(BSTCaseInsensitiveStringMap::<ParentType>, __base) == 0x00);
    assert!(core::mem::size_of::<BSTCaseInsensitiveStringMap::<ParentType>>() == 0x28);
};

/// Trait to represent map operations.
pub trait BSTCaseInsensitiveStringMapTrait<T> {
    /// Computes the hash of the key in a case-insensitive manner.
    ///
    /// # Arguments
    /// - `key`: The string key.
    ///
    /// # Returns
    /// - The hash value.
    fn hash_function(&self, key: &str) -> u32;

    /// Checks if two keys are equal in a case-insensitive manner.
    ///
    /// # Arguments
    /// - `lhs`: The left-hand side key.
    /// - `rhs`: The right-hand side key.
    ///
    /// # Returns
    /// - `true` if equal, `false` otherwise.
    fn key_eq(&self, lhs: &str, rhs: &str) -> bool;
}

impl<T> BSTCaseInsensitiveStringMapTrait<T> for BSTCaseInsensitiveStringMap<T> {
    #[inline]
    fn hash_function(&self, key: &str) -> u32 {
        // Perform case-insensitive hashing
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.to_lowercase().hash(&mut hasher);
        hasher.finish() as u32
    }

    #[inline]
    fn key_eq(&self, lhs: &str, rhs: &str) -> bool {
        lhs.eq_ignore_ascii_case(rhs)
    }
}

/// The virtual function table for `BSTCaseInsensitiveStringMap<T>`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
pub struct BSTCaseInsensitiveStringMapVtbl<T> {
    /// Destructor function pointer.
    pub CxxDrop: fn(this: &mut BSTCaseInsensitiveStringMap<T>),

    /// Function pointer for hashing the key.
    pub HashFunction: fn(this: &BSTCaseInsensitiveStringMap<T>, key: &str) -> u32,

    /// Function pointer for case-insensitive key equality.
    pub KeyEq: fn(this: &BSTCaseInsensitiveStringMap<T>, lhs: &str, rhs: &str) -> bool,
}

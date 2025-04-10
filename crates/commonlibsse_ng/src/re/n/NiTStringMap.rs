//! # NiTStringMap
//!
//! This module defines the `NiTStringMap<T>` and `NiTStringTemplateMap<Parent, T>` structs,
//! simulating the base map and its template inheritance behavior.

use crate::re::NiTMap::NiTMap;
use core::{ffi::c_char, fmt};

/// Represents a string template map.
///
/// Simulates the `NiTStringTemplateMap` C++ class.
///
/// - `V`: Map value type
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NiTStringTemplateMap<Parent, V> {
    /// Base map.
    pub __base: Parent,

    /// Copy flag.
    pub copy: bool,

    // Padding to align with C++ memory layout.
    pub pad21: u8,
    pub pad22: u16,
    pub pad24: u32,

    marker: core::marker::PhantomData<V>,
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
/// - `V`: Map value type
#[repr(C)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NiTStringMap<V> {
    pub __base: NiTStringTemplateMap<NiTMap<*const c_char, V>, V>,
}
const _: () = {
    assert!(core::mem::offset_of!(NiTStringMap::<()>, __base) == 0x00);
    assert!(core::mem::size_of::<NiTStringMap::<()>>() == 0x28);
};

impl<V: fmt::Debug> fmt::Debug for NiTStringMap<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = f.debug_map();
        for (key, value) in self.__base.__base.iter() {
            let key_str = unsafe {
                if key.is_null() {
                    "<null>"
                } else {
                    core::ffi::CStr::from_ptr(*key).to_str().unwrap_or("<invalid utf8>")
                }
            };
            map.entry(&key_str, value);
        }
        map.finish()
    }
}

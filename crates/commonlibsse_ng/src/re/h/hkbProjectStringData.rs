//! # hkbProjectStringData
//!
//! This module defines the `hkbProjectStringData` struct, which inherits from `hkReferencedObject`
//! and represents string data related to the Havok project's animations, behaviors, and characters.

use crate::re::hkArray::hkArray;
use crate::re::hkReferencedObject::hkReferencedObject;
use crate::re::hkStringPtr::hkStringPtr;
use crate::re::offsets_rtti::RTTI_hkbProjectStringData;
use crate::re::offsets_vtable::VTABLE_hkbProjectStringData;
use crate::rel::id::VariantID;
use std::mem;

/// Represents string data for Havok project configurations.
#[repr(C)]
pub struct hkbProjectStringData {
    /// Base class `hkReferencedObject`.
    /// - Offset: `0x0`
    pub __base: hkReferencedObject,

    /// Array of animation filenames.
    /// - Offset: `0x10`
    pub animationFilenames: hkArray<hkStringPtr>,

    /// Array of behavior filenames.
    /// - Offset: `0x20`
    pub behaviorFilenames: hkArray<hkStringPtr>,

    /// Array of character filenames.
    /// - Offset: `0x30`
    pub characterFilenames: hkArray<hkStringPtr>,

    /// Array of event names.
    /// - Offset: `0x40`
    pub eventNames: hkArray<hkStringPtr>,

    /// Path to the animations.
    /// - Offset: `0x50`
    pub animationPath: hkStringPtr,

    /// Path to the behaviors.
    /// - Offset: `0x58`
    pub behaviorPath: hkStringPtr,

    /// Path to the characters.
    /// - Offset: `0x60`
    pub characterPath: hkStringPtr,

    /// Path to the scripts.
    /// - Offset: `0x68`
    pub scriptsPath: hkStringPtr,

    /// Full path to the source.
    /// - Offset: `0x70`
    pub fullPathToSource: hkStringPtr,

    /// Root path.
    /// - Offset: `0x78`
    pub rootPath: hkStringPtr,
}

/// Ensure the memory layout matches the C++ version.
const _: () = {
    assert!(mem::size_of::<hkbProjectStringData>() == 0x80);
    assert!(mem::offset_of!(hkbProjectStringData, __base) == 0x0);
    assert!(mem::offset_of!(hkbProjectStringData, animationFilenames) == 0x10);
    assert!(mem::offset_of!(hkbProjectStringData, behaviorFilenames) == 0x20);
    assert!(mem::offset_of!(hkbProjectStringData, characterFilenames) == 0x30);
    assert!(mem::offset_of!(hkbProjectStringData, eventNames) == 0x40);
    assert!(mem::offset_of!(hkbProjectStringData, animationPath) == 0x50);
    assert!(mem::offset_of!(hkbProjectStringData, behaviorPath) == 0x58);
    assert!(mem::offset_of!(hkbProjectStringData, characterPath) == 0x60);
    assert!(mem::offset_of!(hkbProjectStringData, scriptsPath) == 0x68);
    assert!(mem::offset_of!(hkbProjectStringData, fullPathToSource) == 0x70);
    assert!(mem::offset_of!(hkbProjectStringData, rootPath) == 0x78);
};

impl crate::re::hkRefPtr::hkRefPtrCounted for hkbProjectStringData {}

impl Default for hkbProjectStringData {
    fn default() -> Self {
        Self::new()
    }
}

impl hkbProjectStringData {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_hkbProjectStringData;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_hkbProjectStringData;

    /// Creates a new `hkbProjectStringData` instance with default values.
    ///
    /// - Empty `hkArray` for filenames and event names.
    /// - Empty `hkStringPtr` for paths.
    #[inline]
    pub fn new() -> Self {
        Self {
            __base: hkReferencedObject::new(),
            animationFilenames: hkArray::new(),
            behaviorFilenames: hkArray::new(),
            characterFilenames: hkArray::new(),
            eventNames: hkArray::new(),
            animationPath: hkStringPtr::default(),
            behaviorPath: hkStringPtr::default(),
            characterPath: hkStringPtr::default(),
            scriptsPath: hkStringPtr::default(),
            fullPathToSource: hkStringPtr::default(),
            rootPath: hkStringPtr::default(),
        }
    }
}

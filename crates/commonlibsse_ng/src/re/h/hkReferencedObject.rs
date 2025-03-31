use core::sync::atomic::{AtomicI16, Ordering};

use crate::re::hkBaseObject::hkBaseObject;
use crate::re::offsets_rtti::RTTI_hkReferencedObject;
use crate::re::offsets_vtable::VTABLE_hkReferencedObject;
use crate::re::{hkClass, hkStatisticsCollector};
use crate::rel::id::VariantID;

/// Represents a reference-counted object in the Havok system.
///
/// Inherits from `hkBaseObject` and adds reference counting functionality.
#[repr(C)]
#[derive(Debug)]
pub struct hkReferencedObject {
    /// Base class `hkBaseObject`.
    pub __base: hkBaseObject,

    /// Combined memory size and flags.
    /// - Offset: 0x08
    pub memSizeAndFlags: u16,

    /// Volatile mutable reference count.
    /// - Offset: 0x0A
    pub referenceCount: AtomicI16,

    /// Padding to maintain memory alignment.
    /// - Offset: 0x0C
    pub pad0C: u32,
}

// Compile-time memory layout verification
const _: () = {
    assert!(core::mem::offset_of!(hkReferencedObject, __base) == 0x0);
    assert!(core::mem::offset_of!(hkReferencedObject, memSizeAndFlags) == 0x08);
    assert!(core::mem::offset_of!(hkReferencedObject, referenceCount) == 0x0A);
    assert!(core::mem::offset_of!(hkReferencedObject, pad0C) == 0x0C);
    assert!(core::mem::size_of::<hkReferencedObject>() == 0x10);
};

/// Lock mode enumeration for reference counting behavior.
#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum LockMode {
    None = 0,
    Auto = 1,
    Manual = 2,
}

impl hkReferencedObject {
    /// RTTI identifier for this type.
    pub const RTTI: VariantID = RTTI_hkReferencedObject;

    /// Virtual function table addresses.
    pub const VTABLE: [VariantID; 1] = VTABLE_hkReferencedObject;

    /// Maximum memory size constant.
    pub const MEM_SIZE: u16 = 0x7FFF;

    /// Creates a new `hkReferencedObject` with default values.
    #[inline]
    pub fn new() -> Self {
        Self {
            __base: hkBaseObject::new(),
            memSizeAndFlags: Self::MEM_SIZE,
            referenceCount: AtomicI16::new(1), // Initial reference count of 1
            pad0C: 0,
        }
    }

    /// Adds a reference to this object by incrementing the reference count atomically.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 56606, ae_id = 57010)]
    #[inline]
    pub fn add_reference(&self) {}

    /// Removes a reference from this object by decrementing the reference count atomically.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 56607, ae_id = 57011)]
    #[inline]
    pub fn remove_reference(&self) {}

    /// Gets the current reference count.
    #[inline]
    pub fn get_reference_count(&self) -> i16 {
        self.referenceCount.load(Ordering::Acquire)
    }

    /// Gets the allocated size (based on memSizeAndFlags).
    #[inline]
    pub fn get_allocated_size(&self) -> i32 {
        (self.memSizeAndFlags & Self::MEM_SIZE) as i32
    }
}

/// Virtual function table for `hkReferencedObject`.
#[repr(C)]
pub struct hkReferencedObjectVtbl {
    /// Destructor function pointer (hkBaseObject's virtual destructor is default).
    pub CxxDrop: fn(this: &mut hkReferencedObject),

    /// Gets the class type (returns null by default).
    pub GetClassType: fn(this: &hkReferencedObject) -> Option<*const hkClass>,

    /// Calculates content statistics.
    pub CalcContentStatistics:
        fn(this: &hkReferencedObject, collector: &mut hkStatisticsCollector, cls: Option<&hkClass>),
}

impl Default for hkReferencedObjectVtbl {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl hkReferencedObjectVtbl {
    /// Creates a new default virtual table with stubbed functions.
    #[inline]
    pub const fn new() -> Self {
        const fn CxxDrop(_this: &mut hkReferencedObject) {}

        const fn GetClassType(_this: &hkReferencedObject) -> Option<*const hkClass> {
            None
        }

        const fn CalcContentStatistics(
            _this: &hkReferencedObject,
            _collector: &mut hkStatisticsCollector,
            _cls: Option<&hkClass>,
        ) {
        }

        Self { CxxDrop, GetClassType, CalcContentStatistics }
    }
}

impl Default for hkReferencedObject {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

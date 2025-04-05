//! # ExtraAshPileRef
//!
//! This module defines the `ExtraAshPileRef` struct, which inherits from `BSExtraData` and represents
//! extra data for ash pile references in Skyrim's engine. It includes a virtual table for C++ compatibility
//! and maintains the original memory layout.

use crate::re::BSExtraData::DerivedBSExtraData;
use crate::re::BSPointerHandle::ObjectRefHandle;
use crate::re::offsets_rtti::RTTI_ExtraAshPileRef;
use crate::re::offsets_vtable::VTABLE_ExtraAshPileRef;
use crate::re::{BSExtraData::BSExtraData, ExtraDataType::ExtraDataType};
use crate::rel::id::VariantID;

/// Represents extra data for an ash pile reference.
#[repr(C)]
pub struct ExtraAshPileRef {
    /// Base class `BSExtraData`.
    pub __base: BSExtraData,

    /// The object reference handle pointing to the ash pile.
    /// - Offset: `0x10`
    pub ash_pile_ref: ObjectRefHandle,

    /// Padding to maintain memory alignment.
    /// - Offset: `0x14`
    pub pad14: u32,
}

const _: () = {
    assert!(core::mem::offset_of!(ExtraAshPileRef, __base) == 0x0);
    assert!(core::mem::offset_of!(ExtraAshPileRef, ash_pile_ref) == 0x10);
    assert!(core::mem::offset_of!(ExtraAshPileRef, pad14) == 0x14);
    assert!(core::mem::size_of::<ExtraAshPileRef>() == 0x18);
};

impl Default for ExtraAshPileRef {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl DerivedBSExtraData for ExtraAshPileRef {
    #[inline]
    fn get_extra_data(&self) -> &BSExtraData {
        &self.__base
    }

    #[inline]
    fn get_extra_data_type() -> ExtraDataType {
        Self::EXTRA_DATA_TYPE
    }
}

impl ExtraAshPileRef {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_ExtraAshPileRef;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_ExtraAshPileRef;

    /// The `ExtraDataType` value for ash pile references.
    pub const EXTRA_DATA_TYPE: ExtraDataType = ExtraDataType::AshPileRef;

    /// Creates a new `ExtraAshPileRef` instance with default values.
    ///
    /// - `__base`: Default `BSExtraData`
    /// - `ash_pile_ref`: Default `ObjectRefHandle`
    /// - `pad14`: Zeroed padding
    #[inline]
    pub fn new() -> Self {
        Self { __base: BSExtraData::new(), ash_pile_ref: ObjectRefHandle::default(), pad14: 0 }
    }

    /// Creates a new `ExtraAshPileRef` instance with a specific `ObjectRefHandle`.
    ///
    /// - `ash_pile_ref`: The reference handle pointing to the ash pile.
    ///
    /// # Returns
    /// An instance of `ExtraAshPileRef` with the specified reference handle.
    #[inline]
    pub const fn with_ref(ash_pile_ref: ObjectRefHandle) -> Self {
        Self { __base: BSExtraData::new(), ash_pile_ref, pad14: 0 }
    }

    /// Retrieves the extra data type, always returning `ExtraDataType::kAshPileRef`.
    ///
    /// # Returns
    /// - `ExtraDataType::kAshPileRef`
    #[inline]
    pub const fn get_type(&self) -> ExtraDataType {
        ExtraDataType::AshPileRef
    }
}

/// The virtual function table for `ExtraAshPileRef`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
pub struct ExtraAshPileRefVtbl {
    /// Destructor function pointer.
    pub CxxDrop: fn(this: &mut ExtraAshPileRef),

    /// Function pointer for retrieving the extra data type.
    pub GetType: fn(this: &ExtraAshPileRef) -> ExtraDataType,

    /// Function pointer for equality check.
    pub IsNotEqual: fn(this: &ExtraAshPileRef, rhs: &ExtraAshPileRef) -> bool,
}

impl Default for ExtraAshPileRefVtbl {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl ExtraAshPileRefVtbl {
    /// Creates a new default virtual table with stubbed functions.
    #[inline]
    pub const fn new() -> Self {
        const fn CxxDrop(_this: &mut ExtraAshPileRef) {}

        const fn GetType(_this: &ExtraAshPileRef) -> ExtraDataType {
            ExtraAshPileRef::EXTRA_DATA_TYPE
        }

        const fn IsNotEqual(_this: &ExtraAshPileRef, _rhs: &ExtraAshPileRef) -> bool {
            false
        }

        Self { CxxDrop, GetType, IsNotEqual }
    }
}

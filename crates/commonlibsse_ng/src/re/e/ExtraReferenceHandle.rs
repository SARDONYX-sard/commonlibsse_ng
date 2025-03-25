//! # ExtraReferenceHandle
//!
//! Represents extra data for reference handles.
//!
//! Inherits from `BSExtraData` and includes a reference handle and padding for alignment.
//!
//! # Memory Layout:
//! - `__base`: Base class `BSExtraData`
//! - `container_ref`: The object reference handle (0x10)
//! - `pad14`: Padding for alignment (0x14)

use crate::re::BSExtraData::{BSExtraData, DerivedBSExtraData};
use crate::re::BSPointerHandle::ObjectRefHandle;
use crate::re::ExtraDataType::ExtraDataType;
use crate::re::NiPointer::NiPointer;
use crate::re::TESObjectREFR::TESObjectREFR;
use crate::re::offsets_rtti::RTTI_ExtraReferenceHandle;
use crate::re::offsets_vtable::VTABLE_ExtraReferenceHandle;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct ExtraReferenceHandle {
    /// Base class `BSExtraData`.
    pub __base: BSExtraData,

    /// The object reference handle.
    /// Offset: `0x10`
    pub container_ref: ObjectRefHandle,

    /// Padding to match C++ structure alignment.
    /// Offset: `0x14`
    pub pad14: u32,
}

const _: () = {
    assert!(core::mem::offset_of!(ExtraReferenceHandle, __base) == 0x0);
    assert!(core::mem::offset_of!(ExtraReferenceHandle, container_ref) == 0x10);
    assert!(core::mem::offset_of!(ExtraReferenceHandle, pad14) == 0x14);
    assert!(core::mem::size_of::<ExtraReferenceHandle>() == 0x18);
};

impl Default for ExtraReferenceHandle {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl DerivedBSExtraData for ExtraReferenceHandle {
    #[inline]
    fn get_extra_data(&self) -> &BSExtraData {
        &self.__base
    }

    #[inline]
    fn get_extra_data_type() -> ExtraDataType {
        Self::EXTRA_DATA_TYPE
    }
}

impl ExtraReferenceHandle {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_ExtraReferenceHandle;

    /// Address & offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_ExtraReferenceHandle;

    /// The `ExtraDataType` value for reference handles.
    pub const EXTRA_DATA_TYPE: ExtraDataType = ExtraDataType::ReferenceHandle;

    /// Creates a new `ExtraReferenceHandle` instance with a null reference.
    #[inline]
    pub const fn new() -> Self {
        Self { __base: BSExtraData::new(), container_ref: ObjectRefHandle::null(), pad14: 0 }
    }

    /// Creates a new `ExtraReferenceHandle` with a specific reference handle.
    #[inline]
    pub const fn from_handle(container_ref: ObjectRefHandle) -> Self {
        Self { __base: BSExtraData::new(), container_ref, pad14: 0 }
    }

    /// Gets the extra data type, always returning `ExtraDataType::ReferenceHandle`.
    #[inline]
    pub const fn get_type(&self) -> ExtraDataType {
        ExtraDataType::ReferenceHandle
    }

    /// Checks if this `ExtraReferenceHandle` is not equal to another.
    #[inline]
    pub fn is_not_equal(&self, rhs: &Self) -> bool {
        self.container_ref != rhs.container_ref
    }

    /// Retrieves the original reference as `NiPointer<TESObjectREFR>`.
    #[inline]
    pub fn get_original_reference(&self) -> NiPointer<TESObjectREFR> {
        self.container_ref.get()
    }
}

/// The virtual function table for `ExtraReferenceHandle`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
#[derive(Debug)]
pub struct ExtraReferenceHandleVtbl {
    /// Destructor function pointer.
    pub CxxDrop: fn(this: &mut ExtraReferenceHandle),

    /// Function pointer for retrieving the extra data type.
    pub GetType: fn(this: &ExtraReferenceHandle) -> ExtraDataType,

    /// Function pointer for equality check.
    pub IsNotEqual: fn(this: &ExtraReferenceHandle, rhs: &ExtraReferenceHandle) -> bool,
}

impl Default for ExtraReferenceHandleVtbl {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl ExtraReferenceHandleVtbl {
    /// Creates a new default virtual table with stubbed functions.
    pub const fn new() -> Self {
        const fn CxxDrop(_this: &mut ExtraReferenceHandle) {}

        const fn GetType(_this: &ExtraReferenceHandle) -> ExtraDataType {
            ExtraReferenceHandle::EXTRA_DATA_TYPE
        }

        fn IsNotEqual(this: &ExtraReferenceHandle, rhs: &ExtraReferenceHandle) -> bool {
            this.container_ref != rhs.container_ref
        }

        Self { CxxDrop, GetType, IsNotEqual }
    }
}

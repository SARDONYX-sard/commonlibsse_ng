//! # ExtraCount
use crate::re::BSExtraData::{BSExtraData, DerivedBSExtraData};
use crate::re::ExtraDataType::ExtraDataType;
use crate::re::offsets_rtti::RTTI_ExtraCount;
use crate::re::offsets_vtable::VTABLE_ExtraCount;
use crate::rel::id::VariantID;

/// Represents extra data for item counts.
///
/// Inherits from `BSExtraData` and includes the item count.
///
/// # Memory Layout:
/// - `__base`: Base class `BSExtraData`
/// - `count`: The item count (0x10)
/// - `pad12`: Padding to align with C++ structure (0x12)
/// - `pad14`: Additional padding for alignment (0x14)
#[repr(C)]
pub struct ExtraCount {
    /// Base class `BSExtraData`.
    pub __base: BSExtraData,

    /// The item count.
    /// Offset: `0x10`
    pub count: i16,

    /// Padding for alignment.
    /// Offset: `0x12`
    pub pad12: u16,

    /// Additional padding to match the C++ memory layout.
    /// Offset: `0x14`
    pub pad14: i32,
}

const _: () = {
    assert!(core::mem::offset_of!(ExtraCount, __base) == 0x0);
    assert!(core::mem::offset_of!(ExtraCount, count) == 0x10);
    assert!(core::mem::offset_of!(ExtraCount, pad12) == 0x12);
    assert!(core::mem::offset_of!(ExtraCount, pad14) == 0x14);
    assert!(core::mem::size_of::<ExtraCount>() == 0x18);
};

impl Default for ExtraCount {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl DerivedBSExtraData for ExtraCount {
    #[inline]
    fn get_extra_data(&self) -> &BSExtraData {
        &self.__base
    }

    #[inline]
    fn get_extra_data_type() -> ExtraDataType {
        Self::EXTRA_DATA_TYPE
    }
}

impl ExtraCount {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_ExtraCount;

    /// Address & offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_ExtraCount;

    /// The `ExtraDataType` value for item counts.
    pub const EXTRA_DATA_TYPE: ExtraDataType = ExtraDataType::Count;

    /// Creates a new `ExtraCount` instance with a default count of 0.
    #[inline]
    pub const fn new() -> Self {
        Self { __base: BSExtraData::new(), count: 0, pad12: 0, pad14: 0 }
    }

    /// Creates a new `ExtraCount` instance with a specific count.
    #[inline]
    pub const fn from_count(count: i16) -> Self {
        Self { __base: BSExtraData::new(), count, pad12: 0, pad14: 0 }
    }

    /// Retrieves the extra data type, always returning `ExtraDataType::Count`.
    #[inline]
    pub const fn get_type(&self) -> ExtraDataType {
        ExtraDataType::Count
    }

    /// Checks if this `ExtraCount` is not equal to another.
    #[inline]
    pub const fn is_not_equal(&self, rhs: &Self) -> bool {
        self.count != rhs.count
    }
}

/// The virtual function table for `ExtraCount`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
pub struct ExtraCountVtbl {
    /// Destructor function pointer.
    pub CxxDrop: fn(this: &mut ExtraCount),

    /// Function pointer for retrieving the extra data type.
    pub GetType: fn(this: &ExtraCount) -> ExtraDataType,

    /// Function pointer for equality check.
    pub IsNotEqual: fn(this: &ExtraCount, rhs: &ExtraCount) -> bool,
}

impl Default for ExtraCountVtbl {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl ExtraCountVtbl {
    /// Creates a new default virtual table with stubbed functions.
    pub const fn new() -> Self {
        const fn CxxDrop(_this: &mut ExtraCount) {}

        const fn GetType(_this: &ExtraCount) -> ExtraDataType {
            ExtraCount::EXTRA_DATA_TYPE
        }

        const fn IsNotEqual(this: &ExtraCount, rhs: &ExtraCount) -> bool {
            this.count != rhs.count
        }

        Self { CxxDrop, GetType, IsNotEqual }
    }
}

//! # ExtraHealth
//!
//! Represents extra data for health values.
//!
//! Inherits from `BSExtraData` and includes the health value with padding for memory alignment.
//!
//! # Memory Layout:
//! - `__base`: Base class `BSExtraData`
//! - `health`: The health value (0x10)
//! - `pad14`: Padding for alignment (0x14)

use crate::re::BSExtraData::{BSExtraData, DerivedBSExtraData};
use crate::re::ExtraDataType::ExtraDataType;
use crate::re::NiMath::{ComparisonOptions, nearly_equal};
use crate::re::offsets_rtti::RTTI_ExtraHealth;
use crate::re::offsets_vtable::VTABLE_ExtraHealth;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct ExtraHealth {
    /// Base class `BSExtraData`.
    pub __base: BSExtraData,

    /// The health value.
    /// Offset: `0x10`
    pub health: f32,

    /// Padding to match C++ structure alignment.
    /// Offset: `0x14`
    pub pad14: u32,
}

const _: () = {
    assert!(core::mem::offset_of!(ExtraHealth, __base) == 0x0);
    assert!(core::mem::offset_of!(ExtraHealth, health) == 0x10);
    assert!(core::mem::offset_of!(ExtraHealth, pad14) == 0x14);
    assert!(core::mem::size_of::<ExtraHealth>() == 0x18);
};

impl Default for ExtraHealth {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl DerivedBSExtraData for ExtraHealth {
    #[inline]
    fn get_extra_data(&self) -> &BSExtraData {
        &self.__base
    }

    #[inline]
    fn get_extra_data_type() -> ExtraDataType {
        Self::EXTRA_DATA_TYPE
    }
}

impl ExtraHealth {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_ExtraHealth;

    /// Address & offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_ExtraHealth;

    /// The `ExtraDataType` value for health.
    pub const EXTRA_DATA_TYPE: ExtraDataType = ExtraDataType::Health;

    /// Creates a new `ExtraHealth` instance with default health (0.0).
    #[inline]
    pub const fn new() -> Self {
        Self { __base: BSExtraData::new(), health: 0.0, pad14: 0 }
    }

    /// Creates a new `ExtraHealth` instance with a specific health value.
    #[inline]
    pub const fn from_health(health: f32) -> Self {
        Self { __base: BSExtraData::new(), health, pad14: 0 }
    }

    /// Gets the extra data type, always returning `ExtraDataType::Health`.
    #[inline]
    pub const fn get_type(&self) -> ExtraDataType {
        ExtraDataType::Health
    }

    /// Checks if this `ExtraHealth` is not equal to another.
    #[inline]
    pub const fn is_not_equal(&self, rhs: &Self) -> bool {
        nearly_equal(self.health, rhs.health, ComparisonOptions::const_default())
    }
}

/// The virtual function table for `ExtraHealth`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
#[derive(Debug)]
pub struct ExtraHealthVtbl {
    /// Destructor function pointer.
    pub CxxDrop: fn(this: &mut ExtraHealth),

    /// Function pointer for retrieving the extra data type.
    pub GetType: fn(this: &ExtraHealth) -> ExtraDataType,

    /// Function pointer for equality check.
    pub IsNotEqual: fn(this: &ExtraHealth, rhs: &ExtraHealth) -> bool,
}

impl Default for ExtraHealthVtbl {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl ExtraHealthVtbl {
    /// Creates a new default virtual table with stubbed functions.
    pub const fn new() -> Self {
        const fn CxxDrop(_this: &mut ExtraHealth) {}

        const fn GetType(_this: &ExtraHealth) -> ExtraDataType {
            ExtraHealth::EXTRA_DATA_TYPE
        }

        const fn IsNotEqual(this: &ExtraHealth, rhs: &ExtraHealth) -> bool {
            nearly_equal(this.health, rhs.health, ComparisonOptions::const_default())
        }

        Self { CxxDrop, GetType, IsNotEqual }
    }
}

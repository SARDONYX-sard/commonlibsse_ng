use crate::re::BGSEncounterZone::BGSEncounterZone;
use crate::re::BSExtraData::{BSExtraData, DerivedBSExtraData};
use crate::re::ExtraDataType::ExtraDataType;
use crate::re::offsets_rtti::RTTI_ExtraEncounterZone;
use crate::re::offsets_vtable::VTABLE_ExtraEncounterZone;
use crate::rel::id::VariantID;

/// Represents extra data for encounter zones.
///
/// Inherits from `BSExtraData` and includes a pointer to a `BGSEncounterZone`.
///
/// # Memory Layout:
/// - `__base`: Base class `BSExtraData`
/// - `zone`: Pointer to `BGSEncounterZone` (0x10)
/// - `pad`: Padding to align with C++ structure (0x18)
#[repr(C)]
pub struct ExtraEncounterZone {
    /// Base class `BSExtraData`.
    pub __base: BSExtraData,

    /// Pointer to `BGSEncounterZone`.
    /// Offset: `0x10`
    pub zone: *mut BGSEncounterZone,
}

const _: () = {
    assert!(core::mem::offset_of!(ExtraEncounterZone, __base) == 0x0);
    assert!(core::mem::offset_of!(ExtraEncounterZone, zone) == 0x10);
    assert!(core::mem::size_of::<ExtraEncounterZone>() == 0x18);
};

impl Default for ExtraEncounterZone {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl DerivedBSExtraData for ExtraEncounterZone {
    #[inline]
    fn get_extra_data(&self) -> &BSExtraData {
        &self.__base
    }

    #[inline]
    fn get_extra_data_type() -> ExtraDataType {
        Self::EXTRA_DATA_TYPE
    }
}

impl ExtraEncounterZone {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_ExtraEncounterZone;

    /// Address & offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_ExtraEncounterZone;

    /// The `ExtraDataType` value for encounter zones.
    pub const EXTRA_DATA_TYPE: ExtraDataType = ExtraDataType::EncounterZone;

    /// Creates a new `ExtraEncounterZone` instance with no zone (nullptr).
    #[inline]
    pub const fn new() -> Self {
        Self { __base: BSExtraData::new(), zone: std::ptr::null_mut() }
    }

    /// Creates a new `ExtraEncounterZone` instance with a specified zone.
    #[inline]
    pub const fn from_zone(zone: *mut BGSEncounterZone) -> Self {
        Self { __base: BSExtraData::new(), zone }
    }

    /// Retrieves the extra data type, always returning `ExtraDataType::EncounterZone`.
    #[inline]
    pub const fn get_type(&self) -> ExtraDataType {
        ExtraDataType::EncounterZone
    }

    /// Checks if this `ExtraEncounterZone` is not equal to another.
    #[inline]
    pub fn is_not_equal(&self, rhs: &Self) -> bool {
        self.zone != rhs.zone
    }
}

/// The virtual function table for `ExtraEncounterZone`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
pub struct ExtraEncounterZoneVtbl {
    /// Destructor function pointer.
    pub CxxDrop: fn(this: &mut ExtraEncounterZone),

    /// Function pointer for retrieving the extra data type.
    pub GetType: fn(this: &ExtraEncounterZone) -> ExtraDataType,

    /// Function pointer for equality check.
    pub IsNotEqual: fn(this: &ExtraEncounterZone, rhs: &ExtraEncounterZone) -> bool,
}

impl Default for ExtraEncounterZoneVtbl {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl ExtraEncounterZoneVtbl {
    /// Creates a new default virtual table with stubbed functions.
    pub const fn new() -> Self {
        const fn CxxDrop(_this: &mut ExtraEncounterZone) {}

        const fn GetType(_this: &ExtraEncounterZone) -> ExtraDataType {
            ExtraEncounterZone::EXTRA_DATA_TYPE
        }

        fn IsNotEqual(this: &ExtraEncounterZone, rhs: &ExtraEncounterZone) -> bool {
            this.zone != rhs.zone
        }

        Self { CxxDrop, GetType, IsNotEqual }
    }
}

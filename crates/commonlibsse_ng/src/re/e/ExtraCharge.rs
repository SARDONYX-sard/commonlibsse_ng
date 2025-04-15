use crate::re::BSExtraData::DerivedBSExtraData;
use crate::re::offsets_rtti::RTTI_ExtraCharge;
use crate::re::offsets_vtable::VTABLE_ExtraCharge;
use crate::re::{BSExtraData::BSExtraData, ExtraDataType::ExtraDataType};
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct ExtraCharge {
    pub __base: BSExtraData,

    /// - Offset: `0x10`
    pub charge: f32,

    /// - Offset: `0x14`
    pub pad14: u32,
}

const _: () = {
    assert!(core::mem::offset_of!(ExtraCharge, __base) == 0x0);
    assert!(core::mem::offset_of!(ExtraCharge, charge) == 0x10);
    assert!(core::mem::offset_of!(ExtraCharge, pad14) == 0x14);
    assert!(core::mem::size_of::<ExtraCharge>() == 0x18);
};

impl Default for ExtraCharge {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl DerivedBSExtraData for ExtraCharge {
    #[inline]
    fn get_extra_data(&self) -> &BSExtraData {
        &self.__base
    }

    #[inline]
    fn get_extra_data_type() -> ExtraDataType {
        Self::EXTRA_DATA_TYPE
    }
}

impl ExtraCharge {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_ExtraCharge;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_ExtraCharge;

    /// The `ExtraDataType` value for ash pile references.
    pub const EXTRA_DATA_TYPE: ExtraDataType = ExtraDataType::Charge;

    /// Creates a new `ExtraCharge` instance with default values.
    #[inline]
    pub const fn new() -> Self {
        Self { __base: BSExtraData::new(), charge: 0.0, pad14: 0 }
    }

    /// Retrieves the extra data type, always returning `ExtraDataType::kCharge`.
    ///
    /// # Returns
    /// - `ExtraDataType::kCharge`
    #[inline]
    pub const fn get_type(&self) -> ExtraDataType {
        ExtraDataType::Charge
    }
}

/// The virtual function table for `ExtraCharge`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
pub struct ExtraChargeVtbl {
    /// Destructor function pointer.
    pub CxxDrop: fn(this: &mut ExtraCharge),

    /// Function pointer for retrieving the extra data type.
    pub GetType: fn(this: &ExtraCharge) -> ExtraDataType,

    /// Function pointer for equality check.
    pub IsNotEqual: fn(this: &ExtraCharge, rhs: &ExtraCharge) -> bool,
}

impl Default for ExtraChargeVtbl {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl ExtraChargeVtbl {
    /// Creates a new default virtual table with stubbed functions.
    #[inline]
    pub const fn new() -> Self {
        const fn CxxDrop(_this: &mut ExtraCharge) {}

        const fn GetType(_this: &ExtraCharge) -> ExtraDataType {
            ExtraCharge::EXTRA_DATA_TYPE
        }

        const fn IsNotEqual(_this: &ExtraCharge, _rhs: &ExtraCharge) -> bool {
            false
        }

        Self { CxxDrop, GetType, IsNotEqual }
    }
}

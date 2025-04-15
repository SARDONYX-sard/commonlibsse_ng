use crate::re::BSCoreTypes::FormID;
use crate::re::BSExtraData::{BSExtraData, BSExtraDataVtbl, DerivedBSExtraData};
use crate::re::ExtraDataType::ExtraDataType;
use crate::re::offsets_rtti::RTTI_ExtraUniqueID;
use crate::re::offsets_vtable::VTABLE_ExtraUniqueID;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct ExtraUniqueID {
    pub __base: BSExtraData,
    pub baseID: FormID,
    pub uniqueID: u16,
    pub pad16: u16,
}

const _: () = {
    assert!(core::mem::offset_of!(ExtraUniqueID, __base) == 0x0);
    assert!(core::mem::offset_of!(ExtraUniqueID, baseID) == 0x10);
    assert!(core::mem::offset_of!(ExtraUniqueID, uniqueID) == 0x14);
    assert!(core::mem::offset_of!(ExtraUniqueID, pad16) == 0x16);
    assert!(core::mem::size_of::<ExtraUniqueID>() == 0x18);
};

impl DerivedBSExtraData for ExtraUniqueID {
    #[inline]
    fn get_extra_data(&self) -> &BSExtraData {
        &self.__base
    }

    #[inline]
    fn get_extra_data_type() -> ExtraDataType {
        Self::EXTRA_DATA_TYPE
    }
}

impl ExtraUniqueID {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_ExtraUniqueID;

    /// Address & offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_ExtraUniqueID;

    /// The `ExtraDataType` value for health.
    pub const EXTRA_DATA_TYPE: ExtraDataType = ExtraDataType::UniqueID;

    /// Gets the extra data type, always returning `ExtraDataType::UniqueID`.
    #[inline]
    pub const fn get_type(&self) -> ExtraDataType {
        ExtraDataType::UniqueID
    }
}

/// The virtual function table for `ExtraUniqueID`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
pub struct ExtraUniqueIDVtbl {
    pub __base: BSExtraDataVtbl,
}

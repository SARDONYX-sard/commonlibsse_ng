use core::ptr::NonNull;

use crate::re::BSExtraData::{BSExtraData, BSExtraDataVtbl, DerivedBSExtraData};
use crate::re::EnchantmentItem::EnchantmentItem;
use crate::re::ExtraDataType::ExtraDataType;
use crate::re::offsets_rtti::RTTI_ExtraEnchantment;
use crate::re::offsets_vtable::VTABLE_ExtraEnchantment;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct ExtraEnchantment {
    /// Base class `BSExtraData`.
    pub __base: BSExtraData,
    pub enchantment: Option<NonNull<EnchantmentItem>>,
    pub charge: u16,
    pub removeOnUnequip: bool,
    pub pad1B: u8,
    pub pad1C: u32,
}

const _: () = {
    assert!(core::mem::offset_of!(ExtraEnchantment, __base) == 0x0);
    assert!(core::mem::offset_of!(ExtraEnchantment, enchantment) == 0x10);
    assert!(core::mem::offset_of!(ExtraEnchantment, charge) == 0x18);
    assert!(core::mem::offset_of!(ExtraEnchantment, removeOnUnequip) == 0x1A);
    assert!(core::mem::offset_of!(ExtraEnchantment, pad1B) == 0x1B);
    assert!(core::mem::offset_of!(ExtraEnchantment, pad1C) == 0x1C);
    assert!(core::mem::size_of::<ExtraEnchantment>() == 0x20);
};

impl DerivedBSExtraData for ExtraEnchantment {
    #[inline]
    fn get_extra_data(&self) -> &BSExtraData {
        &self.__base
    }

    #[inline]
    fn get_extra_data_type() -> ExtraDataType {
        Self::EXTRA_DATA_TYPE
    }
}

impl ExtraEnchantment {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_ExtraEnchantment;

    /// Address & offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_ExtraEnchantment;

    /// The `ExtraDataType` value for enchantment.
    pub const EXTRA_DATA_TYPE: ExtraDataType = ExtraDataType::Enchantment;
}

/// The virtual function table for `ExtraEnchantment`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
pub struct ExtraEnchantmentVtbl {
    pub __base: BSExtraDataVtbl,
}

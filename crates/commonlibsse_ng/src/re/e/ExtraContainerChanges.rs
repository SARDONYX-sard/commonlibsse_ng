use crate::re::BSExtraData::BSExtraData;
use crate::re::ExtraDataType::ExtraDataType;
use crate::re::InventoryChanges::InventoryChanges;
use crate::re::offsets_rtti::RTTI_ExtraContainerChanges;
use crate::re::offsets_vtable::VTABLE_ExtraContainerChanges;
use crate::rel::id::VariantID;

#[repr(C)]
pub struct ExtraContainerChanges {
    pub __base: BSExtraData,
    pub changes: *mut InventoryChanges,
}

impl ExtraContainerChanges {
    pub const RTTI: VariantID = RTTI_ExtraContainerChanges;
    pub const VTABLE: [VariantID; 1] = VTABLE_ExtraContainerChanges;
    pub const EXTRA_DATA_TYPE: ExtraDataType = ExtraDataType::ContainerChanges;
}

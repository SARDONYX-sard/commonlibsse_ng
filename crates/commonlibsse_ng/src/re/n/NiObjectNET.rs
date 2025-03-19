use crate::re::BSFixedString::BSFixedString;
use crate::re::NiObject::NiObject;
use crate::re::NiSmartPointer::NiPointer;
use crate::re::{NiExtraData, NiTimeController};

#[repr(C)]
pub struct NiObjectNET {
    pub _base: NiObject, // Inherit from NiObject
    pub name: BSFixedString,
    pub controllers: NiPointer<NiTimeController>,
    pub extra: *mut *mut NiExtraData,
    pub extra_data_size: u16,
    pub max_size: u16,
    pub pad2c: u32,
}

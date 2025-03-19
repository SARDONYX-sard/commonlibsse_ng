use crate::re::NiRefObject::NiRefObject;
use crate::re::offsets_ni_rtti::NiRTTI_NiObject;
use crate::re::offsets_rtti::RTTI_NiObject;
use crate::re::offsets_vtable::VTABLE_NiObject;
use crate::rel::id::VariantID;

#[repr(C)]
pub struct NiObject {
    pub _base: NiRefObject,
}

impl NiObject {
    pub const RTTI: VariantID = RTTI_NiObject;
    pub const NI_RTTI: VariantID = NiRTTI_NiObject;
    pub const VTABLE: [VariantID; 1] = VTABLE_NiObject;
}

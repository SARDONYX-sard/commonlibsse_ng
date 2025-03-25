use crate::re::NiAVObject::NiAVObject;
use crate::re::NiColor::NiColor;
use crate::re::NiPoint3::NiPoint3;
use crate::re::offsets_ni_rtti::NiRTTI_NiLight;
use crate::re::offsets_rtti::RTTI_NiLight;
use crate::re::offsets_vtable::VTABLE_NiLight;
use crate::rel::id::VariantID;
use crate::rel::relocation::{RelocationError, relocate_member, relocate_member_mut};

#[repr(C)]
pub struct NiLight {
    pub __base: NiAVObject,
}
const _: () = assert!(core::mem::size_of::<NiLight>() == 0x110);

impl NiLight {
    pub const RTTI: VariantID = RTTI_NiLight;
    pub const NI_RTTI: VariantID = NiRTTI_NiLight;
    pub const VTABLE: [VariantID; 1] = VTABLE_NiLight;

    /// # Errors
    #[inline]
    pub fn get_light_runtime_data(&self) -> Result<&LIGHT_RUNTIME_DATA, RelocationError> {
        relocate_member(self, 0x110, 0x138)
    }

    /// # Errors
    #[inline]
    pub fn get_light_runtime_data_mut(
        &mut self,
    ) -> Result<&mut LIGHT_RUNTIME_DATA, RelocationError> {
        relocate_member_mut(self, 0x110, 0x138)
    }
}

#[repr(C)]
pub struct LIGHT_RUNTIME_DATA {
    ambient: NiColor,
    diffuse: NiColor,
    radius: NiPoint3,
    fade: f32,
    unk138: u32,
}
const _: () = {
    assert!(core::mem::offset_of!(LIGHT_RUNTIME_DATA, ambient) == 0x0);
    assert!(core::mem::offset_of!(LIGHT_RUNTIME_DATA, diffuse) == 0xc);
    assert!(core::mem::offset_of!(LIGHT_RUNTIME_DATA, radius) == 0x18);
    assert!(core::mem::offset_of!(LIGHT_RUNTIME_DATA, fade) == 0x24);
    assert!(core::mem::offset_of!(LIGHT_RUNTIME_DATA, unk138) == 0x28);
    assert!(core::mem::size_of::<LIGHT_RUNTIME_DATA>() == 0x2c);
};

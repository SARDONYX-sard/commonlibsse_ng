use crate::re::NiAVObject::NiAVObject;
use crate::re::NiColor::NiColor;
use crate::re::NiPoint3::NiPoint3;
use crate::re::offsets_ni_rtti::NiRTTI_NiLight;
use crate::re::offsets_rtti::RTTI_NiLight;
use crate::re::offsets_vtable::VTABLE_NiLight;
use crate::rel::id::VariantID;
use crate::rel::module::ModuleStateError;
use core::ffi::c_void;

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
    pub fn get_light_runtime_data(&self) -> Result<&'static LIGHT_RUNTIME_DATA, ModuleStateError> {
        use crate::rel::relocation::relocate_member;

        let this = self as *const Self as *mut c_void;
        let data = unsafe { relocate_member::<LIGHT_RUNTIME_DATA>(this, 0x110, 0x138) }?;
        if data.is_null() || !data.is_aligned() {
            todo!()
        };

        Ok(unsafe { &*data })
    }

    /// # Errors
    pub fn get_light_runtime_data_mut(
        &mut self,
    ) -> Result<&'static mut LIGHT_RUNTIME_DATA, ModuleStateError> {
        use crate::rel::relocation::relocate_member;

        let this = (self as *mut Self).cast::<c_void>();
        let data = unsafe { relocate_member::<LIGHT_RUNTIME_DATA>(this, 0x110, 0x138) }?;
        if data.is_null() || !data.is_aligned() {
            todo!()
        };

        Ok(unsafe { &mut *data })
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

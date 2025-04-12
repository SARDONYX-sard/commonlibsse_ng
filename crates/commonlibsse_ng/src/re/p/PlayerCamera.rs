mod camera_state;
mod runtime_data;
mod runtime_data2;

pub use self::camera_state::{
    CameraState, CameraStateSE, CameraStateSE_CEnum, CameraStateVR, CameraStateVR_CEnum,
};
pub use self::runtime_data::{RUNTIME_DATA, Unk120, VR_RUNTIME_DATA};
pub use self::runtime_data2::RUNTIME_DATA2;

use crate::re::BSPointerHandle::ActorHandle;
use crate::re::TESCamera::{TESCamera, TESCameraVtbl};
use crate::re::offsets_rtti::RTTI_PlayerCamera;
use crate::re::offsets_vtable::VTABLE_PlayerCamera;
use crate::rel::id::VariantID;
use crate::rel::module::is_vr;
use crate::rel::relocation::{RelocationError, relocate_member, relocate_member_mut};

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerCamera {
    pub __base: TESCamera,         // 0x00
    pub pad39: u8,                 // 0x39
    pub pad3A: u16,                // 0x3A
    pub cameraTarget: ActorHandle, // 0x3C
}
const _: () = assert!(std::mem::size_of::<PlayerCamera>() == 0x40);

/// Simplify with macros due to the large amount of RUNTIME data.
macro_rules! define_runtime_getters {
    (
        $( {
            fn_name: $getter:ident,
            fn_name_mut: $getter_mut:ident,
            type: $data_type:ty,
            se_ae_offset: $se_ae_offset:expr,
            vr_offset: $vr_offset:expr
        } ),* $(,)?
    ) => {
        $(
            /// Gets fields whose offset is determined at runtime.
            ///
            /// # Errors
            /// This function may return an error if the module's runtime is not available or if any error occurs while fetching the runtime state.
            /// Specifically, it calls `ModuleState::map_active`, which could result in an error.
            #[inline]
            pub fn $getter(&self) -> Result<&$data_type, RelocationError> {
                relocate_member(self, $se_ae_offset, $vr_offset)
            }

            /// Gets mutable fields whose offset is determined at runtime.
            ///
            /// # Errors
            /// This function may return an error if the module's runtime is not available or if any error occurs while fetching the runtime state.
            /// Specifically, it calls `ModuleState::map_active_mut`, which could result in an error.
            #[inline]
            pub fn $getter_mut(&mut self) -> Result<&mut $data_type, RelocationError> {
                relocate_member_mut(self, $se_ae_offset, $vr_offset)
            }
        )*
    };
}

impl PlayerCamera {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_PlayerCamera;

    /// Address & offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_PlayerCamera;

    /// Returns the singleton instance of `Self`.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut *mut PlayerCamera",
        default = "None",
        deref_once,
        id(se = 514642, ae = 400802)
    )]
    pub fn get_singleton() -> Option<&'static PlayerCamera> {
        |deref_type: DerefType| unsafe { deref_type.as_ref() }
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 49858, ae_id = 50790)]
    #[inline]
    pub fn force_first_person(&mut self) -> bool {
        if is_vr() {
            return false;
        }
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 49863, ae_id = 50796)]
    #[inline]
    pub fn force_third_person(&mut self) -> bool {
        if is_vr() {
            return false;
        }
    }

    #[inline]
    pub fn is_in_bleedout_mode(&self) -> bool {
        let is_vr = is_vr();
        let state = if is_vr {
            CameraStateVR::VRBleedout.to_state()
        } else {
            CameraStateSE::Bleedout.to_state()
        };
        self.q_camera_equals(state, is_vr)
    }

    #[inline]
    pub fn is_in_first_person(&self) -> bool {
        self.q_camera_equals(CameraStateSE::FirstPerson.to_state(), is_vr())
    }

    #[inline]
    pub fn is_in_free_camera_mode(&self) -> bool {
        self.q_camera_equals(CameraStateSE::Free.to_state(), is_vr())
    }

    #[inline]
    pub fn is_in_third_person(&self) -> bool {
        let is_vr = is_vr();
        let state = if is_vr {
            CameraStateVR::VRThirdPerson.to_state()
        } else {
            CameraStateSE::ThirdPerson.to_state()
        };
        self.q_camera_equals(state, is_vr)
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 49947, ae_id = 50880)]
    #[inline]
    pub fn push_camera_state(&mut self, state: CameraState) {}

    #[inline]
    pub fn q_camera_equals(&self, camera_state: CameraState, is_vr: bool) -> bool {
        if self.__base.currentState.is_null() {
            return false;
        }

        if is_vr {
            match self.get_vr_runtime_data() {
                Ok(runtime_data) => {
                    if let Some(state_index) = camera_state.as_vr() {
                        return !runtime_data.cameraStates[state_index as usize].is_null();
                    };
                    #[cfg(feature = "tracing")]
                    tracing::warn!(
                        "CameraState index for VR is 0..=13 is expected, but got `{}`.",
                        camera_state.0
                    );
                }
                Err(_err) => {
                    #[cfg(feature = "tracing")]
                    tracing::error!("{_err}");
                }
            }
            return false;
        }

        // SE, AE
        match self.get_runtime_data() {
            Ok(runtime_data) => {
                if let Some(state_index) = camera_state.as_se() {
                    return !runtime_data.cameraStates[state_index as usize].is_null();
                }
                #[cfg(feature = "tracing")]
                tracing::warn!(
                    "CameraState index for SE/AE is 0..=12 is expected, but got `{}`.",
                    camera_state.0
                );
            }
            Err(_err) => {
                #[cfg(feature = "tracing")]
                tracing::error!("{_err}");
            }
        }
        false
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 49876, ae_id = 50809)]
    #[inline]
    pub fn toggle_free_camera_mode(&mut self, freeze_time: bool) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 49908, ae_id = 50841)]
    #[inline]
    pub fn update_third_person(&mut self, weapon_drawn: bool) {}

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Runtime data(The offset is determined at runtime, and the field is accessed via a method.)

    define_runtime_getters! {
        {
            fn_name: get_runtime_data,
            fn_name_mut: get_runtime_data_mut,
            type: VR_RUNTIME_DATA,
            se_ae_offset: 0x40,
            vr_offset: 0
        },
        {
            fn_name: get_runtime_data2,
            fn_name_mut: get_runtime_data2_mut,
            type: RUNTIME_DATA2,
            se_ae_offset: 0x13C,
            vr_offset: 0x158
        },
        {
            fn_name: get_vr_runtime_data,
            fn_name_mut: get_vr_runtime_data_mut,
            type: VR_RUNTIME_DATA,
            se_ae_offset: 0x0,
            vr_offset: 0x40
        },
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PlayerCameraVtbl {
    pub __base: TESCameraVtbl, // 0x00
}
const _: () = {
    const VFUNC_COUNT: usize = 0x3;

    const EXPECTED_SIZE: usize = VFUNC_COUNT * core::mem::size_of::<usize>();
    assert!(core::mem::size_of::<PlayerCameraVtbl>() == EXPECTED_SIZE);
};

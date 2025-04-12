/// SE, AE
///
/// These indicate the `TESCameraState` index.
#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum CameraStateSE {
    FirstPerson = 0,
    AutoVanity,
    VATS,
    Free,
    IronSights,
    Furniture,
    PCTransition,
    Tween,
    Animated,
    ThirdPerson,
    Mount,
    Bleedout,
    Dragon,
}

impl CameraStateSE {
    /// Into `CameraState`([`u32`])
    #[inline]
    pub const fn to_state(self) -> CameraState {
        CameraState(self as u32)
    }
}
impl From<CameraStateSE> for CameraState {
    #[inline]
    fn from(value: CameraStateSE) -> Self {
        Self(value as u32)
    }
}

/// VR has VR in between Animated and ThirdPerson
///
/// These indicate the `TESCameraState` index.
#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum CameraStateVR {
    FirstPerson = 0,
    AutoVanity,
    VATS,
    Free,
    IronSights,
    Furniture,
    PCTransition,
    Tween,
    Animated,
    VR = 9,
    VRThirdPerson,
    VRMount,
    VRBleedout,
    VRDragon,
}

impl CameraStateVR {
    /// Into `CameraState`([`u32`])
    #[inline]
    pub const fn to_state(self) -> CameraState {
        CameraState(self as u32)
    }
}

impl From<CameraStateVR> for CameraState {
    #[inline]
    fn from(value: CameraStateVR) -> Self {
        Self(value as u32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct CameraState(pub u32);
const _: () = assert!(core::mem::size_of::<CameraState>() == 0x4);

impl CameraState {
    /// Try to cast valid enum for SE, AE.
    #[inline]
    pub const fn as_se(&self) -> Option<CameraStateSE> {
        CameraStateSE_CEnum(self.0).to_enum()
    }

    /// Try to cast valid enum for VR.
    #[inline]
    pub const fn as_vr(&self) -> Option<CameraStateVR> {
        CameraStateVR_CEnum(self.0).to_enum()
    }
}

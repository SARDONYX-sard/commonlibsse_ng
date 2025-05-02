#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum INPUT_DEVICE_SE {
    Keyboard = 0,
    Mouse = 1,
    Gamepad = 2,
    FlatVirtualKeyboard = 3,
}
const _: () = assert!(INPUT_DEVICE_SE_CEnum::count() == 4);

#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum INPUT_DEVICE_VR {
    Keyboard = 0,
    Mouse = 1,
    Gamepad = 2,
    VivePrimary = 3,
    ViveSecondary = 4,
    OculusPrimary = 5,
    OculusSecondary = 6,
    WMRPrimary = 7,
    WMRSecondary = 8,
    VRVirtualKeyboard = 9,
}
const _: () = assert!(INPUT_DEVICE_VR_CEnum::count() == 10);

/// `INPUT_DEVICE` to store the raw value.
///
/// This is a generic type that exists because the correct value varies depending on the SE, AE , VR environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct INPUT_DEVICE(pub u32);
const _: () = assert!(core::mem::size_of::<INPUT_DEVICE>() == 0x4);

impl INPUT_DEVICE {
    /// Try to cast valid enum for SE, AE.
    #[inline]
    pub const fn as_se(&self) -> Option<INPUT_DEVICE_SE> {
        INPUT_DEVICE_SE_CEnum(self.0).to_enum()
    }

    /// Try to cast valid enum for VR.
    #[inline]
    pub const fn as_vr(&self) -> Option<INPUT_DEVICE_VR> {
        INPUT_DEVICE_VR_CEnum(self.0).to_enum()
    }
}

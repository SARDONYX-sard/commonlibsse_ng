use crate::re::offsets_rtti::RTTI_BSWin32GamepadDevice;
use crate::re::offsets_vtable::VTABLE_BSWin32GamepadDevice;
use crate::rel::id::VariantID;
use windows::Win32::UI::Input::XboxController::XINPUT_STATE;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct BSWin32GamepadDevice {
    pub __base: [u8; 0xd8], // BSPCGamepadDeviceDelegate

    previousState: XINPUT_STATE, // 0x0D8
    previousLT: f32,             // 0x0E8
    previousRT: f32,             // 0x0EC
    previousLX: f32,             // 0x0F0
    previousLY: f32,             // 0x0F4
    previousRX: f32,             // 0x0F8
    previousRY: f32,             // 0x0FC
    currentState: XINPUT_STATE,  // 0x100
    currentLT: f32,              // 0x110
    currentRT: f32,              // 0x114
    currentLX: f32,              // 0x118
    currentLY: f32,              // 0x11C
    currentRX: f32,              // 0x120
    currentRY: f32,              // 0x124
}
const _: () = assert!(core::mem::size_of::<BSWin32GamepadDevice>() == 0x128);

impl BSWin32GamepadDevice {
    /// Address & offset of RTTI for `BSWin32GamepadDevice`.
    pub const RTTI: VariantID = RTTI_BSWin32GamepadDevice;

    /// Address & offset of Virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_BSWin32GamepadDevice;
}

/// Button masks for wButtons
#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum Key {
    Up = 0x0001,
    Down = 0x0002,
    Left = 0x0004,
    Right = 0x0008,
    Start = 0x0010,
    Back = 0x0020,
    LeftThumb = 0x0040,
    RightThumb = 0x0080,
    LeftShoulder = 0x0100,
    RightShoulder = 0x0200,
    A = 0x1000,
    B = 0x2000,
    X = 0x4000,
    Y = 0x8000,

    // arbitrary values
    // IDs meant to be used with ButtonEvent
    LeftTrigger = 0x0009,
    RightTrigger = 0x000A,

    // IDs meant to be used with ThumbStickEvent
    LeftStick = 0x000B,
    RightStick = 0x000C,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ButtonState {
    up = 0x0001,
    down = 0x0002,
    left = 0x0004,
    right = 0x0008,
    start = 0x0010,
    back = 0x0020,
    leftThumb = 0x0040,
    rightThumb = 0x0080,
    leftShoulder = 0x0100,
    /// skip over 2 bits (XInput documentation says the state of these two bits are undefined)
    rightShoulder = 0x0200,
    a = 0x1000,
    b = 0x2000,
    x = 0x4000,
    y = 0x8000,
}

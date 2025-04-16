// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/SKSE/InputMap.h
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/SKSE/InputMap.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::borrow::Cow;

use windows::Win32::Devices::HumanInterfaceDevice::{
    DIK_DELETE, DIK_DIVIDE, DIK_DOWNARROW, DIK_END, DIK_HOME, DIK_INSERT, DIK_LEFTARROW,
    DIK_NUMPADENTER, DIK_PGDN, DIK_PGUP, DIK_RALT, DIK_RCONTROL, DIK_RIGHTARROW, DIK_UPARROW,
};
use windows::Win32::UI::Input::KeyboardAndMouse::GetKeyNameTextW;
use windows::Win32::UI::Input::XboxController::{
    XINPUT_GAMEPAD_A, XINPUT_GAMEPAD_B, XINPUT_GAMEPAD_BACK, XINPUT_GAMEPAD_BUTTON_FLAGS,
    XINPUT_GAMEPAD_DPAD_DOWN, XINPUT_GAMEPAD_DPAD_LEFT, XINPUT_GAMEPAD_DPAD_RIGHT,
    XINPUT_GAMEPAD_DPAD_UP, XINPUT_GAMEPAD_LEFT_SHOULDER, XINPUT_GAMEPAD_LEFT_THUMB,
    XINPUT_GAMEPAD_RIGHT_SHOULDER, XINPUT_GAMEPAD_RIGHT_THUMB, XINPUT_GAMEPAD_START,
    XINPUT_GAMEPAD_X, XINPUT_GAMEPAD_Y,
};
use windows::core::HSTRING;

pub const MACRO_KEYBOARD_OFFSET: u32 = 0; // Not actually used, just for self-documentation
pub const MACRO_NUM_KEYBOARD_KEYS: u32 = 256;

pub const MACRO_MOUSE_BUTTON_OFFSET: u32 = MACRO_NUM_KEYBOARD_KEYS; // 256
pub const MACRO_NUM_MOUSE_BUTTONS: u32 = 8;

pub const MACRO_MOUSE_WHEEL_OFFSET: u32 = MACRO_MOUSE_BUTTON_OFFSET + MACRO_NUM_MOUSE_BUTTONS; // 264
pub const MACRO_MOUSE_WHEEL_DIRECTIONS: u32 = 2;

pub const MACRO_GAMEPAD_OFFSET: u32 = MACRO_MOUSE_WHEEL_OFFSET + MACRO_MOUSE_WHEEL_DIRECTIONS; // 266
pub const MACRO_NUM_GAMEPAD_BUTTONS: u32 = 16;

pub const MAX_MACROS: u32 = MACRO_GAMEPAD_OFFSET + MACRO_NUM_GAMEPAD_BUTTONS; // 282

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GamepadButtonOffset {
    DpadUp = MACRO_GAMEPAD_OFFSET,
    DpadDown,
    DpadLeft,
    DpadRight,
    Start,
    Back,
    LeftThumb,
    RightThumb,
    LeftShoulder,
    RightShoulder,
    A,
    B,
    X,
    Y,
    Lt,
    Rt, // 281
}
const _: () = assert!(GamepadButtonOffset::Rt as u32 == 281);

impl GamepadButtonOffset {
    const fn from_xinput(key_mask: XINPUT_GAMEPAD_BUTTON_FLAGS) -> Option<Self> {
        // if (RE::ControlMap::GetSingleton()->GetGamePadType() == RE::PC_GAMEPAD_TYPE::kOrbis) {
        // 	keyMask = ScePadOffsetToXInput(keyMask);
        // }
        Some(match key_mask {
            XINPUT_GAMEPAD_DPAD_UP => Self::DpadUp,
            XINPUT_GAMEPAD_DPAD_DOWN => Self::DpadDown,
            XINPUT_GAMEPAD_DPAD_LEFT => Self::DpadLeft,
            XINPUT_GAMEPAD_DPAD_RIGHT => Self::DpadRight,
            XINPUT_GAMEPAD_START => Self::Start,
            XINPUT_GAMEPAD_BACK => Self::Back,
            XINPUT_GAMEPAD_LEFT_THUMB => Self::LeftThumb,
            XINPUT_GAMEPAD_RIGHT_THUMB => Self::RightThumb,
            XINPUT_GAMEPAD_LEFT_SHOULDER => Self::LeftShoulder,
            XINPUT_GAMEPAD_RIGHT_SHOULDER => Self::RightShoulder,
            XINPUT_GAMEPAD_A => Self::A,
            XINPUT_GAMEPAD_B => Self::B,
            XINPUT_GAMEPAD_X => Self::X,
            XINPUT_GAMEPAD_Y => Self::Y,

            other => {
                match other.0 {
                    0x9 => Self::Lt,  // Left Trigger game-defined ID
                    0xA => Self::Rt,  // Right Trigger game-defined ID
                    _ => return None, // Invalid return MAX_MACROS
                }
            }
        })
    }

    pub const fn to_xinput(&self) -> XINPUT_GAMEPAD_BUTTON_FLAGS {
        match *self {
            Self::DpadUp => XINPUT_GAMEPAD_DPAD_UP,
            Self::DpadDown => XINPUT_GAMEPAD_DPAD_DOWN,
            Self::DpadLeft => XINPUT_GAMEPAD_DPAD_LEFT,
            Self::DpadRight => XINPUT_GAMEPAD_DPAD_RIGHT,
            Self::Start => XINPUT_GAMEPAD_START,
            Self::Back => XINPUT_GAMEPAD_BACK,
            Self::LeftThumb => XINPUT_GAMEPAD_LEFT_THUMB,
            Self::RightThumb => XINPUT_GAMEPAD_RIGHT_THUMB,
            Self::LeftShoulder => XINPUT_GAMEPAD_LEFT_SHOULDER,
            Self::RightShoulder => XINPUT_GAMEPAD_RIGHT_SHOULDER,
            Self::A => XINPUT_GAMEPAD_A,
            Self::B => XINPUT_GAMEPAD_B,
            Self::X => XINPUT_GAMEPAD_X,
            Self::Y => XINPUT_GAMEPAD_Y,
            Self::Lt => XINPUT_GAMEPAD_BUTTON_FLAGS(0x9), // Custom mapping for LT
            Self::Rt => XINPUT_GAMEPAD_BUTTON_FLAGS(0xA), // Custom mapping for RT
        }

        // if (RE::ControlMap::GetSingleton()->GetGamePadType() == RE::PC_GAMEPAD_TYPE::kOrbis) {
        // 	keyMask = XInputToScePadOffset(keyMask);
        // }
        // return keyMask;
    }

    pub const fn to_str(&self) -> &'static str {
        match self {
            Self::DpadUp => "Gamepad DPad Up",
            Self::DpadDown => "Gamepad DPad Down",
            Self::DpadLeft => "Gamepad DPad Left",
            Self::DpadRight => "Gamepad DPad Right",
            Self::Start => "Gamepad Start",
            Self::Back => "Gamepad Back",
            Self::LeftThumb => "Gamepad Left Thumb",
            Self::RightThumb => "Gamepad Right Thumb",
            Self::LeftShoulder => "Gamepad Left Shoulder",
            Self::RightShoulder => "Gamepad Right Shoulder",
            Self::A => "Gamepad A",
            Self::B => "Gamepad B",
            Self::X => "Gamepad X",
            Self::Y => "Gamepad Y",
            Self::Lt => "Gamepad LT",
            Self::Rt => "Gamepad RT",
        }
    }
}

/// PS4
///
/// Constants for SCE Pad buttons
#[repr(u32)]
pub enum ScePadButton {
    Share = 0x00000001,
    L3 = 0x00000002,
    R3 = 0x00000004,
    Options = 0x00000008,
    Up = 0x00000010,
    Right = 0x00000020,
    Down = 0x00000040,
    Left = 0x00000080,
    L2 = 0x00000100,
    R2 = 0x00000200,
    L1 = 0x00000400,
    R1 = 0x00000800,
    Triangle = 0x00001000,
    Circle = 0x00002000,
    Cross = 0x00004000,
    Square = 0x00008000,
    Playstation = 0x00010000,
    TouchPad = 0x00100000,
    Intercepted = 0x80000000,
}

impl ScePadButton {
    pub const fn from_xinput(key_mask: XINPUT_GAMEPAD_BUTTON_FLAGS) -> Option<Self> {
        Some(match key_mask {
            XINPUT_GAMEPAD_DPAD_UP => Self::Up,
            XINPUT_GAMEPAD_DPAD_DOWN => Self::Down,
            XINPUT_GAMEPAD_DPAD_LEFT => Self::Left,
            XINPUT_GAMEPAD_DPAD_RIGHT => Self::Right,
            XINPUT_GAMEPAD_START => Self::Options,
            XINPUT_GAMEPAD_BACK => Self::TouchPad,
            XINPUT_GAMEPAD_LEFT_THUMB => Self::L3,
            XINPUT_GAMEPAD_RIGHT_THUMB => Self::R3,
            XINPUT_GAMEPAD_LEFT_SHOULDER => Self::L1,
            XINPUT_GAMEPAD_RIGHT_SHOULDER => Self::R1,
            XINPUT_GAMEPAD_A => Self::Cross,
            XINPUT_GAMEPAD_B => Self::Circle,
            XINPUT_GAMEPAD_X => Self::Square,
            XINPUT_GAMEPAD_Y => Self::Triangle,
            _ => return None,
        })
    }

    pub fn to_xinput(&self) -> Option<XINPUT_GAMEPAD_BUTTON_FLAGS> {
        Some(match *self {
            Self::Up => XINPUT_GAMEPAD_DPAD_UP,
            Self::Down => XINPUT_GAMEPAD_DPAD_DOWN,
            Self::Left => XINPUT_GAMEPAD_DPAD_LEFT,
            Self::Right => XINPUT_GAMEPAD_DPAD_RIGHT,
            Self::Options => XINPUT_GAMEPAD_START,
            Self::TouchPad => XINPUT_GAMEPAD_BACK,
            Self::L3 => XINPUT_GAMEPAD_LEFT_THUMB,
            Self::R3 => XINPUT_GAMEPAD_RIGHT_THUMB,
            Self::L1 => XINPUT_GAMEPAD_LEFT_SHOULDER,
            Self::R1 => XINPUT_GAMEPAD_RIGHT_SHOULDER,
            Self::Share => todo!(),
            Self::L2 => todo!(),
            Self::R2 => todo!(),
            Self::Triangle => XINPUT_GAMEPAD_Y,
            Self::Circle => XINPUT_GAMEPAD_B,
            Self::Cross => XINPUT_GAMEPAD_A,
            Self::Square => XINPUT_GAMEPAD_X,
            Self::Playstation | Self::Intercepted => return None,
        })
    }
}

pub fn get_key_name(key_code: u32) -> Cow<'static, str> {
    if (MACRO_MOUSE_BUTTON_OFFSET..MACRO_GAMEPAD_OFFSET).contains(&key_code) {
        get_mouse_button_name(key_code).unwrap_or("Unknown").into()
    } else if (MACRO_GAMEPAD_OFFSET..MAX_MACROS).contains(&key_code) {
        GamepadButtonOffset::from_xinput(XINPUT_GAMEPAD_BUTTON_FLAGS(key_code as u16))
            .map_or("Unknown", |input| input.to_str())
            .into()
    } else {
        get_keyboard_key_name(key_code).into()
    }
}

fn get_keyboard_key_name(key_code: u32) -> String {
    let mut scancode = key_code & 0xFF;

    scancode = match scancode {
        DIK_NUMPADENTER => 0x11C,
        DIK_RCONTROL => 0x11D,
        DIK_DIVIDE => 0x135,
        DIK_RALT => 0x138,
        DIK_HOME => 0x147,
        DIK_UPARROW => 0x148,
        DIK_PGUP => 0x149,
        DIK_LEFTARROW => 0x14B,
        DIK_RIGHTARROW => 0x14D,
        DIK_END => 0x14F,
        DIK_DOWNARROW => 0x150,
        DIK_PGDN => 0x151,
        DIK_INSERT => 0x152,
        DIK_DELETE => 0x153,
        _ => scancode,
    };

    let mut l_param = (scancode << 16) as i32;
    if scancode == 0x45 {
        l_param |= 1 << 24;
    }

    let mut buffer = [0_u16; 256];
    let length = unsafe { GetKeyNameTextW(l_param, buffer.as_mut_slice()) };
    if length > 0 {
        return HSTRING::from_wide(&buffer[..length as usize]).to_string();
    }

    String::new()
}

const fn get_mouse_button_name(key_code: u32) -> Option<&'static str> {
    Some(match key_code {
        256 => "Left Mouse Button",
        257 => "Right Mouse Button",
        258 => "Middle Mouse Button",
        259 => "Mouse Button 3",
        260 => "Mouse Button 4",
        261 => "Mouse Button 5",
        262 => "Mouse Button 6",
        263 => "Mouse Button 7",
        264 => "Mouse Wheel Up",
        265 => "Mouse Wheel Down",
        _ => return None,
    })
}

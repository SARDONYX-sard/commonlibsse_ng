// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/SKSE/Interfaces.h
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/SKSE/Interfaces.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::ffi::c_void;

type MessagingCallback = fn(msg: &Message);

pub struct Message {
    pub sender: &'static str,
    pub msg_type: u32,
    pub data_len: u32,
    pub data: *mut c_void,
}

pub struct MessagingInterface;
impl MessagingInterface {
    pub const VERSION: u32 = 2;

    pub fn version(&self) -> u32 {
        Self::VERSION
    }

    pub fn register_listener(&self, callback: MessagingCallback) -> bool {
        // Implementation
        true
    }
}

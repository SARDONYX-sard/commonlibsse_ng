// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/SKSE/Interfaces.h
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/SKSE/Interfaces.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::ffi::{c_void, CStr};

use crate::skse::{api::get_plugin_handle, impls::stab::SKSEMessagingInterface};

type MessagingCallback = fn(msg: &Message);

#[repr(u32)]
enum MessageType {
    PostLoad,
    PostPostLoad,
    PreLoadGame,
    PostLoadGame,
    SaveGame,
    DeleteGame,
    InputLoaded,
    NewGame,
    DataLoaded,

    Total,
}

#[repr(u32)]
pub enum Dispatcher {
    ModEvent = 0,
    CameraEvent,
    CrosshairEvent,
    ActionEvent,
    NiNodeUpdateEvent,

    Total,
}

pub struct Message {
    pub sender: &'static str,
    pub msg_type: u32,
    pub data_len: u32,
    pub data: *mut c_void,
}

pub struct MessagingInterface {
    address: *const SKSEMessagingInterface,
}

impl MessagingInterface {
    pub const VERSION: u32 = 2;

    pub fn version(&self) -> u32 {
        unsafe { (*self.get_proxy()).interface_version }
    }

    pub fn dispatch(
        &self,
        message_type: MessageType,
        data: *mut c_void,
        data_len: u32,
        receiver: &CStr,
    ) -> bool {
        let result = unsafe {
            ((*self.get_proxy()).dispatch)(
                get_plugin_handle(),
                message_type as u32,
                data,
                data_len,
                receiver.as_ptr(),
            )
        };
        if !result {
            let receiver = if receiver.is_empty() {
                "all listeners"
            } else {
                receiver.to_str().unwrap_or_default()
            };
            tracing::warn!("Failed to dispatch message to {receiver }");
        }

        result
    }

    pub fn get_event_dispatcher(&self, dispatcher_id: Dispatcher) -> *mut c_void {
        unsafe { ((*self.get_proxy()).get_event_dispatcher)(dispatcher_id as u32) }
    }

    pub fn register_listener(&self, callback: MessagingCallback) -> bool {
        self.register_listener2(c"SKSE", callback)
    }

    pub fn register_listener2(&self, sender: &CStr, callback: MessagingCallback) -> bool {
        let void_callback = (callback as *mut MessagingCallback).cast::<c_void>();
        let result = unsafe {
            ((*self.get_proxy()).register_listener)(
                get_plugin_handle(),
                sender.as_ptr(),
                void_callback,
            )
        };

        if !result {
            tracing::warn!(
                "Failed to register listener for sender: {}",
                sender.to_string_lossy()
            );
        }

        result
    }

    fn get_proxy(&self) -> *const SKSEMessagingInterface {
        assert!(!self.address.is_null());
        self.address.cast()
    }
}

// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/SKSE/Interfaces.h
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/SKSE/Interfaces.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::ffi::{CStr, c_char, c_void};

use crate::skse::{api::get_plugin_handle, impls::stab::SKSEMessagingInterface};

type MessagingCallback = fn(msg: &Message);

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MessageType {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dispatcher {
    ModEvent = 0,
    CameraEvent,
    CrosshairEvent,
    ActionEvent,
    NiNodeUpdateEvent,

    Total,
}

#[derive(Clone)]
#[repr(C)]
pub struct Message {
    pub sender: *const c_char,
    pub msg_type: MessageType,
    pub data_len: u32,
    pub data: *mut c_void,
}

// # Why does this struct need to implement Debug manually?
// In the case of `*const c_char`, Debug is a memory address, **which is difficult to debug**.
// Therefore, implement it manually and display the string.
impl core::fmt::Debug for Message {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Message")
            .field("sender", &unsafe { CStr::from_ptr(self.sender) })
            .field("msg_type", &self.msg_type)
            .field("data_len", &self.data_len)
            .field("data", &self.data)
            .finish()
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct MessagingInterface(&'static SKSEMessagingInterface);

impl MessagingInterface {
    pub const VERSION: u32 = 2;

    #[inline]
    pub(crate) const fn new(interface: &'static SKSEMessagingInterface) -> Self {
        Self(interface)
    }

    #[inline]
    pub const fn version(&self) -> u32 {
        self.get_inner().interfaceVersion
    }

    #[inline]
    /// # Safety
    pub unsafe fn dispatch(
        &self,
        message_type: MessageType,
        data: *mut c_void,
        data_len: u32,
        receiver: &CStr,
    ) -> bool {
        let result = unsafe {
            (self.get_inner().Dispatch)(
                get_plugin_handle(),
                message_type as u32,
                data,
                data_len,
                receiver.as_ptr(),
            )
        };
        if !result {
            let _receiver = if receiver.is_empty() {
                "all listeners"
            } else {
                receiver.to_str().unwrap_or_default()
            };
            #[cfg(feature = "tracing")]
            tracing::warn!("Failed to dispatch message to {_receiver}");
        }

        result
    }

    #[inline]
    pub fn get_event_dispatcher(&self, dispatcher_id: Dispatcher) -> *mut c_void {
        unsafe { (self.get_inner().GetEventDispatcher)(dispatcher_id as u32) }
    }

    /// Listen to SKSE's in-game events(e.g. load save)
    #[inline]
    pub fn register_listener(&self, callback: MessagingCallback) -> bool {
        self.register_listener2(c"SKSE", callback)
    }

    /// Listen to sender name in-game events(e.g. load save)
    pub fn register_listener2(&self, sender: &CStr, callback: MessagingCallback) -> bool {
        #[allow(clippy::fn_to_numeric_cast_any)]
        let void_callback = (callback as *mut MessagingCallback).cast::<c_void>();
        let result = unsafe {
            (self.get_inner().RegisterListener)(get_plugin_handle(), sender.as_ptr(), void_callback)
        };

        if !result {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                "Failed to register listener for sender: {}",
                sender.to_string_lossy()
            );
        }

        result
    }

    #[inline]
    const fn get_inner(&self) -> &'static SKSEMessagingInterface {
        self.0
    }
}

// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/SKSE/Interfaces.h
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/SKSE/Interfaces.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Rust bindings for the SKSE Messaging Interface
//!
//! This module provides Rust representations of the SKSE messaging system, which allows plugins to communicate with each other.
//! It includes message types, dispatchers, and the main `MessagingInterface` wrapper.
//!
//! # References
//! - [Original C++ Code](https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/SKSE/Interfaces.h)
//! - [C++ Implementation](https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/SKSE/Interfaces.cpp)
use crate::skse::{
    api::{ApiStorageError, get_plugin_handle},
    impls::stab::SKSEMessagingInterface,
};
use std::{
    borrow::Cow,
    ffi::{CStr, c_char, c_void},
};

/// Represents the different types of messages that can be sent or received through SKSE's messaging system.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MessageType {
    /// Fired after all plugins are loaded.
    PostLoad,
    /// Fired after all `PostLoad` events have completed.
    PostPostLoad,
    /// Fired before loading a game save.
    PreLoadGame,
    /// Fired after loading a game save.
    PostLoadGame,
    /// Fired before saving a game.
    SaveGame,
    /// Fired before deleting a game save.
    DeleteGame,
    /// Fired when the input system is loaded.
    InputLoaded,
    /// Fired when starting a new game.
    NewGame,
    /// Fired after all game data has loaded.
    DataLoaded,

    /// Placeholder for the total number of message types.
    Total,
}

/// Represents the different event dispatchers that SKSE provides.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dispatcher {
    /// Dispatcher for mod events.
    ModEvent = 0,
    /// Dispatcher for camera events.
    CameraEvent,
    /// Dispatcher for crosshair events.
    CrosshairEvent,
    /// Dispatcher for action events.
    ActionEvent,
    /// Dispatcher for NiNode update events.
    NiNodeUpdateEvent,

    /// Placeholder for the total number of dispatchers.
    Total,
}

/// Represents a message sent through the SKSE messaging system.
#[derive(Clone)]
#[repr(C)]
pub struct Message {
    /// The name of the sender as a C string.
    pub sender: *const c_char,
    /// The type of message.
    pub msg_type: MessageType,
    /// The length of the data buffer.
    pub data_len: u32,
    /// Pointer to the message data.
    pub data: *mut c_void,
}

// # Why does this struct need to implement Debug manually?
// In the case of `*const c_char`, Debug is a memory address, **which is difficult to debug**.
// Therefore, implement it manually and display the string.
impl core::fmt::Debug for Message {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let data: Cow<'static, str> = if self.data_len == 0 {
            "".into()
        } else if !self.data.is_null() && self.data.cast::<u8>().is_aligned() {
            if crate::rex::win32::is_valid_range(self.data.cast::<u8>(), self.data_len as usize) {
                String::from_utf8_lossy(unsafe {
                    core::slice::from_raw_parts(self.data.cast::<u8>(), self.data_len as usize)
                })
            } else {
                "inaccessible ptr".into()
            }
        } else {
            "null/unaligned ptr".into()
        };

        f.debug_struct("Message")
            .field("sender", &unsafe { CStr::from_ptr(self.sender) })
            .field("msg_type", &self.msg_type)
            .field("data_len", &self.data_len)
            .field("data_ptr", &self.data)
            .field("data", &data)
            .finish()
    }
}

/// APIs that enable data to be sent and received between plugins.
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct MessagingInterface(&'static SKSEMessagingInterface);

impl MessagingInterface {
    /// The version number of the messaging interface.
    pub const VERSION: u32 = 2;

    /// Creates a new `MessagingInterface` instance from the raw SKSE interface.
    #[inline]
    pub(crate) const fn new(interface: &'static SKSEMessagingInterface) -> Self {
        Self(interface)
    }

    /// Returns the version number of the messaging interface.
    #[inline]
    pub const fn version(&self) -> u32 {
        self.0.interfaceVersion
    }

    /// Dispatches a message to SKSE listeners.
    ///
    /// # Errors
    /// If the internal global API storage is uninitialized because forgot to call `skse::init`
    #[inline]
    pub fn dispatch<T>(
        &self,
        message_type: MessageType,
        data: &mut T,
        data_len: u32,
        receiver: Option<&CStr>,
    ) -> Result<(), MessagingError> {
        unsafe { self.dispatch_raw(message_type, (data as *mut T).cast(), data_len, receiver) }
    }

    /// Dispatches a message to SKSE listeners.
    ///
    /// # Errors
    /// If the internal global API storage is uninitialized because forgot to call `skse::init`
    ///
    /// # Safety
    /// If the reference to the pointer pointing to data is valid.
    pub unsafe fn dispatch_raw(
        &self,
        message_type: MessageType,
        data: *mut c_void,
        data_len: u32,
        receiver: Option<&CStr>,
    ) -> Result<(), MessagingError> {
        let result = unsafe {
            (self.0.Dispatch)(
                get_plugin_handle()?,
                message_type as u32,
                data,
                data_len,
                receiver.map_or(core::ptr::null_mut(), |cstr| cstr.as_ptr()),
            )
        };
        if !result {
            return Err(MessagingError::DispatchFailed {
                message_type,
                receiver: receiver
                    .map_or("all listeners", |receiver| receiver.to_str().unwrap_or_default())
                    .to_string(),
            });
        }

        Ok(())
    }

    /// Gets the event dispatcher for a specific dispatcher id.
    #[inline]
    pub fn get_event_dispatcher(&self, dispatcher_id: Dispatcher) -> *mut c_void {
        unsafe { (self.0.GetEventDispatcher)(dispatcher_id as u32) }
    }

    /// Registers a listener for SKSE's in-game events (e.g., loading saves).
    ///
    /// # Errors
    /// If the internal global API storage is uninitialized because forgot to call `skse::init`
    ///
    /// # Event Data
    /// - `PreLoadGame`:  The name of the save data
    /// - `PostLoadGame`: Invalid ptr(data length 1)
    ///
    /// # Example
    ///
    /// ```rust:no_compile
    /// if let Some(messaging) = commonlibsse_ng::skse::api::get_messaging_interface() {
    ///     messaging.register_skse_listener(|message| {
    ///         #[cfg(feature = "tracing")]
    ///         tracing::info!("SKSE event: {message:#?}");
    ///     });
    /// }
    /// ```
    #[inline]
    pub fn register_skse_listener(&self, f: fn(msg: &Message)) -> Result<(), MessagingError> {
        self.register_listener(c"SKSE", f)
    }

    /// Registers a listener for a specific plugin's in-game events.
    ///
    /// # Errors
    /// If the internal global API storage is uninitialized because forgot to call `skse::init`
    pub fn register_listener(
        &self,
        sender: &CStr,
        f: fn(msg: &Message),
    ) -> Result<(), MessagingError> {
        #[allow(clippy::fn_to_numeric_cast_any)]
        let void_callback = (f as *mut fn(msg: &Message)).cast::<c_void>();
        let result = unsafe {
            (self.0.RegisterListener)(get_plugin_handle()?, sender.as_ptr(), void_callback)
        };

        if !result {
            return Err(MessagingError::RegisterListenerFailed {
                sender_name: sender.to_string_lossy().to_string(),
            });
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, snafu::Snafu)]
pub enum MessagingError {
    /// Failed to dispatch message to {receiver}, kind: {message_type:?}
    DispatchFailed { message_type: MessageType, receiver: String },

    /// Failed to register listener for sender: {sender_name}
    RegisterListenerFailed { sender_name: String },

    #[snafu(transparent)]
    ApiStorageError { source: ApiStorageError },
}

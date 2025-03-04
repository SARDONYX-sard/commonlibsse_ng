// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/SKSE/Interfaces.h
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/SKSE/Interfaces.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::rel::version::Version;
use crate::skse::impls::stab::{
    PluginHandle, PluginInfo, SKSEInterface, SKSEMessagingInterface, SKSEObjectInterface,
    SKSEPapyrusInterface, SKSEScaleformInterface, SKSESerializationInterface, SKSETaskInterface,
    SKSETrampolineInterface,
};
use crate::skse::interfaces::query::QueryInterface;
use core::ffi::{CStr, c_void};

/// Aimed at providing an API.
///
/// That is equivalent in memory layout to `SKSEInterface` and easy to use,
/// in order to allow it to be used instead of the argument `SKSEInterface` in the `SKSEPlugin_Load`
/// symbol.
#[derive(Debug)]
#[repr(transparent)]
pub struct LoadInterface(SKSEInterface);

/// # Safety
/// This can only be implemented by interfaces defined in SKSE.
unsafe trait QueryTarget {
    /// Cast to function table(struct)
    fn cast(ptr: *mut c_void) -> &'static Self;
    /// Get this query id
    fn id() -> u32;
}

macro_rules! impl_query_target {
    ($($t:ty => $id:expr),*) => {
        $(
            unsafe impl QueryTarget for $t {
                #[inline]
                fn cast(ptr: *mut c_void) -> &'static Self {
                    assert!(!ptr.is_null(), "SKSE interface query returned null");
                    unsafe { &*(ptr as *const Self) }
                }

                #[inline]
                fn id() -> u32 {
                    $id
                }
            }
        )*
    };
}

impl_query_target!(
    SKSEScaleformInterface => 1,
    SKSEPapyrusInterface => 2,
    SKSESerializationInterface => 3,
    SKSETaskInterface => 4,
    SKSEMessagingInterface => 5,
    SKSEObjectInterface => 6,
    SKSETrampolineInterface => 7
);

impl LoadInterface {
    /// Get the plugin handle (index of how many dlls SKSE has loaded) of this SKSE plugin dll.
    #[inline]
    pub fn get_plugin_handle(&self) -> PluginHandle {
        unsafe { (self.0.GetPluginHandle)() }
    }

    /// Get information about a plugin given its name.
    ///
    /// Returns a pointer to `PluginInfo` if found, otherwise `null`.
    #[inline]
    pub fn get_plugin_info(&self, name: &CStr) -> *const PluginInfo {
        unsafe { (self.0.GetPluginInfo)(name.as_ptr()) }
    }

    /// Get the release index of the plugin system.
    #[inline]
    pub fn get_release_index(&self) -> u32 {
        unsafe { (self.0.GetReleaseIndex)() }
    }

    /// Get a reference to the global variables for each interface.
    #[allow(private_bounds)]
    #[inline]
    pub fn query_interface<T: QueryTarget>(&self) -> &'static T {
        let fn_table = unsafe { (self.0.QueryInterface)(T::id()) };
        debug_assert!(!fn_table.is_null(), "SKSE interface query returned null");
        T::cast(fn_table)
    }
}

impl QueryInterface for LoadInterface {
    #[inline]
    fn editor_version(&self) -> u32 {
        self.0.editorVersion
    }

    #[inline]
    fn is_editor(&self) -> bool {
        self.0.isEditor != 0
    }

    #[inline]
    fn runtime_version(&self) -> Version {
        let packed = self.0.runtimeVersion;
        let major = ((packed & 0xFF000000) >> 24) as u16;
        let minor = ((packed & 0x00FF0000) >> 16) as u16;
        let revision = ((packed & 0x0000FFF0) >> 4) as u16;
        let build = (packed & 0x0000000F) as u16;
        Version::new(major, minor, revision, build)
    }

    #[inline]
    fn skse_version(&self) -> u32 {
        self.0.skseVersion
    }
}

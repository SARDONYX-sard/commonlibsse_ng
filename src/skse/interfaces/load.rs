// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/SKSE/Interfaces.h
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/SKSE/Interfaces.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::query::QueryInterface;
use crate::skse::impls::stab::{PluginHandle, PluginInfo};
use core::ffi::c_void;
use std::ffi::CStr;

#[repr(u32)]
pub enum LoadInterfaceEnum {
    Invalid = 0,
    ScaleForm,
    Papyrus,
    Serialization,
    Task,
    Messaging,
    Object,
    Trampoline,
    Total,
}

#[repr(C)]
pub struct LoadInterface {
    _base: QueryInterface,
}

impl LoadInterface {
    /// # Safety
    pub unsafe fn get_plugin_handle(&self) -> PluginHandle {
        let base = &*self._base.get_proxy();
        (base.get_plugin_handle)()
    }

    pub fn get_plugin_info(&self, name: &CStr) -> *const PluginInfo {
        let base = unsafe { &*self._base.get_proxy() };
        unsafe { (base.get_plugin_info)(name.as_ptr()).cast() }
    }

    pub fn get_release_index(&self) -> u32 {
        let base = unsafe { &*self._base.get_proxy() };
        unsafe { (base.get_release_index)() }
    }

    /// # Safety
    pub unsafe fn query_interface(&self, id: u32) -> *const c_void {
        let base = &*self._base.get_proxy();
        (base.query_interface)(id)
    }
}

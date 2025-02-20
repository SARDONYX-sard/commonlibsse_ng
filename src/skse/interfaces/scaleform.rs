// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/SKSE/Interfaces.h
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/SKSE/Interfaces.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::re::{GFxMovieView, GFxValue, InventoryEntryData};
use crate::skse::impls::stab::SKSEScaleformInterface;
use core::ffi::{c_void, CStr};

type RegCallback = fn(a_view: *mut GFxMovieView, a_root: *mut GFxValue) -> bool;
type RegInvCallback =
    fn(a_view: *mut GFxMovieView, a_object: *mut GFxValue, a_item: *mut InventoryEntryData);

#[derive(Debug)]
pub struct ScaleformInterface {
    address: *const u8,
}

impl ScaleformInterface {
    pub const VERSION: u32 = 2;

    pub fn version(&self) -> u32 {
        unsafe { (*self.get_proxy()).interface_version }
    }

    pub fn register(&self, callback: RegCallback, name: &CStr) -> bool {
        #[allow(clippy::fn_to_numeric_cast_any)]
        let void_callback = (callback as *mut RegInvCallback).cast::<c_void>();
        let result = unsafe { ((*self.get_proxy()).register)(name.as_ptr(), void_callback) };

        if !result {
            #[cfg(feature = "tracing")]
            tracing::error!("Failed to register scaleform callback: {:?}", name);
        };
        result
    }

    pub fn register_for_inventory(&self, callback: RegInvCallback) {
        #[allow(clippy::fn_to_numeric_cast_any)]
        let void_callback = (callback as *mut RegInvCallback).cast::<c_void>();
        unsafe { ((*self.get_proxy()).register_for_inventory)(void_callback) }
    }

    fn get_proxy(&self) -> *const SKSEScaleformInterface {
        assert!(!self.address.is_null());
        self.address.cast()
    }
}

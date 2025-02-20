// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/SKSE/Interfaces.h
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/SKSE/Interfaces.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::skse::impls::stab::{
    SKSEDelayFunctorManager, SKSEObjectInterface, SKSEObjectRegistry, SKSEPersistentObjectStorage,
};

#[derive(Debug)]
pub struct ObjectInterface {
    address: *const u8,
}

impl ObjectInterface {
    pub const VERSION: u32 = 2;

    pub fn version(&self) -> u32 {
        unsafe { (*self.get_proxy()).interface_version }
    }

    pub fn get_delay_functor_manager(&self) -> *mut SKSEDelayFunctorManager {
        unsafe { ((*self.get_proxy()).get_delay_functor_manager)() }
    }

    pub fn get_object_registry(&self) -> *mut SKSEObjectRegistry {
        unsafe { ((*self.get_proxy()).get_object_registry)() }
    }

    pub fn get_persistent_object_storage(&self) -> *mut SKSEPersistentObjectStorage {
        unsafe { ((*self.get_proxy()).get_persistent_object_storage)() }
    }

    fn get_proxy(&self) -> *const SKSEObjectInterface {
        assert!(!self.address.is_null());
        self.address.cast()
    }
}

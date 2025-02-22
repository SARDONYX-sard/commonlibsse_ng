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

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct ObjectInterface(&'static SKSEObjectInterface);

impl ObjectInterface {
    pub const VERSION: u32 = 2;

    #[inline]
    pub(crate) const fn new(interface: &'static SKSEObjectInterface) -> Self {
        Self(interface)
    }

    #[inline]
    pub const fn version(&self) -> u32 {
        self.0.interfaceVersion
    }

    #[inline]
    pub fn get_delay_functor_manager(&self) -> *mut SKSEDelayFunctorManager {
        unsafe { (self.0.GetDelayFunctorManager)() }
    }

    #[inline]
    pub fn get_object_registry(&self) -> *mut SKSEObjectRegistry {
        unsafe { (self.0.GetObjectRegistry)() }
    }

    #[inline]
    pub fn get_persistent_object_storage(&self) -> *mut SKSEPersistentObjectStorage {
        unsafe { (self.0.GetPersistentObjectStorage)() }
    }
}

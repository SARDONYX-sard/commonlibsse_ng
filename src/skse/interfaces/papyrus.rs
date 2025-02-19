// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/SKSE/Interfaces.h
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/SKSE/Interfaces.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::{re::BSScript, skse::impls::stab::SKSEPapyrusInterface};

type RegFunction1 = fn(vm: *mut BSScript::Internal::VirtualMachine) -> bool;
type RegFunction2 = fn(vm: *mut BSScript::IVirtualMachine) -> bool;

pub struct PapyrusInterface {
    address: *const u8,
}

impl PapyrusInterface {
    pub const VERSION: u32 = 1;

    pub fn version(&self) -> u32 {
        unsafe { (*self.get_proxy()).interface_version }
    }

    pub fn register<F>(&self, func: F) -> bool
    where
        F: Fn(&mut BSScript::Internal::VirtualMachine) -> bool,
    {
        self.register_impl(func)
    }

    pub fn register_multiple<F, R>(&self, func_iter: R) -> bool
    where
        F: Fn(&mut BSScript::Internal::VirtualMachine) -> bool,
        R: IntoIterator<Item = F>,
    {
        func_iter.into_iter().all(|f| self.register_impl(f))
    }

    fn register_impl<F>(&self, func: F) -> bool
    where
        F: Fn(&mut BSScript::Internal::VirtualMachine) -> bool,
    {
        unimplemented!()
        // unsafe { ((*self.get_proxy()).register)((&mut func as *mut _).cast()) }
    }

    pub fn get_proxy(&self) -> *const SKSEPapyrusInterface {
        assert!(!self.address.is_null());
        self.address.cast()
    }
}

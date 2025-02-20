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
// type RegFunction2 = fn(vm: *mut BSScript::IVirtualMachine) -> bool;

#[derive(Debug)]
pub struct PapyrusInterface {
    address: *const u8,
}

impl PapyrusInterface {
    pub const VERSION: u32 = 1;

    pub fn version(&self) -> u32 {
        unsafe { (*self.get_proxy()).interface_version }
    }

    pub fn register(&self, func: RegFunction1) -> bool {
        self.register_impl(func)
    }

    pub fn register_multiple<R>(&self, func_iter: R) -> bool
    where
        R: IntoIterator<Item = RegFunction1>,
    {
        func_iter.into_iter().all(|f| self.register_impl(f))
    }

    pub fn register_impl(&self, mut func: RegFunction1) -> bool {
        let vm = BSScript::Internal::VirtualMachine::get_singleton();

        if !vm.is_null() {
            func(vm);
            return true;
        }

        let result =
            unsafe { ((*self.get_proxy()).register)((&mut func as *mut RegFunction1).cast()) };
        if !result {
            #[cfg(feature = "tracing")]
            tracing::error!("Failed to register papyrus callback");
        };
        result
    }

    // pub fn register_impl2(&self, mut func: RegFunction2) -> bool {
    //     let vm = BSScript::Internal::VirtualMachine::get_singleton();

    //     if !vm.is_null() {
    //         func(vm);
    //         return true;
    //     }

    //     let result =
    //         unsafe { ((*self.get_proxy()).register)((&mut func as *mut RegFunction2).cast()) };
    //     if !result {
    //         tracing::error!("Failed to register papyrus callback");
    //     };
    //     result
    // }

    fn get_proxy(&self) -> *const SKSEPapyrusInterface {
        assert!(!self.address.is_null());
        self.address.cast()
    }
}

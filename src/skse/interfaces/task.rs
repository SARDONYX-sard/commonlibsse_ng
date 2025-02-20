// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/SKSE/Interfaces.h
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/SKSE/Interfaces.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::skse::impls::stab::{SKSETaskInterface, TaskDelegate, UiDelegateV1};

// wrong implementation
// Difficult due to the need to implement C++ virtual functions
// https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/SKSE/Interfaces.cpp#L204
type TaskFn = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
pub struct TaskInterface {
    address: *const u8,
}

impl TaskInterface {
    pub const VERSION: u32 = 2;

    pub fn version(&self) -> u32 {
        unsafe { (*self.get_proxy()).interface_version }
    }

    pub fn add_task(&self, mut task: TaskFn) {
        unsafe { ((*self.get_proxy()).add_task)((&mut task as *mut TaskFn).cast()) }
    }

    pub fn add_task_delegate(&self, mut task: TaskDelegate) {
        unsafe { ((*self.get_proxy()).add_task)((&mut task as *mut TaskDelegate).cast()) }
    }

    pub fn add_ui_task(&self, mut task: TaskFn) {
        unsafe { ((*self.get_proxy()).add_ui_task)((&mut task as *mut TaskFn).cast()) }
    }

    pub fn add_ui_task_delegate(&self, mut task: UiDelegateV1) {
        unsafe { ((*self.get_proxy()).add_ui_task)((&mut task as *mut UiDelegateV1).cast()) }
    }

    /// # Panics
    pub fn get_proxy(&self) -> *const SKSETaskInterface {
        assert!(!self.address.is_null());
        self.address.cast()
    }
}

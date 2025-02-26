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

#[derive(Debug, Clone)]
pub struct TaskInterface(&'static SKSETaskInterface);

impl TaskInterface {
    pub const VERSION: u32 = 2;

    #[inline]
    pub(crate) const fn new(interface: &'static SKSETaskInterface) -> Self {
        Self(interface)
    }

    #[inline]
    pub const fn version(&self) -> u32 {
        self.0.interfaceVersion
    }

    #[inline]
    pub fn add_task(&self, mut task: TaskFn) {
        unsafe { (self.0.AddTask)((&mut task as *mut TaskFn).cast()) }
    }

    #[inline]
    pub fn add_task_delegate(&self, mut task: TaskDelegate) {
        unsafe { (self.0.AddTask)((&mut task as *mut TaskDelegate).cast()) }
    }

    #[inline]
    pub fn add_ui_task(&self, mut task: TaskFn) {
        unsafe { (self.0.AddUiTask)((&mut task as *mut TaskFn).cast()) }
    }

    #[inline]
    pub fn add_ui_task_delegate(&self, mut task: UiDelegateV1) {
        unsafe { (self.0.AddUiTask)((&mut task as *mut UiDelegateV1).cast()) }
    }
}

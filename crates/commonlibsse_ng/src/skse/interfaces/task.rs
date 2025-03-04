// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/SKSE/Interfaces.h
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/SKSE/Interfaces.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::skse::impls::stab::SKSETaskInterface;
use core::ffi::c_void;

#[derive(Debug, Clone)]
pub struct TaskInterface(&'static SKSETaskInterface);

impl TaskInterface {
    /// The version number of the task interface.
    pub const VERSION: u32 = 2;

    #[inline]
    pub(crate) const fn new(interface: &'static SKSETaskInterface) -> Self {
        Self(interface)
    }

    /// Returns the version number of the task interface.
    #[inline]
    pub const fn version(&self) -> u32 {
        self.0.interfaceVersion
    }

    // NOTE: Omitted because I have not found a way to reproduce the ABI of std::function in Rust.

    #[inline]
    pub fn add_task_delegate(&self, f: Box<dyn FnOnce() + Send + 'static>) {
        unsafe { (self.0.AddTask)(TaskDelegate::new_as_c_void(f)) }
    }

    #[inline]
    pub fn add_ui_task_delegate(&self, f: Box<dyn FnOnce() + Send + 'static>) {
        unsafe { (self.0.AddUiTask)((UIDelegateV1::new_as_c_void(f)).cast()) }
    }
}

/// Same vtbl layout as TaskDelegate, so we can use it all the time.
type UIDelegateV1 = TaskDelegate;

/// Rust structure that exactly reproduces the memory layout of C++ virtual class
///
/// See [`C to C++ vtbl FFI`](https://godbolt.org/z/eTY4M7h7f)
#[repr(C)] // <- This will mess up the memory layout if we don't guarantee the order of the fields.
struct TaskDelegate {
    /// When a C++ virtual function is impl, a pointer to the virtual function table
    /// is added to the class situation. This reproduces it.
    vtbl: &'static TaskDelegateVtbl,
    /// This is not inherently present on the C++ side, but is necessary to run the Rust task.
    ///
    /// One-time call by `Option`
    rust_fn: Option<Box<dyn FnOnce() + Send + 'static>>,
}

/// Virtual table for task delegate functions.
#[repr(C)]
#[derive(Debug)]
struct TaskDelegateVtbl {
    /// Executes the task.
    ///
    /// It is SKSE in C++ that calls this, not itself.
    run: fn(&mut TaskDelegate),
    /// Delete itself here because SKSE can call this to delete memory.
    ///
    /// In other words, if this is called twice or used after deletion (Use after free), it is an undefined operation.
    dispose: fn(*mut TaskDelegate),
}

impl TaskDelegate {
    /// Executes the task.
    ///
    /// It is SKSE in C++ that calls this, not itself.
    fn run(&mut self) {
        // One-time call by `Option`
        if let Some(rust_fn) = self.rust_fn.take() {
            rust_fn();
        }
    }

    /// Delete itself here because SKSE can call this to delete memory.
    ///
    /// In other words, if this is called twice or used after deletion (Use after free), it is an undefined operation.
    fn dispose(task: *mut Self) {
        unsafe { drop(Box::from_raw(task)) };
    }
}

impl TaskDelegate {
    /// Creates a new Task delegate
    fn new(f: Box<dyn FnOnce() + Send + 'static>) -> Box<Self> {
        const TASK_VIRTUAL_FN_TABLE: TaskDelegateVtbl =
            TaskDelegateVtbl { run: TaskDelegate::run, dispose: TaskDelegate::dispose };

        Box::new(Self { rust_fn: Some(f), vtbl: &TASK_VIRTUAL_FN_TABLE })
    }

    /// Creates a new C++ task delegate
    ///
    /// This struct is not dropped unless dispose is called.
    fn new_as_c_void(f: Box<dyn FnOnce() + Send + 'static>) -> *mut c_void {
        Box::into_raw(Box::new(Self::new(f))).cast()
    }
}

// C++ Original code
// - https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/REL/Module.h
// - load_segments, clear: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/REL/Module.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Module handling library for Skyrim SE/AE/VR .

mod module_core;
mod module_handle;
mod runtime;
mod segment;

pub use self::module_core::{Module, ModuleInitError};
pub use self::module_handle::{ModuleHandle, ModuleHandleError};
pub use self::runtime::Runtime;
pub use self::segment::{Segment, SegmentName};

use std::sync::atomic::AtomicBool;
use std::sync::{LazyLock, RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockResult};

/// Clear flag separated from singleton instances to avoid taking locks unnecessarily.
static IS_CLEARED: AtomicBool = AtomicBool::new(false);
static MODULE: LazyLock<RwLock<ModuleState>> = LazyLock::new(|| RwLock::new(ModuleState::init()));

/// Represents the state of the module.
///
/// This enum implements an API to manage a single global variable of internally managed module (e.g. `SkyrimSE.exe`) information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleState {
    /// The module is successfully initialized and active.
    Active(Module),

    /// The module instance has been explicitly cleared and memory has been freed.
    Cleared,

    /// The module failed to initialize.
    FailedInit(ModuleInitError),
}

impl ModuleState {
    /// Initialize the module.
    fn init() -> Self {
        match Module::init() {
            Ok(module) => Self::Active(module),
            Err(err) => Self::FailedInit(err),
        }
    }

    /// Attempts to retrieve a read-only reference to the current module state.
    ///
    /// It is better to call [`Clone::clone`] and drop the `guard` than to lock for a long time.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::rel::module::ModuleState;
    ///
    /// match ModuleState::get() {
    ///     Ok(guard) => match &*guard { // <- It is locked during this block scope.
    ///         ModuleState::Active(module) => {
    ///             println!("Module version: {}", module.version);
    ///         }
    ///         ModuleState::Cleared => println!("Module has been reset."),
    ///         ModuleState::FailedInit(module_init_error) => {
    ///             tracing::error!("Failed to initialize module: {module_init_error}");
    ///         }
    ///         // unlocked by guard
    ///     },
    ///     Err(lock_err) => tracing::error!("Failed to lock. {lock_err}"),
    /// };
    /// ```
    ///
    /// # Example(Long time reading)
    ///
    /// If we read the information of a module for a long time we should [`Drop::drop`].
    ///
    /// Q. Is the address of the Module singleton valid in that case?
    ///
    /// A. This is valid unless someone in the current process is calling `FreeLibrary`(Win32 API) unnaturally for `SKSE`, `SkyrimSE.exe`, etc.
    ///    And there is no call to `FreeLibrary` in the Module singleton.
    ///
    /// ```
    /// use commonlibsse_ng::rel::module::{ModuleState, Runtime};
    ///
    /// // This time we clone the information to read the module information for a long time
    /// // and immediately let go of the guard.
    /// let module_state = match ModuleState::get() {
    ///     Ok(guard) => guard.clone(), // Auto unlock by `Drop::drop`
    ///     Err(lock_err) => {
    ///         tracing::error!("RwLock is poisoned: {lock_err}");
    ///         return;
    ///     }
    /// };
    ///
    /// if let ModuleState::Active(module) = module_state {
    ///    // Some kind of prolonged processing. (Reproduced in sleep in the example)
    ///    std::thread::sleep(std::time::Duration::from_secs(2));
    ///
    ///    assert_eq!(module.runtime, Runtime::Se);
    ///    tracing::info!("Module version: {}", module.version);
    /// }
    /// ```
    ///
    /// # Errors
    /// If the thread that had previously acquired a lock on the singleton instance panics(i.e. poisoned), an error is returned.
    #[inline]
    pub fn get() -> TryLockResult<RwLockReadGuard<'static, Self>> {
        MODULE.try_read()
    }

    /// Attempts to apply a function to the active module state.
    ///
    /// This function tries to acquire a read lock on the module state and applies
    /// the provided function `f` if the module state is [`ModuleState::Active`].
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::rel::module::ModuleState;
    ///
    /// let result = ModuleState::map_active(|module| module.version.clone());
    /// match result {
    ///     Ok(version) => println!("Module version: {}", version),
    ///     Err(err) => eprintln!("Error: {:?}", err),
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if:
    /// - The module state is [`ModuleState::Cleared`].
    /// - The module state is [`ModuleState::FailedInit`], in which case the initialization error is propagated.
    /// - The internal lock is poisoned.
    pub fn map_active<F, T>(f: F) -> Result<T, ModuleStateError>
    where
        F: FnOnce(&Module) -> T,
    {
        let guard = MODULE
            .try_read()
            .map_err(|_| ModuleStateError::ModuleLockIsPoisoned)?;

        match &*guard {
            Self::Active(module) => Ok(f(module)),
            Self::Cleared => Err(ModuleStateError::ModuleHasBeenCleared),
            Self::FailedInit(module_init_error) => Err(ModuleStateError::FailedInit {
                source: module_init_error.clone(),
            }),
        }
    }

    /// Attempts to retrieve a mutable reference to the active module.
    ///
    /// If the module is in the `Cleared` state, it will be reinitialized.
    ///
    /// **Note:** If you only want to read the value of a Module, you should use `ModuleState::get`.
    ///
    /// Reasons: The `MODULE` singleton uses [`RwLock`], multiple threads can read at the same time if other threads are not using write.
    ///
    /// Also, it is better to call [`Clone::clone`] and drop the `guard` than to lock for a long time.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::rel::module::ModuleState;
    ///
    /// match ModuleState::get_or_init_mut() {
    ///     Ok(mut drop_guard) => match &mut *drop_guard {
    ///         ModuleState::Active(module) => {
    ///             println!("Module version: {}", module.version);
    ///             // We can change this value, but be careful when modifying it.
    ///             module.file_path = "Test".to_string();
    ///         }
    ///         ModuleState::Cleared => println!("Module has been reset."),
    ///         ModuleState::FailedInit(module_init_error) => {
    ///             tracing::error!("Failed to initialize module: {module_init_error}");
    ///         }
    ///     },
    ///     Err(lock_err) => tracing::error!("Failed to lock. {lock_err}"),
    /// };
    /// ```
    ///
    /// # Errors
    /// If the thread that had previously acquired a lock on the singleton instance panics(i.e. poisoned), an error is returned.
    pub fn get_or_init_mut() -> TryLockResult<RwLockWriteGuard<'static, Self>> {
        use core::sync::atomic::Ordering;

        if IS_CLEARED.load(Ordering::Acquire) {
            if let Ok(mut guard) = MODULE.try_write() {
                *guard = Self::init();
            };
            IS_CLEARED.store(true, Ordering::Release);
        }

        MODULE.try_write()
    }

    /// Clears the module, transitioning it to the `Cleared` state.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::rel::module::ModuleState;
    ///
    /// assert!(ModuleState::reset().is_ok());
    /// ```
    ///
    /// # Errors
    /// If the thread that had previously acquired a lock on the singleton instance panics, an error is returned.
    pub fn reset() -> TryLockResult<()> {
        MODULE
            .try_write()
            .map_or(Err(std::sync::TryLockError::WouldBlock), |mut guard| {
                *guard = Self::Cleared;
                IS_CLEARED.store(true, std::sync::atomic::Ordering::Release);
                Ok(())
            })
    }
}

/// Type definition for treating an instance of information management as an error when it is in
/// a state where information cannot be obtained.
#[derive(Debug, snafu::Snafu)]
pub enum ModuleStateError {
    /// The thread that was getting Module's lock panicked.
    ModuleLockIsPoisoned,

    /// Module has been cleared
    ModuleHasBeenCleared,

    /// Module initialization error
    #[snafu(display("Module initialization error: {source}"))]
    FailedInit {
        source: crate::rel::module::ModuleInitError,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_reset() {
        assert!(ModuleState::reset().is_ok());
        assert!(IS_CLEARED.load(std::sync::atomic::Ordering::Acquire));
    }
}

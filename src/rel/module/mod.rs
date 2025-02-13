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

use std::sync::{LazyLock, RwLock};

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
        match Module::from_skyrim() {
            Ok(module) => Self::Active(module),
            Err(err) => Self::FailedInit(err),
        }
    }

    /// Attempts to apply a function to the active module state.
    ///
    /// This function tries to acquire a read lock on the module state and applies
    /// the provided function `f` if the module state is [`ModuleState::Active`].
    ///
    /// If you do not know when you did the [`Self::reset`], or if you want it to be reinitialized automatically if necessary,
    /// [`Self::map_or_init`] is useful.
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
    ///
    /// # Panics
    /// This function might panic when called if the lock is already held by the current thread.
    pub fn map_active<F, T>(f: F) -> Result<T, ModuleStateError>
    where
        F: FnOnce(&Module) -> T,
    {
        let guard = MODULE
            .read()
            .map_err(|_| ModuleStateError::ModuleLockIsPoisoned)?;

        match &*guard {
            Self::Active(module) => Ok(f(module)),
            Self::Cleared => Err(ModuleStateError::ModuleHasBeenCleared),
            Self::FailedInit(module_init_error) => Err(ModuleStateError::FailedInit {
                source: module_init_error.clone(),
            }),
        }
    }

    /// Attempts to apply a function to the active module state, initializing it if necessary.
    ///
    /// If the module state is `Cleared`, it will be initialized before applying the function `f`.
    /// This function also attempts a read lock first and falls back to initialization only if needed.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::rel::module::ModuleState;
    ///
    /// let result = ModuleState::map_or_init(|module| module.version.clone());
    /// match result {
    ///     Ok(version) => println!("Module version: {}", version),
    ///     Err(err) => eprintln!("Error: {:?}", err),
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if:
    /// - The module state is [`ModuleState::FailedInit`], in which case the initialization error is propagated.
    /// - The internal lock is poisoned.
    ///
    /// # Panics
    /// This function might panic when called if the lock is already held by the current thread.
    pub fn map_or_init<F, T>(f: F) -> Result<T, ModuleStateError>
    where
        F: FnOnce(&Module) -> T,
    {
        if let Ok(guard) = MODULE.read() {
            if let Self::Active(module) = &*guard {
                return Ok(f(module));
            }
        }

        // The fact that it was not `Active` means that it absolutely needs to be initialized.
        let (ret, module_state) = match Module::from_skyrim() {
            Ok(module) => (Ok(f(&module)), Self::Active(module)),
            Err(err) => {
                let ret_err = ModuleStateError::FailedInit {
                    source: err.clone(),
                };
                (Err(ret_err), Self::FailedInit(err))
            }
        };

        // Delaying lock acquisition to avoid prolonged lock acquisition.
        MODULE
            .write()
            .map(|mut guard| *guard = module_state)
            .map_err(|_| ModuleStateError::ModuleLockIsPoisoned)?;

        ret
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
    ///
    /// # Panics
    /// This function might panic when called if the lock is already held by the current thread.
    pub fn reset() -> Result<(), ModuleStateError> {
        MODULE
            .write()
            .map_or(Err(ModuleStateError::ModuleLockIsPoisoned), |mut guard| {
                *guard = Self::Cleared;
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
    }
}

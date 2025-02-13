// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/REL/Offset.h
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT

use crate::rel::id::DataBaseError;
use crate::rel::module::ModuleState;

/// Represents an ID with a possible VR-specific offset.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Offset(usize);

impl Offset {
    /// Creates a new `Offset` instance.
    #[inline]
    pub const fn new(offset: usize) -> Self {
        Self(offset)
    }

    /// Get the absolute address corresponding to the offset.
    ///
    /// # Errors
    /// Returns an error if the ID cannot be resolved.
    #[inline]
    pub fn address(&self) -> Result<usize, DataBaseError> {
        let offset = self.offset();
        Ok(if offset == 0 {
            0
        } else {
            Self::base()? + offset
        })
    }

    /// Get the offset.
    #[inline]
    pub const fn offset(&self) -> usize {
        self.0
    }

    /// Get the base address of the module.
    ///
    /// # Errors
    /// Returns an error if the module is in an invalid state.
    #[inline]
    fn base() -> Result<usize, crate::rel::module::ModuleStateError> {
        ModuleState::map_or_init(|module| module.base.as_raw())
    }
}

/// Represents an ID with a possible VR-specific offset.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VariantOffset {
    se_offset: u64,
    ae_offset: u64,
    vr_offset: u64,
}

impl VariantOffset {
    /// Creates a new `VariantOffset` instance.
    #[inline]
    pub const fn new(se_offset: u64, ae_offset: u64, vr_offset: u64) -> Self {
        Self {
            se_offset,
            ae_offset,
            vr_offset,
        }
    }

    /// Get the absolute address corresponding to the offset.
    ///
    /// # Errors
    /// Returns an error if the ID cannot be resolved.
    #[inline]
    pub fn address(&self) -> Result<usize, DataBaseError> {
        let offset = self.offset()?;
        Ok(if offset == 0 {
            0
        } else {
            Self::base()? + offset
        })
    }

    /// Get the offset corresponding to the `Runtime`.
    ///
    /// # Errors
    /// Returns an error if the ID is not found.
    #[inline]
    pub fn offset(&self) -> Result<usize, DataBaseError> {
        use crate::rel::module::Runtime;

        let runtime = ModuleState::map_or_init(|module| module.runtime)?; // derived Copy

        Ok(match runtime {
            Runtime::Ae => self.ae_offset,
            Runtime::Se => self.se_offset,
            Runtime::Vr => self.vr_offset,
        } as usize)
    }

    /// Get the base address of the module.
    ///
    /// # Errors
    /// Returns an error if the module is in an invalid state.
    #[inline]
    fn base() -> Result<usize, crate::rel::module::ModuleStateError> {
        ModuleState::map_or_init(|module| module.base.as_raw())
    }
}

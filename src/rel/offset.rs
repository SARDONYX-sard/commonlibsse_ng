// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/REL/Offset.h
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT

use crate::rel::id::DataBaseError;
use crate::rel::module::ModuleState;
use crate::rel::ResolvableAddress;

/// Represents an offset that can be used to compute an absolute address.
///
/// This struct wraps a `usize` value, which directly corresponds to an offset.
///
/// ```
/// use commonlibsse_ng::rel::offset::Offset;
/// use commonlibsse_ng::rel::ResolvableAddress as _;
///
/// let offset = Offset::new(0x1000);
/// assert_eq!(offset.offset().unwrap(), 0x1000);
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Offset(usize);

impl Offset {
    /// Creates a new `Offset` instance with the given value.
    #[inline]
    pub const fn new(offset: usize) -> Self {
        Self(offset)
    }
}

impl ResolvableAddress for Offset {
    /// Returns the stored offset value.
    ///
    /// This implementation absolutely returns [`Result::Ok`].
    #[inline]
    fn offset(&self) -> Result<usize, DataBaseError> {
        Ok(self.0)
    }
}

/// Represents an offset that varies depending on the runtime environment.
///
/// This struct holds three possible offset values, each corresponding to a
/// different runtime: Special Edition (`se_offset`), Anniversary Edition (`ae_offset`),
/// and Virtual Reality (`vr_offset`).
///
/// The appropriate offset is selected based on the current runtime.
///
/// # Example
/// ```rust
/// use commonlibsse_ng::rel::offset::VariantOffset;
/// use commonlibsse_ng::rel::ResolvableAddress as _;
///
/// let variant_offset = VariantOffset::new(0x1000, 0x2000, 0x3000);
/// let offset = variant_offset.offset().unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VariantOffset {
    se_offset: u64,
    ae_offset: u64,
    vr_offset: u64,
}

impl VariantOffset {
    /// Creates a new `VariantOffset` instance with specified offsets for each runtime.
    #[inline]
    pub const fn new(se_offset: u64, ae_offset: u64, vr_offset: u64) -> Self {
        Self {
            se_offset,
            ae_offset,
            vr_offset,
        }
    }
}

impl ResolvableAddress for VariantOffset {
    /// Retrieves the offset based on the current runtime.
    ///
    /// # Errors
    /// Returns an error if the module state is invalid or the runtime is unknown.
    #[inline]
    fn offset(&self) -> Result<usize, DataBaseError> {
        use crate::rel::module::Runtime;

        let runtime = ModuleState::map_or_init(|module| module.runtime)?; // Derived Copy

        Ok(match runtime {
            Runtime::Ae => self.ae_offset,
            Runtime::Se => self.se_offset,
            Runtime::Vr => self.vr_offset,
        } as usize)
    }
}

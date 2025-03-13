//! REL dir portion of `CommonLibSSE-NG` written by hand.

pub mod id;
#[cfg(feature = "win_api")]
pub mod module;
pub mod offset;
pub mod relocation;
pub mod version;

use self::id::DataBaseError;
use self::module::{ModuleState, ModuleStateError};
use core::ffi::c_void;
use core::num::NonZeroUsize;
use core::ptr::NonNull;

/// A trait for resolving an absolute address based on an offset.
///
/// Implementing types must provide an `offset()` method that returns the offset
/// used to compute the final address.
///
/// The base address is retrieved using the `base()` method, which fetches
/// the module's base address.
///
/// # Errors
/// - If the offset cannot be resolved, `offset()` returns a `DataBaseError`.
/// - If the base address cannot be retrieved, `base()` returns a `ModuleStateError`.
pub trait ResolvableAddress {
    /// Returns the offset associated with this instance.
    ///
    /// # Errors
    /// Returns an error if the offset cannot be determined.
    fn offset(&self) -> Result<NonZeroUsize, DataBaseError>;

    /// Computes the absolute address by adding the offset to the module's base address.
    ///
    /// If the offset is `0`, the function returns `ptr::null_mut` as well.
    ///
    /// # Errors
    /// - Returns `DataBaseError` if the offset cannot be determined.
    /// - Returns `ModuleStateError` if the base address is unavailable.
    #[inline]
    fn address(&self) -> Result<NonNull<c_void>, DataBaseError> {
        let offset = self.offset()?;
        Ok(unsafe { Self::base()?.byte_add(offset.get()) })
    }

    /// Retrieves the base address of the module.
    ///
    /// # Errors
    /// Returns an error if the module is in an invalid state.
    #[inline]
    fn base() -> Result<NonNull<c_void>, ModuleStateError> {
        ModuleState::map_or_init(|module| module.base.0)
    }
}

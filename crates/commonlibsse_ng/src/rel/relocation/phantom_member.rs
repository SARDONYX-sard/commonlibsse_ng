use crate::skse::version::RUNTIME_SSE_1_6_629;

use super::{RelocationError, relocate_member_if_newer, relocate_member_if_newer_mut};
use core::marker::PhantomData;

/// A zero-sized marker used to access dynamically relocated members.
///
/// In the C++ class, it is not possible to simultaneously obtain mutable references
/// to multiple fields due to Rust's borrowing rules. To work around this, `PhantomMember` provides
/// an API for safely accessing individual fields while respecting the newer runtime offsets.
///
/// This struct is a placeholder that allows access to relocated members by providing
/// runtime-based offset calculations.
///
/// # Generics
/// - `T`: Member type
/// - `OLD`: Offset when < ver.1_6_629
/// - `NEW`: Offset when >= ver.1_6_629
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhantomMember<T, const OLD: isize, const NEW: isize> {
    marker: PhantomData<T>,
}

impl<T, const OLD: isize, const NEW: isize> Clone for PhantomMember<T, OLD, NEW> {
    #[inline]
    fn clone(&self) -> Self {
        Self { marker: self.marker }
    }
}

impl<T, const OLD: isize, const NEW: isize> PhantomMember<T, OLD, NEW> {
    /// Retrieves a reference to the member as immutable.
    ///
    /// # Safety
    /// This performs an unsafe relocation based on runtime version checks.
    /// Returns `None` if the relocation fails.
    ///
    /// # Errors
    /// - This function may return an error if the module's state cannot be accessed, or if the `map_active` call fails when fetching the current version.
    /// - If the pointer is null
    /// - If the pointer is unaligned
    #[inline]
    pub fn get(&self) -> Result<&T, RelocationError> {
        unsafe { relocate_member_if_newer(RUNTIME_SSE_1_6_629, self, OLD, NEW) }
    }

    /// Retrieves a mutable reference to the member.
    ///
    /// # Safety
    /// This performs an unsafe relocation based on runtime version checks.
    /// Returns `None` if the relocation fails.
    ///
    /// # Errors
    /// - This function may return an error if the module's state cannot be accessed, or if the `map_active` call fails when fetching the current version.
    /// - If the pointer is null
    /// - If the pointer is unaligned
    #[inline]
    pub fn get_mut(&mut self) -> Result<&mut T, RelocationError> {
        unsafe { relocate_member_if_newer_mut(RUNTIME_SSE_1_6_629, self, OLD, NEW) }
    }
}

// SPDX-FileCopyrightText: (c) The Rust Project Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// - https://github.com/rust-lang/rust/blob/master/LICENSE-MIT
//
//! Rust's Allocator(removed self) compatible memory allocator for Skyrim.

pub mod allocator;
pub mod global;
pub mod tes_global;

use core::ptr;
use core::{alloc::Layout, hint, ptr::NonNull};

use std::alloc::{alloc, alloc_zeroed, dealloc, realloc};
use stdx::alloc::{AllocError, non_null_empty_slice};

#[inline]
#[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
#[allow(clippy::unused_self)]
pub(crate) fn alloc_impl(layout: Layout, zeroed: bool) -> Result<NonNull<[u8]>, AllocError> {
    match layout.size() {
        0 => Ok(non_null_empty_slice(layout)),
        // SAFETY: `layout` is non-zero in size,
        size => unsafe {
            let raw_ptr = if zeroed { alloc_zeroed(layout) } else { alloc(layout) };
            let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
            Ok(NonNull::slice_from_raw_parts(ptr, size))
        },
    }
}

// SAFETY: Same as `Allocator::grow`
#[inline]
#[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
pub(crate) unsafe fn grow_impl(
    ptr: NonNull<u8>,
    old_layout: Layout,
    new_layout: Layout,
    zeroed: bool,
) -> Result<NonNull<[u8]>, AllocError> {
    debug_assert!(
        new_layout.size() >= old_layout.size(),
        "`new_layout.size()` must be greater than or equal to `old_layout.size()`"
    );

    match old_layout.size() {
        0 => alloc_impl(new_layout, zeroed),

        // SAFETY: `new_size` is non-zero as `old_size` is greater than or equal to `new_size`
        // as required by safety conditions. Other conditions must be upheld by the caller
        old_size if old_layout.align() == new_layout.align() => unsafe {
            let new_size = new_layout.size();

            // `realloc` probably checks for `new_size >= old_layout.size()` or something similar.
            hint::assert_unchecked(new_size >= old_layout.size());

            let raw_ptr = realloc(ptr.as_ptr(), old_layout, new_size);
            let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
            if zeroed {
                raw_ptr.add(old_size).write_bytes(0, new_size - old_size);
            }
            Ok(NonNull::slice_from_raw_parts(ptr, new_size))
        },

        // SAFETY: because `new_layout.size()` must be greater than or equal to `old_size`,
        // both the old and new memory allocation are valid for reads and writes for `old_size`
        // bytes. Also, because the old allocation wasn't yet deallocated, it cannot overlap
        // `new_ptr`. Thus, the call to `copy_nonoverlapping` is safe. The safety contract
        // for `dealloc` must be upheld by the caller.
        old_size => unsafe {
            let new_ptr = alloc_impl(new_layout, zeroed)?;
            ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.cast().as_ptr(), old_size);
            deallocate(ptr, old_layout);
            Ok(new_ptr)
        },
    }
}

#[inline]
#[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
unsafe fn deallocate(ptr: NonNull<u8>, layout: Layout) {
    if layout.size() != 0 {
        // SAFETY: `layout` is non-zero in size,
        // other conditions must be upheld by the caller
        unsafe { dealloc(ptr.as_ptr(), layout) }
    }
}

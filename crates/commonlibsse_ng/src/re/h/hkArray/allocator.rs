//! Allocator for `hkArray`

/// A memory allocator trait used for allocating and freeing memory blocks,
/// primarily for use with `hkArray`.
///
/// # Safety
///
/// This trait is `unsafe` because the implementor must ensure that:
/// - `alloc` returns a valid pointer to a block of memory of at least `new_size` bytes,
///   or a null pointer if allocation fails.
/// - `free` is only called with pointers previously returned by `alloc`, and the `num_bytes`
///   must match the size passed to `alloc`.
///
/// Improper implementations or usage of these functions may lead to undefined behavior.
pub trait Allocator {
    /// Allocates a memory block of the given size in bytes.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `new_size` is non-negative and that the result
    /// is properly managed and eventually passed to `free`.
    unsafe fn alloc(new_size: i32) -> *mut u8;

    /// Allocates a zero-initialized memory block of the given size in bytes.
    ///
    /// # Safety
    ///
    /// This default implementation calls `alloc` and then zeroes the memory.
    unsafe fn alloc_zeroed(new_size: i32) -> *mut u8 {
        let ptr = unsafe { Self::alloc(new_size) };
        if !ptr.is_null() && new_size > 0 {
            unsafe { core::ptr::write_bytes(ptr, 0, new_size as usize) };
        }
        ptr
    }

    /// Frees a memory block previously returned by `alloc`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `ptr` was returned by `alloc` and that `num_bytes`
    /// matches the original allocation size.
    unsafe fn free(ptr: *mut u8, num_bytes: i32);
}

/// The default allocator can only be used in-game, so a Rust allocator exists for use in CI and for testing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RustAllocator;

impl Allocator for RustAllocator {
    unsafe fn alloc(new_size: i32) -> *mut u8 {
        use core::ptr;
        use std::alloc::{Layout, alloc};

        if new_size <= 0 {
            return ptr::null_mut();
        }

        let layout = Layout::from_size_align(new_size as usize, core::mem::align_of::<u8>());
        layout.map_or(ptr::null_mut(), |layout| unsafe { alloc(layout) })
    }

    unsafe fn free(ptr: *mut u8, num_bytes: i32) {
        use std::alloc::{Layout, dealloc};

        if ptr.is_null() || num_bytes <= 0 {
            return;
        }

        let layout = Layout::from_size_align(num_bytes as usize, core::mem::align_of::<u8>());
        if let Ok(layout) = layout {
            unsafe { dealloc(ptr, layout) }
        }
    }
}

/// An allocator for Skyrim that can only be used in-game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkyrimAllocator;

impl Allocator for SkyrimAllocator {
    unsafe fn alloc(new_size: i32) -> *mut u8 {
        use crate::re::hkContainerAllocators::{Allocator, hkContainerHeapAllocator};

        if let Some(allocator) = hkContainerHeapAllocator::get_singleton_mut() {
            if let Some(vtable) = unsafe { allocator.vtbl.as_ref() } {
                let allocator = allocator as *mut Allocator;
                let mut new_size = new_size;
                return (vtable.__base.BufAlloc)(allocator.cast(), &mut new_size);
            };
        };

        core::ptr::null_mut()
    }

    unsafe fn free(ptr: *mut u8, num_bytes: i32) {
        use crate::re::hkContainerAllocators::{Allocator, hkContainerHeapAllocator};

        if let Some(allocator) = hkContainerHeapAllocator::get_singleton_mut() {
            if let Some(vtable) = unsafe { allocator.vtbl.as_ref() } {
                let allocator = allocator as *mut Allocator;
                (vtable.__base.BlockFree)(allocator.cast(), ptr, num_bytes);
            };
        };
    }
}

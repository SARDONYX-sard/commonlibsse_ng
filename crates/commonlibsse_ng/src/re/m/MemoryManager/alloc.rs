pub mod global;

#[cfg(not(feature = "test_on_ci"))]
pub use self::inner::{alloc, alloc_zeroed, dealloc, realloc};
#[cfg(feature = "test_on_ci")] // Since TESAllocator is not available for CI, use Rust's.
pub use std::alloc::{alloc, alloc_zeroed, dealloc, realloc};

mod inner {
    use core::alloc::Layout;
    use core::ffi::c_void;
    use core::ptr;

    use crate::re::MemoryManager::MemoryManager;

    /// Allocates memory using the custom memory manager.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the returned pointer is uninitialized and must be manually managed.
    ///
    /// Corresponds to C++: `malloc`
    ///
    /// Returns null on failure.
    #[inline]
    pub unsafe fn alloc(layout: Layout) -> *mut u8 {
        unsafe {
            MemoryManager::GetSingleton().as_mut().map_or(ptr::null_mut(), |heap| {
                heap.Allocate(layout.size(), layout.align() as i32, true).cast()
            })
        }
    }

    /// Allocates zero-initialized memory using the custom memory manager.
    ///
    /// # Safety
    ///
    /// This function is unsafe because manual deallocation is required.
    ///
    /// Corresponds to C++: `calloc`
    ///
    /// Returns null on failure.
    #[inline]
    pub unsafe fn alloc_zeroed(layout: Layout) -> *mut u8 {
        let ptr = unsafe { alloc(layout) };
        if !ptr.is_null() {
            unsafe { ptr::write_bytes(ptr, 0, layout.size()) };
        }
        ptr
    }

    /// Reallocates a memory block using the custom memory manager.
    ///
    /// # Safety
    ///
    /// - `ptr` must have been allocated by `alloc` or `alloc_zeroed` with the given `old_layout`.
    /// - The memory may be moved; the original pointer must not be used after calling this.
    ///
    /// Corresponds to C++: `realloc`
    ///
    /// Returns null on failure.
    #[inline]
    pub unsafe fn realloc(ptr: *mut u8, old_layout: Layout, new_size: usize) -> *mut u8 {
        let alignment = old_layout.align() as i32;
        unsafe {
            MemoryManager::GetSingleton().as_mut().map_or(ptr::null_mut(), |heap| {
                heap.Reallocate(ptr.cast::<c_void>(), new_size, alignment, true).cast()
            })
        }
    }

    /// Deallocates memory previously allocated with `alloc` or `alloc_zeroed`.
    ///
    /// # Safety
    ///
    /// - `ptr` must have been allocated by `alloc` or `alloc_zeroed` with the same `layout`.
    /// - Undefined behavior may occur if `ptr` is invalid or if `layout` does not match.
    ///
    /// Corresponds to C++: `free`
    #[inline]
    pub unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
        if let Some(heap) = unsafe { MemoryManager::GetSingleton().as_mut() } {
            let aligned = layout.align() > 1;
            unsafe { heap.Deallocate(ptr.cast::<c_void>(), aligned) };
        }
    }
}

use crate::re::BSTArray::Allocator;

use core::alloc::Layout;
use core::ffi::c_void;
use std::alloc::{alloc_zeroed, dealloc};

/// Allocator implemented using Rust's global allocator.
///
/// This is basically used for testing, Rust docs example
pub struct RustAllocator {
    data: *mut c_void,
    capacity: u32,
}

unsafe impl Allocator for RustAllocator {
    #[inline]
    fn new() -> Self {
        Self { data: core::ptr::null_mut(), capacity: 0 }
    }

    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.data.cast_const()
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut c_void {
        self.data.cast()
    }

    #[inline]
    fn capacity(&self) -> u32 {
        self.capacity
    }

    #[inline]
    unsafe fn allocate(&mut self, layout: Layout) -> *mut c_void {
        unsafe { alloc_zeroed(layout).cast::<c_void>() }
    }

    #[inline]
    unsafe fn deallocate(&mut self, ptr: *mut c_void) {
        if !ptr.is_null() {
            unsafe { dealloc(ptr.cast::<u8>(), Self::ptr_layout(self.capacity as usize)) };
        }
    }

    #[inline]
    fn set_allocator_traits(&mut self, data: *mut c_void, capacity: u32, type_size: usize) {
        self.data = data;
        self.capacity = capacity;
        let _ = type_size;
    }
}

use core::alloc::Layout;
use core::ffi::c_void;
use core::ptr::{self};

use crate::re::BSTArray::Allocator;
use crate::re::MemoryManager::{free, malloc};

#[repr(C)]
#[derive(Debug)]
pub struct BSTArrayHeapAllocator {
    data: *mut c_void,
    capacity: u32,
}
const_assert_eq!(core::mem::size_of::<BSTArrayHeapAllocator>(), 0x10);

unsafe impl Allocator for BSTArrayHeapAllocator {
    #[inline]
    fn new() -> Self {
        Self { data: ptr::null_mut(), capacity: 0 }
    }

    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.data.cast_const()
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut c_void {
        self.data
    }

    #[inline]
    fn capacity(&self) -> u32 {
        self.capacity
    }

    #[inline]
    unsafe fn allocate(&mut self, layout: Layout) -> *mut c_void {
        let size = layout.size();
        let mem = unsafe { malloc(size) };
        if !mem.is_null() {
        } else {
            unsafe { ptr::write_bytes(mem, 0, size) };
        }

        mem
    }

    #[inline]
    unsafe fn deallocate(&mut self, ptr: *mut c_void) {
        unsafe { free(ptr) };
    }

    #[inline]
    fn set_allocator_traits(&mut self, data: *mut c_void, capacity: u32, type_size: usize) {
        let _ = type_size;
        self.data = data;
        self.capacity = capacity;
    }
}

impl Default for BSTArrayHeapAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for BSTArrayHeapAllocator {
    fn drop(&mut self) {
        unsafe { self.deallocate(self.data) };
    }
}

impl Clone for BSTArrayHeapAllocator {
    fn clone(&self) -> Self {
        let mut new_alloc = Self::new();

        let src_ptr = self.data;
        if !src_ptr.is_null() {
            let capacity = self.capacity as usize;

            if capacity > 0 {
                let dst_ptr = unsafe { new_alloc.allocate(Self::ptr_layout(capacity)) };
                unsafe { ptr::copy_nonoverlapping(src_ptr, dst_ptr, capacity) };
                new_alloc.data = dst_ptr;
            }
        }

        new_alloc
    }
}

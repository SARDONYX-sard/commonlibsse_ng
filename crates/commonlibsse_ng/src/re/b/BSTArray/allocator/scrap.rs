use core::ffi::c_void;
use core::ptr::{self, copy_nonoverlapping, null_mut};
use std::alloc::Layout;

use crate::re::BSTArray::Allocator;
use crate::re::MemoryManager::MemoryManager;
use crate::re::ScrapHeap::ScrapHeap;

#[derive(Debug)]
pub struct BSScrapArrayAllocator {
    allocator: *mut ScrapHeap,
    data: *mut c_void,
    capacity: u32,
}
const _: () = assert!(core::mem::size_of::<BSScrapArrayAllocator>() == 0x18);

unsafe impl Allocator for BSScrapArrayAllocator {
    #[inline]
    fn new() -> Self {
        Self { allocator: null_mut(), data: null_mut(), capacity: 0 }
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
        if !self.allocator.is_null() {
            if let Some(heap) = unsafe { MemoryManager::GetSingleton().as_mut() } {
                self.allocator = unsafe { heap.GetThreadScrapHeap() };
            };
        }
        assert!(!self.allocator.is_null(), "allocator must not be null");

        let (size, alignment) = (layout.size(), layout.align());
        let mem = unsafe {
            self.allocator.as_mut().map_or(ptr::null_mut(), |heap| heap.allocate(size, alignment))
        };
        assert!(!mem.is_null(), "mem must not be null");
        unsafe { ptr::write_bytes(mem, 0, size) };

        mem
    }

    #[inline]
    unsafe fn deallocate(&mut self, ptr: *mut c_void) {
        if let Some(allocator) = unsafe { self.allocator.as_mut() } {
            if !ptr.is_null() {
                unsafe { allocator.deallocate(ptr) };
            }
        }
    }

    #[inline]
    fn set_allocator_traits(&mut self, data: *mut c_void, capacity: u32, type_size: usize) {
        let _ = type_size;
        self.data = data;
        self.capacity = capacity;
    }
}

impl Default for BSScrapArrayAllocator {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for BSScrapArrayAllocator {
    fn drop(&mut self) {
        unsafe { self.deallocate(self.data) };
    }
}

impl Clone for BSScrapArrayAllocator {
    #[inline]
    fn clone(&self) -> Self {
        let mut new_allocator = Self::new();
        new_allocator.capacity = self.capacity;

        if self.capacity > 0 {
            unsafe {
                const PTR_ALIGN_SIZE: usize = align_of::<*mut c_void>();
                let layout = Layout::from_size_align(self.capacity as usize, PTR_ALIGN_SIZE)
                    .expect("Valid layout");
                new_allocator.data = new_allocator.allocate(layout);
                copy_nonoverlapping(self.data, new_allocator.data, self.capacity as usize);
            }
        }
        new_allocator
    }
}

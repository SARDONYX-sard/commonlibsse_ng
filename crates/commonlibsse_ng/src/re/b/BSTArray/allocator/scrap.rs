use core::ptr::{copy_nonoverlapping, null_mut};
use std::alloc::{Layout, alloc_zeroed, dealloc};

use crate::re::MemoryManager::MemoryManager;
use crate::re::ScrapHeap::ScrapHeap;

#[derive(Debug)]
pub struct BSScrapArrayAllocator {
    allocator: *mut ScrapHeap, // Placeholder for custom allocator
    data: *mut u8,
    capacity: u32,
}

impl BSScrapArrayAllocator {
    pub const fn new() -> Self {
        Self { allocator: null_mut(), data: null_mut(), capacity: 0 }
    }

    pub fn data_mut(&mut self) -> *mut u8 {
        self.data
    }

    pub const fn data(&self) -> *const u8 {
        self.data
    }

    pub const fn capacity(&self) -> u32 {
        self.capacity
    }

    /// Returns allocated the ptr(maybe valid ptr or null ptr).
    pub unsafe fn allocate(&mut self, size: usize) -> *mut u8 {
        if !self.allocator.is_null() {
            if let Some(heap) = unsafe { MemoryManager::GetSingleton().as_mut() } {
                self.allocator = unsafe { heap.GetThreadScrapHeap() };
            };
        }
        assert!(!self.allocator.is_null(), "allocator must not be null");
        let layout = Layout::from_size_align(size, core::mem::align_of::<u8>()).unwrap();

        unsafe { alloc_zeroed(layout) }
    }

    pub fn deallocate(&mut self, ptr: *mut u8) {
        if !ptr.is_null() {
            let layout =
                Layout::from_size_align(self.capacity as usize, std::mem::align_of::<u8>())
                    .unwrap();
            unsafe {
                dealloc(ptr, layout);
            }
        }
    }

    fn set_allocator_traits(&mut self, data: *mut u8, capacity: u32) {
        self.data = data;
        self.capacity = capacity;
    }
}

impl Clone for BSScrapArrayAllocator {
    fn clone(&self) -> Self {
        let mut new_allocator = Self::new();
        new_allocator.capacity = self.capacity;
        if self.capacity > 0 {
            unsafe {
                new_allocator.data = new_allocator.allocate(self.capacity as usize);
                copy_nonoverlapping(self.data, new_allocator.data, self.capacity as usize);
            }
        }
        new_allocator
    }
}

impl Drop for BSScrapArrayAllocator {
    fn drop(&mut self) {
        self.deallocate(self.data);
    }
}

impl Default for BSScrapArrayAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl BSScrapArrayAllocator {
    pub fn assign(&mut self, other: &Self) {
        if self.data != other.data {
            self.deallocate(self.data);
            self.capacity = other.capacity;
            if self.capacity > 0 {
                unsafe {
                    self.data = self.allocate(self.capacity as usize);
                    copy_nonoverlapping(other.data, self.data, self.capacity as usize);
                }
            }
        }
    }

    pub fn move_assign(&mut self, mut other: Self) {
        if self.data != other.data {
            self.deallocate(self.data);
            self.allocator = other.allocator;
            self.data = other.data;
            self.capacity = other.capacity;

            other.allocator = null_mut();
            other.data = null_mut();
            other.capacity = 0;
        }
    }
}

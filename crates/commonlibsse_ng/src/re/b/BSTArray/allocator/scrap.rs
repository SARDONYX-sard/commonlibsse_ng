use core::alloc::Layout;
use core::ffi::c_void;
use core::ptr::{NonNull, copy_nonoverlapping, null_mut};
use std::alloc::handle_alloc_error;

use crate::re::BSTArray::Allocator;
use crate::re::MemoryManager::alloc::scrap_alloc;
use crate::re::ScrapHeap::ScrapHeap;

#[derive(Debug)]
pub struct BSScrapArrayAllocator {
    allocator: Option<NonNull<ScrapHeap>>,
    data: *mut c_void,
    capacity: u32,
}
const _: () = assert!(core::mem::size_of::<BSScrapArrayAllocator>() == 0x18);

unsafe impl Allocator for BSScrapArrayAllocator {
    #[inline]
    fn new() -> Self {
        Self { allocator: None, data: null_mut(), capacity: 0 }
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
        let Ok((scrap_heap, ptr)) = (match self.allocator {
            Some(allocator) => unsafe { scrap_alloc::realloc(allocator, layout) },
            None => unsafe { scrap_alloc::alloc_zeroed(layout) },
        }) else {
            handle_alloc_error(layout)
        };
        self.allocator = Some(scrap_heap);
        ptr.cast().as_ptr()
    }

    #[inline]
    unsafe fn deallocate(&mut self, ptr: *mut c_void) {
        if let Some(allocator) = self.allocator.as_mut() {
            if !ptr.is_null() {
                unsafe { allocator.as_mut().deallocate(ptr) };
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

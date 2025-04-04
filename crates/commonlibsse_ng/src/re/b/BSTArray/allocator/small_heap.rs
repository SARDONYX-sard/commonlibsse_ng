use core::ffi::c_void;
use core::ptr::{self, copy_nonoverlapping};
use std::alloc::Layout;

use crate::re::BSTArray::Allocator;
use crate::re::MemoryManager::{free, malloc};

/// A memory allocator that supports both local (stack-based) and heap-based allocations.
///
/// This allocator is designed to handle memory for arrays with a small fixed size (`N`) on the stack
/// or allocate larger arrays on the heap when necessary. It provides functionality to allocate and deallocate
/// memory dynamically, switching between stack-based and heap-based storage as required.
pub struct BSTSmallArrayHeapAllocator<const N: usize> {
    // The current allocated capacity (in number of elements).
    capacity: u32,
    // Indicates whether the data is stored locally (on the stack).
    // - local: 1 if local, 0 if heap.
    local: u32,
    // The union of local stack data and heap pointer.
    data: Data<N>,
}
const _: () = assert!(core::mem::size_of::<BSTSmallArrayHeapAllocator<8>>() == 0x10);

impl<const N: usize> BSTSmallArrayHeapAllocator<N> {
    /// Returns `true` if the allocator is using local (stack-based) storage, `false` if it's using heap storage.
    pub const fn local(&self) -> bool {
        self.local != 0
    }

    /// Releases the memory, resetting the allocator to use local stack-based storage.
    /// If the allocator was using heap-based storage, it will free the memory.
    #[inline]
    fn release(&mut self) {
        if !self.local() {
            unsafe { free(self.data.heap) };
        }
        unsafe { ptr::write_bytes(self.as_mut_ptr(), 0, self.capacity() as usize) };
        self.capacity = N as u32;
        self.local = 1;
    }
}

unsafe impl<const N: usize> Allocator for BSTSmallArrayHeapAllocator<N> {
    fn new() -> Self {
        Self { capacity: 0, local: 1, data: Data { local: [0; N] } }
    }

    #[inline]
    fn as_ptr(&self) -> *const c_void {
        unsafe { if self.local() { self.data.local.as_ptr().cast() } else { self.data.heap } }
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut c_void {
        unsafe { if self.local() { self.data.local.as_mut_ptr().cast() } else { self.data.heap } }
    }

    #[inline]
    fn capacity(&self) -> u32 {
        self.capacity
    }

    unsafe fn allocate(&mut self, layout: Layout) -> *mut c_void {
        let size = layout.size();
        if size > N {
            let mem = unsafe { malloc(size) };
            if mem.is_null() {
                panic!("out of memory");
            }
            unsafe { ptr::write_bytes(mem, 0, size) };
            mem
        } else {
            unsafe { self.data.local.as_mut_ptr().cast() }
        }
    }

    unsafe fn deallocate(&mut self, ptr: *mut c_void) {
        let local_ptr: *const c_void = unsafe { self.data.local.as_ptr().cast() };
        if local_ptr != ptr {
            unsafe { free(ptr) };
        }
    }

    fn set_allocator_traits(&mut self, data: *mut c_void, capacity: u32, type_size: usize) {
        self.capacity = capacity;
        if (capacity as usize * type_size) > N {
            self.local = 0;
            self.data.heap = data;
        }
    }
}

impl<const N: usize> Default for BSTSmallArrayHeapAllocator<N> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Drop for BSTSmallArrayHeapAllocator<N> {
    #[inline]
    fn drop(&mut self) {
        self.release();
    }
}

impl<const N: usize> Clone for BSTSmallArrayHeapAllocator<N> {
    fn clone(&self) -> Self {
        let mut new_allocator = Self::new();

        {
            let cloned = &mut new_allocator;
            cloned.capacity = self.capacity();
            cloned.local = self.local;

            let capacity = self.capacity() as usize;
            if !cloned.local() {
                let mem = unsafe { malloc(capacity) };
                if mem.is_null() {
                    panic!("heap allocation failed")
                } else {
                    cloned.data.heap = mem;
                }
            };
            unsafe { copy_nonoverlapping(self.as_ptr(), cloned.as_mut_ptr(), capacity) };
        };
        new_allocator
    }
}

impl<const N: usize> core::fmt::Debug for BSTSmallArrayHeapAllocator<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("BSTSmallArrayHeapAllocator");
        s.field("capacity", &self.capacity);
        s.field("kind", if self.local() { &"Local" } else { &"Heap" });

        unsafe {
            s.field(
                "data",
                &format_args!(
                    "Union {{ {} }}",
                    if self.local() {
                        format!("local_data: {:X?}", &self.data.local)
                    } else {
                        format!("heap_ptr: {:p}", self.data.heap)
                    }
                ),
            );

            if !self.local() && !self.data.heap.is_null() {
                let slice = std::slice::from_raw_parts(self.data.heap, self.capacity as usize);
                s.field("heap_data", &format_args!("{:X?}", slice));
            }
        }

        s.finish()
    }
}

/// A union that stores either a heap pointer or a fixed-size array for local storage.
#[repr(C)]
union Data<const N: usize> {
    // Pointer to heap memory.
    heap: *mut c_void,
    // Fixed-size array for local (stack) storage.
    local: [u8; N],
}

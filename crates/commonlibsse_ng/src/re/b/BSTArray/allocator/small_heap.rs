use core::mem::align_of;
use core::ptr::copy_nonoverlapping;
use std::alloc::{Layout, alloc_zeroed, dealloc};

pub struct BSTSmallArrayHeapAllocator<const N: usize> {
    capacity: u32,
    local: u32,
    data: Data<N>,
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

#[repr(C)]
union Data<const N: usize> {
    heap: *mut u8,
    local: [u8; N],
}

impl<const N: usize> BSTSmallArrayHeapAllocator<N> {
    pub const fn new() -> Self {
        Self { capacity: 0, local: 1, data: Data { local: [0; N] } }
    }

    pub const fn local(&self) -> bool {
        self.local != 0
    }

    pub fn data_mut(&mut self) -> *mut u8 {
        unsafe { if self.local() { self.data.local.as_mut_ptr() } else { self.data.heap } }
    }

    pub const fn data(&self) -> *const u8 {
        unsafe { if self.local() { self.data.local.as_ptr() } else { self.data.heap } }
    }

    pub const fn capacity(&self) -> u32 {
        self.capacity
    }

    fn allocate(&mut self, size: usize) -> *mut u8 {
        if size > N {
            let layout = Layout::from_size_align(size, align_of::<u8>()).unwrap();
            let mem = unsafe { alloc_zeroed(layout) };
            if mem.is_null() {
                panic!("out of memory");
            }
            self.local = 0;
            self.data.heap = mem;
            mem
        } else {
            self.local = 1;
            unsafe { self.data.local.as_mut_ptr() }
        }
    }

    fn deallocate(&mut self) {
        if !self.local() {
            let layout = Layout::from_size_align(self.capacity as usize, align_of::<u8>()).unwrap();
            unsafe {
                dealloc(self.data.heap, layout);
            }
        }
    }

    fn copy_from(&mut self, other: &Self) {
        self.deallocate();
        self.capacity = other.capacity;
        self.local = other.local;
        if self.local() {
            unsafe {
                copy_nonoverlapping(other.data.local.as_ptr(), self.data.local.as_mut_ptr(), N);
            }
        } else {
            let layout = Layout::from_size_align(self.capacity as usize, align_of::<u8>()).unwrap();
            let mem = unsafe { alloc_zeroed(layout) };
            if mem.is_null() {
                panic!("out of memory");
            }
            unsafe {
                copy_nonoverlapping(other.data.heap, mem, self.capacity as usize);
            }
            self.data.heap = mem;
        }
    }

    fn move_from(&mut self, mut other: Self) {
        self.deallocate();
        self.capacity = other.capacity;
        self.local = other.local;
        if self.local() {
            unsafe {
                copy_nonoverlapping(other.data.local.as_ptr(), self.data.local.as_mut_ptr(), N);
            }
        } else {
            self.data.heap = unsafe { other.data.heap };
        }
        other.reset();
    }

    fn reset(&mut self) {
        self.deallocate();
        self.capacity = 0;
        self.local = 1;
        self.data.local = [0; N];
    }
}

impl<const N: usize> Drop for BSTSmallArrayHeapAllocator<N> {
    fn drop(&mut self) {
        self.deallocate();
    }
}

impl<const N: usize> Clone for BSTSmallArrayHeapAllocator<N> {
    fn clone(&self) -> Self {
        let mut new_allocator = Self::new();
        new_allocator.copy_from(self);
        new_allocator
    }
}

impl<const N: usize> Default for BSTSmallArrayHeapAllocator<N> {
    fn default() -> Self {
        Self::new()
    }
}

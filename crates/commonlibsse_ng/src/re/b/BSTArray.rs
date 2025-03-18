use core::ffi::c_void;
use core::slice;
use std::alloc::{self, Layout};
use std::marker::PhantomData;
use std::ptr::{self, NonNull};

use crate::re::offsets_rtti::RTTI_BSTArrayBase__IAllocatorFunctor;
use crate::re::offsets_vtable::VTABLE_BSTArrayBase__IAllocatorFunctor;
use crate::rel::id::VariantID;

#[repr(C)]
struct BSTArrayBase {
    pub size: u32,
}
const_assert_eq!(core::mem::size_of::<BSTArrayBase>(), 0x4);

impl BSTArrayBase {
    /// C++ `BSTArrayBase::empty`
    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }
}

#[repr(C)]
struct IAllocatorFunctor {
    vtable_: *const IAllocatorFunctorVtbl,
}
const_assert_eq!(core::mem::size_of::<IAllocatorFunctor>(), 0x8);

impl IAllocatorFunctor {
    pub const RTTI: VariantID = RTTI_BSTArrayBase__IAllocatorFunctor;
    pub const VTABLE: [VariantID; 1] = VTABLE_BSTArrayBase__IAllocatorFunctor;
}

struct IAllocatorFunctorVtbl {
    allocate: fn(this: *mut c_void, num: u32, elem_size: u32) -> bool,
    reallocate: fn(
        this: *mut c_void,
        min_new_size_items: u32,
        front_copy_count: u32,
        back_copy_count: u32,
        elem_size: u32,
    ) -> bool,
    deallocate: fn(this: *mut c_void) -> c_void,
}

#[repr(C)]
pub struct BSTArrayHeapAllocator {
    data: *mut u8,
    capacity: u32,
}
const_assert_eq!(core::mem::size_of::<BSTArrayHeapAllocator>(), 0x10);

impl BSTArrayHeapAllocator {
    pub const fn new() -> Self {
        Self { data: ptr::null_mut(), capacity: 0 }
    }

    /// # Errors
    /// - If size > `isize::MAX`
    /// - If allocation failed
    pub fn allocate(&mut self, size: usize) -> Result<(), AllocatorError> {
        if size == 0 {
            return Ok(());
        }
        let layout = Layout::array::<u8>(size)
            .map_err(|_| AllocatorError::InvalidLayout { requested_size: size })?;
        let mem = unsafe { alloc::alloc_zeroed(layout) };
        if mem.is_null() {
            return Err(AllocatorError::AllocationFailed { layout });
        }
        self.data = mem;
        self.capacity = size as u32;
        Ok(())
    }

    /// # Errors
    /// - If `self.capacity` > `isize::MAX`
    pub fn deallocate(&mut self) -> Result<(), AllocatorError> {
        if !self.data.is_null() {
            let layout = Layout::array::<u8>(self.capacity as usize).map_err(|_| {
                AllocatorError::InvalidLayout { requested_size: self.capacity as usize }
            })?;
            unsafe {
                alloc::dealloc(self.data, layout);
            }
            self.data = ptr::null_mut();
            self.capacity = 0;
        }
        Ok(())
    }

    pub const fn capacity(&self) -> u32 {
        self.capacity
    }

    pub const fn data(&self) -> *const u8 {
        self.data
    }
}

impl Drop for BSTArrayHeapAllocator {
    fn drop(&mut self) {
        let _ = self.deallocate();
    }
}

impl Clone for BSTArrayHeapAllocator {
    fn clone(&self) -> Self {
        let mut new_alloc = Self::new();
        if self.capacity > 0 && matches!(new_alloc.allocate(self.capacity as usize), Ok(())) {
            unsafe {
                ptr::copy_nonoverlapping(self.data, new_alloc.data, self.capacity as usize);
            }
        }
        new_alloc
    }
}

impl Default for BSTArrayHeapAllocator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, snafu::Snafu)]
pub enum AllocatorError {
    /// The heap memory you tried to allocate is too large. The `BSTArrayHeapAllocator` only supports allocating less than isize::MAX but requested {requested_size}.
    InvalidLayout { requested_size: usize },
    /// Heap allocation failed. Layout attempted to allocate: {layout:?}
    AllocationFailed { layout: Layout },
}

pub struct BSTSmallArray<T> {
    pub data: *mut T,
    pub size: usize,
    pub capacity: usize,
}

#[repr(C)]
pub struct BSTArray<T> {
    // BSTArrayHeapALlocator
    data: NonNull<T>,
    capacity: usize,

    // BSTArrayBase
    size: usize,
    _marker: PhantomData<T>,
}

impl<T> Default for BSTArray<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BSTArray<T> {
    pub const fn new() -> Self {
        Self { data: NonNull::dangling(), size: 0, capacity: 0, _marker: PhantomData }
    }

    /// # Panics
    pub fn with_capacity(capacity: usize) -> Self {
        let layout = Layout::array::<T>(capacity).expect("Invalid layout");
        let ptr = unsafe { alloc::alloc(layout).cast::<T>() };
        let data = NonNull::new(ptr).unwrap_or(NonNull::dangling());
        Self { data, size: 0, capacity, _marker: PhantomData }
    }

    pub fn push(&mut self, value: T) {
        if self.size == self.capacity {
            self.grow();
        }
        unsafe {
            self.data.as_ptr().add(self.size).write(value);
        }
        self.size += 1;
    }

    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let layout = Layout::array::<T>(new_capacity).expect("Invalid layout");
        let new_ptr = unsafe { alloc::alloc(layout).cast::<T>() };
        let new_data = NonNull::new(new_ptr).expect("Allocation failed");

        unsafe {
            ptr::copy_nonoverlapping(self.data.as_ptr(), new_data.as_ptr(), self.size);
            let old_layout = Layout::array::<T>(self.capacity).expect("Invalid layout");
            alloc::dealloc(self.data.as_ptr().cast::<u8>(), old_layout);
        }
        self.data = new_data;
        self.capacity = new_capacity;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.size { Some(unsafe { &*self.data.as_ptr().add(index) }) } else { None }
    }
}

impl<T> Drop for BSTArray<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.size {
                ptr::drop_in_place(self.data.as_ptr().add(i));
            }
            let layout = Layout::array::<T>(self.capacity).expect("Invalid layout");
            alloc::dealloc(self.data.as_ptr().cast::<u8>(), layout);
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct BSStaticArray<T> {
    data: NonNull<T>,
    size: u32,
}

impl<T> BSStaticArray<T> {
    /// # Panics
    #[inline]
    pub const fn new(data: *mut T, size: u32) -> Self {
        Self { data: NonNull::new(data).expect("data pointer must not be null"), size }
    }

    #[inline]
    pub const fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.data.as_ptr(), self.size as usize) }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.data.as_ptr(), self.size as usize) }
    }

    #[inline]
    pub const fn len(&self) -> u32 {
        self.size
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }

    #[inline]
    pub fn get(&self, index: u32) -> Option<&T> {
        self.as_slice().get(index as usize)
    }

    #[inline]
    pub fn get_mut(&mut self, index: u32) -> Option<&mut T> {
        self.as_mut_slice().get_mut(index as usize)
    }

    #[inline]
    pub fn front(&self) -> Option<&T> {
        self.get(0)
    }

    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.get_mut(0)
    }

    #[inline]
    pub fn back(&self) -> Option<&T> {
        self.get(self.size - 1)
    }

    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.get_mut(self.size - 1)
    }

    #[inline]
    pub fn iter(&self) -> core::slice::Iter<T> {
        self.as_slice().iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> core::slice::IterMut<T> {
        self.as_mut_slice().iter_mut()
    }
}

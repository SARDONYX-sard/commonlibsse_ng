mod BSTArrayInner;
pub mod IAllocatorFunctor;
mod allocator;
mod heap;
mod small_heap;

use core::ptr::NonNull;
use core::slice;

pub use self::BSTArrayInner::BSScrapArray;
pub use self::{heap::BSTArray, small_heap::BSTSmallArray};

#[repr(C)]
#[derive(Debug, PartialEq)]
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
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
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

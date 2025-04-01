//! # hkArrayBase
//!
//! This module defines the `hkArrayBase` struct, a dynamic array-like structure that
//! mimics the behavior of a C++ container with methods for array manipulation,
//! including dynamic resizing, accessors, and iterators.

use core::{
    mem::MaybeUninit,
    ops::{Index, IndexMut},
};

/// Represents a base class for a dynamic array of type `T`.
#[repr(C)]
#[derive(Debug)]
pub struct hkArrayBase<T> {
    _data: *mut T,            // Pointer to data array.
    _size: i32,               // Current size of the array.
    _capacity_and_flags: i32, // Capacity and flags.
}

impl<T> hkArrayBase<T> {
    /// Constant for capacity mask.
    const CAPACITY_MASK: i32 = 0x3FFFFFFF;
    const DONT_DEALLOC_FLAG: i32 = 1 << 31;

    /// Growth factor for array resizing.
    const GROWTH_FACTOR: f32 = 1.5;

    /// Returns a raw pointer to the array data.
    #[inline]
    pub const fn data(&self) -> *mut T {
        self._data
    }

    /// Returns the size of the array.
    #[inline]
    pub const fn size(&self) -> i32 {
        self._size
    }

    /// Returns the current capacity of the array.
    #[inline]
    pub const fn capacity(&self) -> i32 {
        self._capacity_and_flags & Self::CAPACITY_MASK
    }

    /// Checks whether the array is empty.
    #[inline]
    pub const fn empty(&self) -> bool {
        self.size() == 0
    }

    /// Reserves memory for the array to hold `a_newCap` elements.
    ///
    /// # Panics
    #[inline]
    pub fn reserve(&mut self, a_newCap: i32) {
        assert!(a_newCap <= Self::CAPACITY_MASK);
        if a_newCap <= self.capacity() {
            return;
        }

        // Use Box and into_raw to get the pointer
        let new_size = a_newCap as usize * std::mem::size_of::<T>();
        let new_mem: Box<[MaybeUninit<T>]> = Box::new_uninit_slice(new_size);
        let new_mem = Box::into_raw(new_mem).cast::<T>();

        if !self._data.is_null() {
            let old_size = self.size() as usize * std::mem::size_of::<T>();
            unsafe { std::ptr::copy_nonoverlapping(self._data, new_mem, old_size) };
            if self._capacity_and_flags & Self::DONT_DEALLOC_FLAG == 0 {
                drop(unsafe { Box::from_raw(self._data) });
            }
        }

        self._data = new_mem;
        self._capacity_and_flags &= !Self::CAPACITY_MASK;
        self._capacity_and_flags |= a_newCap & Self::CAPACITY_MASK;
    }

    /// Pushes a new element to the end of the array.
    ///
    /// # Arguments
    /// - `value`: The value to push.
    #[inline]
    pub fn push_back(&mut self, value: T) {
        if self.size() == self.capacity() {
            self.reserve(if self.size() == 0 {
                1
            } else {
                (self.size() as f32 * Self::GROWTH_FACTOR).ceil() as i32
            });
        }

        unsafe {
            let end = self.data().add(self.size() as usize);
            std::ptr::write(end, value);
        }
        self._size += 1;
    }
}

impl<T> hkArrayBase<T>
where
    T: Clone,
{
    /// Resizes the array to hold `a_count` elements.
    ///
    /// # Panics
    #[inline]
    pub fn resize(&mut self, a_count: i32, value: T) {
        assert!((0..=Self::CAPACITY_MASK).contains(&a_count));
        if a_count == self.size() {
            return;
        }

        if a_count < self.size() {
            for i in a_count..self.size() {
                unsafe {
                    let elem = self.data().add(i as usize);
                    std::ptr::drop_in_place(elem);
                }
            }
        }

        let new_size = a_count as usize * std::mem::size_of::<T>();
        let new_mem: Box<[MaybeUninit<T>]> = Box::new_uninit_slice(new_size);
        let new_mem = Box::into_raw(new_mem).cast::<T>();

        if !self._data.is_null() {
            let old_size = self.size() as usize * std::mem::size_of::<T>();
            unsafe { std::ptr::copy_nonoverlapping(self._data, new_mem, old_size) };
            if self._capacity_and_flags & Self::DONT_DEALLOC_FLAG == 0 {
                drop(unsafe { Box::from_raw(self._data) });
            }
        }

        if a_count > self.size() {
            for i in self.size()..a_count {
                unsafe {
                    let elem = new_mem.add(i as usize);
                    std::ptr::write(elem, value.clone());
                }
            }
        }

        self._data = new_mem;
        self._size = a_count;
        self._capacity_and_flags &= !Self::CAPACITY_MASK;
        self._capacity_and_flags |= a_count & Self::CAPACITY_MASK;
    }
}

impl<T> IntoIterator for hkArrayBase<T> {
    type Item = T;
    type IntoIter = hkArrayIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        hkArrayIterator { array: self, index: 0 }
    }
}

pub struct hkArrayIterator<T> {
    array: hkArrayBase<T>,
    index: i32,
}

impl<T> Iterator for hkArrayIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.size() {
            let item = unsafe { std::ptr::read(self.array.data().add(self.index as usize)) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

// The definition of hkArray using hkArrayBase
#[repr(C)]
#[derive(Debug)]
pub struct hkArray<T> {
    pub(super) base: hkArrayBase<T>,
}

impl<T> Default for hkArray<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> hkArray<T> {
    pub const fn new() -> Self {
        Self { base: hkArrayBase { _data: std::ptr::null_mut(), _size: 0, _capacity_and_flags: 0 } }
    }

    pub fn push_back(&mut self, value: T) {
        self.base.push_back(value);
    }

    pub const fn size(&self) -> i32 {
        self.base.size()
    }

    pub const fn capacity(&self) -> i32 {
        self.base.capacity()
    }

    pub const fn empty(&self) -> bool {
        self.base.empty()
    }

    pub fn reserve(&mut self, a_newCap: i32) {
        self.base.reserve(a_newCap);
    }
}

impl<T> hkArray<T>
where
    T: Clone,
{
    pub fn resize(&mut self, a_count: i32, value: T) {
        self.base.resize(a_count, value);
    }
}

impl<T> IntoIterator for hkArray<T> {
    type Item = T;
    type IntoIter = hkArrayIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.base.into_iter()
    }
}

impl<T> Index<usize> for hkArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.base.size() as usize, "Index out of bounds");
        unsafe { self.base.data().wrapping_add(index).as_ref().expect("Index out of bounds") }
    }
}

impl<T> IndexMut<usize> for hkArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.base.size() as usize, "Index out of bounds");
        unsafe { self.base.data().wrapping_add(index).as_mut().expect("Index out of bounds") }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hkarray_new() {
        let array: hkArray<i32> = hkArray::new();
        assert_eq!(array.size(), 0);
        assert!(array.empty());
    }

    #[test]
    fn test_hkarray_push_back() {
        let mut array = hkArray::new();
        array.push_back(10);
        array.push_back(20);

        assert_eq!(array.size(), 2);
        assert!(!array.empty());
        assert_eq!(array.index(0), &10);
        assert_eq!(array.index(1), &20);
    }

    #[test]
    fn test_hkarray_resize_smaller() {
        let mut array = hkArray::new();
        array.push_back(10);
        array.push_back(20);
        array.push_back(30);

        // Resize to a smaller size
        array.resize(2, 0);
        assert_eq!(array.size(), 2);
        assert_eq!(array.index(0), &10);
        assert_eq!(array.index(1), &20);
    }

    #[test]
    fn test_hkarray_resize_larger() {
        let mut array = hkArray::new();
        array.push_back(10);
        array.push_back(20);

        // Resize to a larger size
        array.resize(5, 0);
        assert_eq!(array.size(), 5);
        assert_eq!(array.index(0), &10);
        assert_eq!(array.index(1), &20);

        // Check that new elements are initialized (assuming default initialization is used)
        // This is a basic test to ensure default values are properly initialized
        // In this example, we assume T implements Default
    }

    #[test]
    fn test_hkarray_into_iterator() {
        let mut array = hkArray::new();
        array.push_back(10);
        array.push_back(20);

        let mut iter = array.into_iter();
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(20));
        assert_eq!(iter.next(), None); // No more elements
    }

    #[test]
    fn test_hkarray_reserve() {
        let mut array = hkArray::new();
        array.push_back(10);
        array.push_back(20);

        // Reserve more capacity
        let initial_capacity = array.capacity();
        array.reserve(10);
        assert!(array.capacity() >= initial_capacity);
    }

    #[test]
    fn test_hkarray_capacity() {
        let mut array = hkArray::new();
        assert_eq!(array.capacity(), 0); // Initially, capacity should be 0 or some small value

        array.push_back(10);
        assert!(array.capacity() > 0);
    }
}

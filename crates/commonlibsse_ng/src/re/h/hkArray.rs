//! # hkArrayBase
//!
//! This module defines the `hkArrayBase` struct, a dynamic array-like structure that
//! mimics the behavior of a C++ container with methods for array manipulation,
//! including dynamic resizing, accessors, and iterators.
mod allocator;

pub use self::allocator::{Allocator, RustAllocator, SkyrimAllocator};

use core::{
    marker::PhantomData,
    ops::{Index, IndexMut},
    ptr::{self, NonNull},
};

/// Represents a base class for a dynamic array of type `T`.
#[repr(C)]
#[derive(Debug)]
pub struct hkArrayBase<T, A: Allocator = SkyrimAllocator> {
    data: Option<NonNull<T>>,
    size: i32,
    capacityAndFlags: i32,
    allocator: PhantomData<A>,
}
const _: () = assert!(core::mem::size_of::<hkArrayBase<*mut ()>>() == 0x10);

impl<T> Default for hkArrayBase<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T, A> hkArrayBase<T, A>
where
    A: Allocator,
{
    /// Constant for capacity mask.
    const CAPACITY_MASK: i32 = 0x3FFFFFFF;

    /// Growth factor for array resizing.
    const GROWTH_FACTOR: f32 = 1.5;

    const fn need_dealloc(&self) -> bool {
        const DONT_DEALLOC_FLAG: i32 = 1 << 31;
        self.capacityAndFlags & DONT_DEALLOC_FLAG == 0
    }

    /// # panics
    /// If failed to allocate heap memory.
    fn allocate_new_memory(&self, new_capacity: i32, copy_len: i32) -> NonNull<T> {
        let element_one_size = core::mem::size_of::<T>() as i32;

        let new_mem = unsafe {
            let new_size = new_capacity * element_one_size;
            let ptr = A::alloc_zeroed(new_size).cast::<T>();
            NonNull::new(ptr).expect("[hkArray] new_mem must be non null")
        };

        if let Some(data) = self.data {
            if copy_len > 0 {
                unsafe {
                    ptr::copy_nonoverlapping(data.as_ptr(), new_mem.as_ptr(), copy_len as usize);
                }
            }
            if self.need_dealloc() {
                let old_size = self.len() * element_one_size;
                unsafe { A::free(data.as_ptr().cast(), old_size) };
            }
        }

        new_mem
    }

    /// Clears the capacity bits (preserves flags).
    const fn clear_capacity_bits(&mut self) {
        self.capacityAndFlags &= !Self::CAPACITY_MASK;
    }

    #[inline]
    pub const fn new() -> Self {
        Self { data: None, size: 0, capacityAndFlags: 0, allocator: PhantomData }
    }

    /// Returns a raw pointer to the array data.
    ///
    /// - Equivalent C++ method: `data`
    #[inline]
    pub const fn as_ptr(&self) -> *mut T {
        match self.data {
            Some(p) => p.as_ptr(),
            None => ptr::null_mut(),
        }
    }

    /// Returns the size of the array.
    ///
    /// Not bytes_count. Return `N` of `T * N`
    #[inline]
    pub const fn len(&self) -> i32 {
        self.size
    }

    /// Returns the current capacity of the array.
    #[inline]
    pub const fn capacity(&self) -> i32 {
        self.capacityAndFlags & Self::CAPACITY_MASK
    }

    /// Checks whether the array is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Reserves memory for the array to hold new capacity elements.
    ///
    /// # Panics
    #[inline]
    pub fn reserve(&mut self, new_capacity: i32) {
        assert!(new_capacity <= Self::CAPACITY_MASK);
        if new_capacity <= self.capacity() {
            return;
        }

        let new_mem = self.allocate_new_memory(new_capacity, self.len());
        self.data = Some(new_mem);
        self.clear_capacity_bits();
        self.capacityAndFlags |= new_capacity & Self::CAPACITY_MASK;
    }

    /// Pushes a new element to the end of the array.
    ///
    /// - Equivalent C++ method: `push_back`
    #[inline]
    pub fn push(&mut self, value: T) {
        if self.len() == self.capacity() {
            let new_capacity = if self.is_empty() {
                1
            } else {
                (self.len() as f32 * Self::GROWTH_FACTOR).ceil() as i32
            };
            self.reserve(new_capacity);
        }

        unsafe {
            let end = self.as_ptr().add(self.len() as usize);
            ptr::write(end, value);
        }
        self.size += 1;
    }
}

impl<T, A> hkArrayBase<T, A>
where
    T: Clone,
    A: Allocator,
{
    /// Resizes the array to hold `a_count` elements.
    ///
    /// # Panics
    #[inline]
    pub fn resize(&mut self, new_len: i32, value: T) {
        assert!((0..=Self::CAPACITY_MASK).contains(&new_len));
        if new_len == self.len() {
            return;
        }

        if new_len < self.len() {
            for i in new_len..self.len() {
                unsafe {
                    let elem = self.as_ptr().add(i as usize);
                    ptr::drop_in_place(elem);
                }
            }
        }

        let new_mem = self.allocate_new_memory(new_len, self.len().min(new_len));

        if new_len > self.len() {
            for i in self.len()..new_len {
                unsafe {
                    let elem = new_mem.add(i as usize);
                    ptr::write(elem.as_ptr(), value.clone());
                }
            }
        }

        self.data = Some(new_mem);
        self.size = new_len;
        self.clear_capacity_bits();
        self.capacityAndFlags |= new_len & Self::CAPACITY_MASK;
    }
}

impl<T, A> IntoIterator for hkArrayBase<T, A>
where
    A: Allocator,
{
    type Item = T;
    type IntoIter = hkArrayIterator<T, A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        hkArrayIterator { array: self, index: 0 }
    }
}

pub struct hkArrayIterator<T, A>
where
    A: Allocator,
{
    array: hkArrayBase<T, A>,
    index: i32,
}

impl<T, A> Iterator for hkArrayIterator<T, A>
where
    A: Allocator,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.len() {
            let item = unsafe { self.array.data?.add(self.index as usize).read() };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.array.len() as usize;
        (len, Some(len))
    }
}

// The definition of hkArray using hkArrayBase
#[repr(C)]
#[derive(Debug)]
pub struct hkArray<T, A: Allocator = SkyrimAllocator> {
    pub(super) __base: hkArrayBase<T, A>,
}

impl<T, A> Default for hkArray<T, A>
where
    A: Allocator,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T, A> hkArray<T, A>
where
    A: Allocator,
{
    #[inline]
    pub const fn new() -> Self {
        Self { __base: hkArrayBase::new() }
    }
    /// Pushes a new element to the end of the array.
    ///
    /// - Equivalent C++ method: `push_back`
    #[inline]
    pub fn push(&mut self, value: T) {
        self.__base.push(value);
    }

    /// - Equivalent C++ method: `size`
    #[inline]
    pub const fn len(&self) -> i32 {
        self.__base.len()
    }

    #[inline]
    pub const fn capacity(&self) -> i32 {
        self.__base.capacity()
    }

    /// - Equivalent C++ method: `empty`
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.__base.is_empty()
    }

    /// Reserves memory for the array to hold new capacity elements.
    ///
    /// # Panics
    #[inline]
    pub fn reserve(&mut self, new_capacity: i32) {
        self.__base.reserve(new_capacity);
    }
}

impl<T, A> hkArray<T, A>
where
    T: Clone,
    A: Allocator,
{
    #[inline]
    pub fn resize(&mut self, count: i32, value: T) {
        self.__base.resize(count, value);
    }
}

impl<T, A> IntoIterator for hkArray<T, A>
where
    A: Allocator,
{
    type Item = T;
    type IntoIter = hkArrayIterator<T, A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.__base.into_iter()
    }
}

impl<T, A> Index<usize> for hkArray<T, A>
where
    A: Allocator,
{
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.__base.len() as usize, "Index out of bounds");
        unsafe { self.__base.as_ptr().wrapping_add(index).as_ref().expect("Index out of bounds") }
    }
}

impl<T, A> IndexMut<usize> for hkArray<T, A>
where
    A: Allocator,
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.__base.len() as usize, "Index out of bounds");
        unsafe { self.__base.as_ptr().wrapping_add(index).as_mut().expect("Index out of bounds") }
    }
}

#[cfg(test)]
mod tests {
    use super::{hkArray as hkArray_, *};

    type hkArray<T> = hkArray_<T, RustAllocator>;

    #[test]
    fn test_hk_array_new() {
        let array: hkArray<i32> = hkArray::new();
        assert_eq!(array.len(), 0);
        assert!(array.is_empty());
    }

    #[test]
    fn test_hk_array_push_back() {
        let mut array = hkArray::new();
        array.push(10);
        array.push(20);

        assert_eq!(array.len(), 2);
        assert!(!array.is_empty());
        assert_eq!(array.index(0), &10);
        assert_eq!(array.index(1), &20);
    }

    #[test]
    fn test_hk_array_resize_smaller() {
        let mut array = hkArray::new();
        array.push(10);
        array.push(20);
        array.push(30);

        array.resize(2, 0);
        assert_eq!(array.len(), 2);
        assert_eq!(array.index(0), &10);
        assert_eq!(array.index(1), &20);
    }

    #[test]
    fn test_hk_array_resize_larger() {
        let mut array = hkArray::new();
        array.push(10);
        array.push(20);

        array.resize(5, 0);
        assert_eq!(array.len(), 5);
        assert_eq!(array.index(0), &10);
        assert_eq!(array.index(1), &20);
    }

    #[test]
    fn test_hk_array_into_iterator() {
        let mut array = hkArray::new();
        array.push(10);
        array.push(20);

        let mut iter = array.into_iter();
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(20));
        assert_eq!(iter.next(), None); // No more elements
    }

    #[test]
    fn test_hk_array_reserve() {
        let mut array = hkArray::new();
        array.push(10);
        array.push(20);

        // Reserve more capacity
        let initial_capacity = array.capacity();
        array.reserve(10);
        assert!(array.capacity() >= initial_capacity);
    }

    #[test]
    fn test_hk_array_capacity() {
        let mut array = hkArray::new();
        assert_eq!(array.capacity(), 0); // Initially, capacity should be 0 or some small value

        array.push(10);
        assert!(array.capacity() > 0);
    }
}

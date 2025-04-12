//! # hkArray
//!
//! This module defines the `hkArrayBase` struct, a dynamic array-like structure that
//! mimics the behavior of a C++ container with methods for array manipulation,
//! including dynamic resizing, accessors, and iterators.
mod allocator;
mod array_base;

use core::ops::{Index, IndexMut};

pub use self::allocator::{Allocator, RustAllocator, SkyrimAllocator};
pub use self::array_base::{
    hkArrayBase, hkArrayDrain, hkArrayIterMut, hkArrayIterator, hkArrayRefIterator,
};

/// A dynamic array container C++'s `hkArray`, backed by a custom allocator.
///
/// # Example
/// ```
/// use commonlibsse_ng::re::hkArray::{hkArray, RustAllocator};
///
/// let mut array = hkArray::<i32, RustAllocator>::new();
/// array.push(1);
/// array.push(2);
/// assert_eq!(array.len(), 2);
/// assert_eq!(array[0], 1);
/// ```
///
/// Note: The default allocator (`SkyrimAllocator`) only works in-game.
/// For examples outside of the game, use `RustAllocator`.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct hkArray<T, A: Allocator = SkyrimAllocator> {
    __base: hkArrayBase<T, A>,
}

impl<T, A> hkArray<T, A>
where
    A: Allocator,
{
    /// Creates a new empty array.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::re::hkArray::{hkArray, RustAllocator};
    ///
    /// let array: hkArray<u32, RustAllocator> = hkArray::new();
    /// assert!(array.is_empty());
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self { __base: hkArrayBase::new() }
    }

    #[inline]
    pub fn with_capacity(capacity: i32) -> Self {
        Self { __base: hkArrayBase::with_capacity(capacity) }
    }

    /// Pushes a new element to the end of the array.
    ///
    /// # Panics
    /// Panics if memory allocation fails.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::re::hkArray::{hkArray, RustAllocator};
    ///
    /// let mut array = hkArray::<u32, RustAllocator>::new();
    /// array.push(42);
    /// ```
    #[inline]
    pub fn push(&mut self, value: T) {
        self.__base.push(value);
    }

    /// Returns the number of elements in the array.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::re::hkArray::{hkArray, RustAllocator};
    ///
    /// let array = hkArray::<u32, RustAllocator>::new();
    /// assert_eq!(array.len(), 0);
    /// ```
    #[inline]
    pub const fn len(&self) -> usize {
        self.__base.len()
    }

    /// Returns the capacity of the array.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::re::hkArray::{hkArray, RustAllocator};
    ///
    /// let array = hkArray::<u32, RustAllocator>::new();
    /// assert_eq!(array.capacity(), 0);
    /// ```
    #[inline]
    pub const fn capacity(&self) -> i32 {
        self.__base.capacity()
    }

    /// Checks if the array is empty.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::re::hkArray::{hkArray, RustAllocator};
    ///
    /// let array = hkArray::<u32, RustAllocator>::new();
    /// assert!(array.is_empty());
    /// ```
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.__base.is_empty()
    }

    /// Reserves capacity for at least `new_capacity` elements.
    ///
    /// # Panics
    /// Panics if `new_capacity` exceeds the internal capacity limit.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::re::hkArray::{hkArray, RustAllocator};
    ///
    /// let mut array = hkArray::<u32, RustAllocator>::new();
    /// array.reserve(10);
    /// assert!(array.capacity() >= 10);
    /// ```
    #[inline]
    pub fn reserve(&mut self, new_capacity: i32) {
        self.__base.reserve(new_capacity);
    }

    #[inline]
    pub const fn get(&self, index: usize) -> Option<&T> {
        self.__base.get(index)
    }

    #[inline]
    pub const fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.__base.get_mut(index)
    }

    /// Checks if the array contains the given element.
    ///
    /// Returns `true` if the element is present in the array, and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use commonlibsse_ng::re::hkArray::{hkArray, RustAllocator};
    ///
    /// let mut array = hkArray::<i32, RustAllocator>::with_capacity(10);
    /// array.push(1);
    /// array.push(2);
    /// assert!(array.contains(&1));
    /// assert!(!array.contains(&3));
    /// ```
    #[inline]
    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        for i in 0..self.len() {
            if let Some(item) = self.get(i) {
                if item == value {
                    return true;
                }
            }
        }
        false
    }

    /// Retains only the elements that satisfy the predicate.
    ///
    /// This method takes a closure that accepts an element of the array and returns a boolean.
    /// Elements for which the closure returns `true` will be kept, while elements for which
    /// it returns `false` will be removed.
    ///
    /// # Examples
    ///
    /// ```
    /// use commonlibsse_ng::re::hkArrayBase::{hkArrayBase, RustAllocator};
    ///
    /// let mut array = hkArrayBase::<i32, RustAllocator>::with_capacity(10);
    /// array.push(1);
    /// array.push(2);
    /// array.push(3);
    /// array.retain(|&x| x > 1);
    /// assert_eq!(array.len(), 2);
    /// assert!(array.contains(&2));
    /// assert!(array.contains(&3));
    /// ```
    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.__base.retain(f);
    }

    /// Removes a range of elements from the array, returning them as a vector.
    ///
    /// This method removes the elements within the specified range and returns them as
    /// a `Vec<T>`. The range must be within the bounds of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use commonlibsse_ng::re::hkArray::{hkArray, RustAllocator};
    ///
    /// let mut array = hkArray::<i32, RustAllocator>::with_capacity(10);
    /// array.push(1);
    /// array.push(2);
    /// array.push(3);
    /// array.push(4);
    /// array.push(5);
    /// let drained = array.drain(0..2);
    /// assert_eq!(drained.collect::<Vec<_>>(), vec![1, 2]);
    /// assert_eq!(array.len(), 3);
    /// assert_eq!(array[0], 3);
    /// assert_eq!(array[1], 4);
    /// assert_eq!(array[2], 5);
    /// ```
    ///
    /// # Panics
    /// Panics if the range is out of bounds.
    #[inline]
    pub fn drain<R>(&mut self, range: R) -> hkArrayDrain<'_, T, A>
    where
        R: core::ops::RangeBounds<usize>,
    {
        self.__base.drain(range)
    }
}

impl<T, A> hkArray<T, A>
where
    T: Clone,
    A: Allocator,
{
    /// Resizes the array to the specified length, filling new elements with the given value.
    ///
    /// # Panics
    /// Panics if `count` is negative or exceeds the maximum allowed capacity.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::re::hkArray::{hkArray, RustAllocator};
    ///
    /// let mut array = hkArray::<u32, RustAllocator>::new();
    /// array.resize(3, 7);
    /// assert_eq!(array.len(), 3);
    /// assert_eq!(array[0], 7);
    /// ```
    #[inline]
    pub fn resize(&mut self, count: i32, value: T) {
        self.__base.resize(count, value);
    }
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
        &self.__base[index]
    }
}

impl<T, A> IndexMut<usize> for hkArray<T, A>
where
    A: Allocator,
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.__base[index]
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

use core::{
    ffi::c_void,
    marker::PhantomData,
    ops::{Index, IndexMut, Range, RangeBounds},
    ptr::{self, NonNull},
    slice,
};

use crate::re::BSTArray::allocator::Allocator;

use super::allocator::BSScrapArrayAllocator;

#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BSTArrayBase {
    pub size: u32,
}
const_assert_eq!(core::mem::size_of::<BSTArrayBase>(), 0x4);

impl BSTArrayBase {
    /// Create a empty array size.
    #[inline]
    pub const fn new() -> Self {
        Self { size: 0 }
    }
}

///  `BSScrapArray<T, A>` is a growable array type that provides a dynamic array
///
/// Internally, the array stores:
/// - a pointer to the data buffer
/// - the number of elements (`len`)
/// - the capacity of the allocation (`cap`)
///
/// # Features
///
/// - Compatible with Havok's binary layout for `BSTArray<T>`
/// - Supports growable or fixed-capacity semantics
/// - Custom allocator support (`A: Allocator`)
/// - Methods similar to `Vec<T>`
///
/// # Panics
///
/// Most methods panic under the following conditions:
///
/// - Indexing out of bounds (`array[index]`)
/// - Reserving more capacity than is possible
/// - Pushing to a full fixed-capacity array (if applicable)
///
/// # Example
///
/// ```no_run
/// use commonlibsse_ng::re::BSTArray::BSScrapArray;
///
/// let mut array = BSScrapArray::<i32>::new();
/// array.push(1);
/// array.push(2);
/// assert_eq!(array.len(), 2);
/// assert_eq!(array[1], 2);
/// ```
///
/// # See also
///
/// - [`Vec<T>`]
/// - [`Box<[T]>`]
#[repr(C)]
pub struct BSScrapArray<T, A = BSScrapArrayAllocator>
where
    A: Allocator,
{
    __base: A,
    __base1: BSTArrayBase,
    _marker: PhantomData<T>,
}
const _: () = assert!(core::mem::size_of::<BSScrapArray<u8>>() == 0x20);

impl<T, A> BSScrapArray<T, A>
where
    A: Allocator,
{
    /// Creates a new, empty `BSTArray<T, A>` with the specified allocator.
    ///
    /// The array will not allocate until elements are pushed.
    ///
    /// # Example
    /// ```no_run
    /// use commonlibsse_ng::re::BSTArray::BSScrapArray;
    ///
    /// let array = BSScrapArray::<i32>::new();
    /// assert!(array.is_empty());
    /// ```
    pub fn new() -> Self {
        Self { __base: A::new(), __base1: BSTArrayBase::new(), _marker: PhantomData }
    }

    /// Creates a new, empty `BSTArray<T, A>` with the capacity.
    ///
    /// # Example
    /// ```no_run
    /// use commonlibsse_ng::re::BSTArray::BSScrapArray;
    ///
    /// let array = BSScrapArray::<i32>::with_capacity(5);
    /// assert_eq!(array.capacity(), 5);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        let mut allocator = A::new();

        let new_data = unsafe { allocator.allocate(A::ptr_layout(capacity)) };
        let capacity = capacity as u32;
        Self::set_allocator_traits(&mut allocator, new_data, capacity);

        Self { __base: allocator, __base1: BSTArrayBase::new(), _marker: PhantomData }
    }

    /// Returns the number of elements in the array.
    ///
    /// This is also referred to as the array’s "length".
    ///
    /// # Example
    /// ```no_run
    /// use commonlibsse_ng::re::BSTArray::BSScrapArray;
    ///
    /// let array = BSScrapArray::<i32>::new();
    /// assert_eq!(array.len(), 0);
    /// ```
    #[inline]
    pub const fn len(&self) -> usize {
        self.__base1.size as usize
    }

    /// Returns `true` if the array contains no elements.
    ///
    /// # Example
    /// ```no_run
    /// use commonlibsse_ng::re::BSTArray::BSScrapArray;
    ///
    /// let array = BSScrapArray::<i32>::new();
    /// assert!(array.is_empty());
    /// ```
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the total number of elements the array can hold without reallocating.
    ///
    /// This is the allocated capacity, which may be larger than the current length.
    ///
    /// # Example
    /// ```no_run
    /// use commonlibsse_ng::re::BSTArray::BSScrapArray;
    ///
    /// let mut array = BSScrapArray::<i32>::with_capacity(10);
    /// assert!(array.capacity() >= 10);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.__base.capacity() as usize
    }

    /// Shrinks the capacity of the array as much as possible.
    ///
    /// It will drop any excess capacity not used by the current elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use commonlibsse_ng::re::BSTArray::BSScrapArray;
    ///
    /// let mut array = BSScrapArray::<i32>::with_capacity(10);
    /// array.push(1);
    /// assert_eq!(array.len(), 1);
    /// array.shrink_to_fit();
    /// assert!(array.capacity() >= array.len());
    /// ```
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        let len = self.len();
        self.change_capacity(len);
    }

    /// Appends an element to the back of the array.
    ///
    /// # Panics
    /// Panics if the array is at fixed capacity and cannot grow.
    ///
    /// # Example
    /// ```no_run
    /// use commonlibsse_ng::re::BSTArray::BSScrapArray;
    ///
    /// let mut array = BSScrapArray::<i32>::new();
    /// array.push(5);
    /// assert_eq!(array[0], 5);
    /// ```
    #[inline]
    pub fn push(&mut self, value: T) {
        let size = self.__base1.size;
        if size == self.__base.capacity() {
            self.grow();
        }
        unsafe {
            self.as_mut_ptr().add(size as usize).write(value);
        }

        self.__base1.size += 1;
    }

    /// Removes the last element from the array and returns it, or `None` if it's empty.
    ///
    /// # Example
    /// ```no_run
    /// use commonlibsse_ng::re::BSTArray::BSScrapArray;
    ///
    /// let mut array = BSScrapArray::<i32>::new();
    /// array.push(1);
    /// assert_eq!(array.pop(), Some(1));
    /// assert_eq!(array.pop(), None);
    /// ```
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        let len = self.len();
        if len == 0 {
            None
        } else {
            self.__base1.size -= 1;
            unsafe { Some(ptr::read(self.as_ptr().add(len - 1))) }
        }
    }

    /// Returns a reference to the element at the given index, if it exists.
    ///
    /// # Example
    /// ```no_run
    /// use commonlibsse_ng::re::BSTArray::BSScrapArray;
    ///
    /// let mut array = BSScrapArray::<i32>::new();
    /// array.push(42);
    /// assert_eq!(array.get(0), Some(&42));
    /// assert_eq!(array.get(1), None);
    /// ```
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len() {
            return unsafe { self.as_ptr().add(index).as_ref() };
        }
        None
    }

    /// Returns a mutable reference to the element at the given index, if it exists.
    ///
    /// # Example
    /// ```no_run
    /// use commonlibsse_ng::re::BSTArray::BSScrapArray;
    ///
    /// let mut array = BSScrapArray::<i32>::new();
    /// array.push(10);
    /// if let Some(x) = array.get_mut(0) {
    ///     *x += 1;
    /// }
    /// assert_eq!(array[0], 11);
    /// ```
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len() {
            return unsafe { self.as_mut_ptr().add(index).as_mut() };
        }
        None
    }

    /// Clears the array, removing all elements but preserving the capacity.
    ///
    /// # Examples
    /// ```
    /// use commonlibsse_ng::re::BSTArray::BSScrapArray;
    ///
    /// let mut array = BSScrapArray::<i32>::with_capacity(10);
    /// array.push(1);
    /// array.push(2);
    /// assert_eq!(array.len(), 2);
    /// array.clear();
    /// assert_eq!(array.len(), 0);
    /// assert_eq!(array.capacity(), 10); // Capacity is preserved
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        // Drop all elements in the array without changing capacity
        for i in 0..self.len() {
            unsafe {
                // SAFETY: we're dropping each element in place
                ptr::drop_in_place(self.as_mut_ptr().add(i));
            }
        }

        self.__base1.size = 0; // Reset the length, but keep the allocated capacity
    }

    /// Returns a raw pointer to the array’s buffer.
    ///
    /// This is useful for FFI or direct memory manipulation.
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.__base.as_ptr().cast()
    }

    /// Returns a mutable raw pointer to the array’s buffer.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.__base.as_mut_ptr().cast()
    }

    /// Checks if the array contains the given element.
    ///
    /// Returns `true` if the element is present in the array, and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use commonlibsse_ng::re::BSTArray::BSScrapArray;
    ///
    /// let mut array = BSScrapArray::<i32>::with_capacity(10);
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
    /// ```no_run
    /// use commonlibsse_ng::re::BSTArray::BSScrapArray;
    ///
    /// let mut array = BSScrapArray::<i32>::with_capacity(10);
    /// array.push(1);
    /// array.push(2);
    /// array.push(3);
    /// array.retain(|&x| x > 1);
    /// assert_eq!(array.len(), 2);
    /// assert!(array.contains(&2));
    /// assert!(array.contains(&3));
    /// ```
    #[inline]
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut retained = 0;

        for i in 0..self.len() {
            let elem = match unsafe { self.as_ptr().add(i).as_ref() } {
                Some(elem) => elem,
                None => continue,
            };

            if f(elem) {
                if retained != i {
                    unsafe {
                        let src = self.as_ptr().add(i);
                        let dst = self.as_mut_ptr().add(retained);
                        ptr::copy_nonoverlapping(src, dst, 1);
                    }
                }
                retained += 1;
            } else {
                // Drop elements that do not match the predicate
                unsafe { ptr::drop_in_place(self.as_mut_ptr().add(i)) };
            }
        }

        self.__base1.size = retained as u32;
    }

    /// Resizes the array to the specified length.
    ///
    /// If the array is resized to a larger length, the new elements will be initialized
    /// using the default constructor for `T`. If the array is resized to a smaller length,
    /// elements at the end will be dropped.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use commonlibsse_ng::re::BSTArray::BSScrapArray;
    ///
    /// let mut array = BSScrapArray::<i32>::with_capacity(10);
    /// array.push(1);
    /// array.push(2);
    /// array.resize(5, 0);
    /// assert_eq!(array.len(), 5);
    /// assert_eq!(array[3], 0);
    /// ```
    #[inline]
    pub fn resize(&mut self, new_size: usize, value: T)
    where
        T: Clone,
    {
        let prev_size = self.len();
        if new_size > prev_size {
            for _ in prev_size..new_size {
                self.push(value.clone());
            }
        } else {
            for i in new_size..prev_size {
                unsafe { ptr::drop_in_place(self.as_mut_ptr().add(i)) };
            }
        }
        self.__base1.size = new_size as u32;
    }

    /// Removes a range of elements from the array, returning them as a vector.
    ///
    /// This method removes the elements within the specified range and returns them as
    /// a `Vec<T>`. The range must be within the bounds of the array.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use commonlibsse_ng::re::BSTArray::BSScrapArray;
    ///
    /// let mut array = BSScrapArray::<i32>::with_capacity(10);
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
    pub fn drain<R>(&mut self, range: R) -> BSTDrain<'_, T, A>
    where
        R: RangeBounds<usize>,
    {
        let len = self.len();
        let Range { start, end } = stdx::slice::range(range, ..len);
        debug_assert!(start <= end);
        debug_assert!(end <= len);

        // Need this.
        // If the size is not changed before creating iter for Drain, inconsistencies will occur.
        self.__base1.size = start as u32;

        BSTDrain {
            iter: unsafe { core::slice::from_raw_parts(self.as_ptr().add(start), end - start) }
                .iter(),
            tail_start: end,
            tail_len: len - end,
            array: unsafe { NonNull::new_unchecked(self as *mut Self) },
        }
    }

    /// Returns a slice of all elements in the array.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        let ptr = self.as_ptr();
        let len = self.len();

        if ptr.is_null() || (len == 0) {
            return &[];
        }
        unsafe { slice::from_raw_parts(ptr, len) }
    }

    /// Returns a mutable slice of all elements in the array.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        let ptr = self.as_mut_ptr();
        let len = self.len();

        if ptr.is_null() || (len == 0) {
            return &mut [];
        }
        unsafe { slice::from_raw_parts_mut(ptr, len) }
    }

    /// Returns an iterator over the elements of the array.
    ///
    /// This iterator yields references to the elements in the array.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use commonlibsse_ng::re::BSTArray::BSScrapArray;
    ///
    /// let mut array = BSScrapArray::<i32>::with_capacity(10);
    /// array.push(1);
    /// array.push(2);
    /// let sum: i32 = array.iter().sum();
    /// assert_eq!(sum, 3);
    /// ```
    #[inline]
    pub const fn iter(&self) -> BSTArrayIterator<'_, T, A> {
        BSTArrayIterator { array: self, index: 0 }
    }

    fn grow(&mut self) {
        const MIN_CAPACITY: usize = 4;
        const GROWTH_FACTOR: usize = 2;

        let old_capacity = self.capacity();
        let new_capacity =
            if old_capacity == 0 { MIN_CAPACITY } else { old_capacity * GROWTH_FACTOR };
        self.change_capacity(new_capacity);
    }

    fn change_capacity(&mut self, new_capacity: usize) {
        let new_data = if new_capacity > 0 {
            let layout = A::ptr_layout(new_capacity);
            unsafe { self.__base.allocate(layout).cast::<T>() }
        } else {
            ptr::null_mut()
        };

        let old_data = self.__base.as_mut_ptr().cast::<T>();
        if !old_data.is_null() {
            let old_capacity = self.capacity();
            if !new_data.is_null() {
                let copy_count = core::cmp::min(old_capacity, new_capacity);
                // Safety: There is no uninitialized location because allocate is 0 filled.
                unsafe { ptr::copy_nonoverlapping(old_data, new_data, copy_count) };
            }

            unsafe { self.__base.deallocate(old_data.cast()) };
        }

        Self::set_allocator_traits(&mut self.__base, new_data.cast(), new_capacity as u32);
    }

    fn set_allocator_traits(allocator: &mut A, data: *mut c_void, capacity: u32) {
        allocator.set_allocator_traits(data, capacity, core::mem::size_of::<T>());
    }
}

impl<T, A> Index<usize> for BSScrapArray<T, A>
where
    A: Allocator,
{
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len(), "Index out of bounds");
        unsafe { &*self.as_ptr().add(index) }
    }
}

impl<T, A> IndexMut<usize> for BSScrapArray<T, A>
where
    A: Allocator,
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.len(), "Index out of bounds");
        unsafe { &mut *self.as_mut_ptr().add(index) }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Iterator

/// Iterator returned by `BSTArray::drain()`
pub struct BSTDrain<'a, T, A>
where
    A: Allocator,
{
    tail_start: usize, // = range.end
    tail_len: usize,   // = original_len - range.end
    iter: core::slice::Iter<'a, T>,
    array: NonNull<BSScrapArray<T, A>>,
}

impl<T, A> Iterator for BSTDrain<'_, T, A>
where
    A: Allocator,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| unsafe { ptr::read(item) })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T, A> DoubleEndedIterator for BSTDrain<'_, T, A>
where
    A: Allocator,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|item| unsafe { ptr::read(item) })
    }
}

impl<T, A> ExactSizeIterator for BSTDrain<'_, T, A>
where
    A: Allocator,
{
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T, A: Allocator> Drop for BSTDrain<'_, T, A> {
    fn drop(&mut self) {
        // Copyright (c) 2018 The Servo Project Developers
        // SPDX-License-Identifier: Apache-2.0 OR MIT
        // https://github.com/servo/rust-smallvec/blob/v2/src/lib.rs#L3

        if core::mem::needs_drop::<T>() {
            self.for_each(drop);
        }

        // Copy backward data not subject to drain to the drained start location
        if self.tail_len > 0 {
            unsafe {
                let array = self.array.as_mut();

                let start = array.len();
                let tail = self.tail_start;
                if tail != start {
                    let ptr = array.as_mut_ptr();
                    let src = ptr.add(tail);
                    let dst = ptr.add(start);
                    ptr::copy(src, dst, self.tail_len);
                }
                array.__base1.size = (start + self.tail_len) as u32;
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct BSTArrayIterator<'a, T, A>
where
    A: Allocator,
{
    array: &'a BSScrapArray<T, A>,
    index: usize,
}

impl<'a, T, A> Iterator for BSTArrayIterator<'a, T, A>
where
    A: Allocator,
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.len() {
            let item = unsafe { &*self.array.as_ptr().add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

pub struct BSTArrayIntoIterator<T, A>
where
    A: Allocator,
{
    array: BSScrapArray<T, A>,
    index: usize,
}

impl<T, A> Iterator for BSTArrayIntoIterator<T, A>
where
    A: Allocator,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.len() {
            let item = unsafe { ptr::read(self.array.as_ptr().add(self.index)) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<T, A> IntoIterator for BSScrapArray<T, A>
where
    A: Allocator,
{
    type Item = T;
    type IntoIter = BSTArrayIntoIterator<T, A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        BSTArrayIntoIterator { array: self, index: 0 }
    }
}

impl<'a, T, A> IntoIterator for &'a BSScrapArray<T, A>
where
    A: Allocator,
{
    type Item = &'a T;
    type IntoIter = BSTArrayIterator<'a, T, A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        BSTArrayIterator { array: self, index: 0 }
    }
}

pub struct BSTArrayIterMut<'a, T, A>
where
    A: Allocator,
{
    array: &'a mut BSScrapArray<T, A>,
    index: usize,
}

impl<'a, T, A> Iterator for BSTArrayIterMut<'a, T, A>
where
    A: Allocator,
{
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.len() {
            unsafe {
                let ptr = self.array.as_mut_ptr().add(self.index);
                self.index += 1;
                Some(&mut *ptr)
            }
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.array.len();
        (len, Some(len))
    }
}

impl<'a, T, A> IntoIterator for &'a mut BSScrapArray<T, A>
where
    A: Allocator,
{
    type Item = &'a mut T;
    type IntoIter = BSTArrayIterMut<'a, T, A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        BSTArrayIterMut { array: self, index: 0 }
    }
}

impl<T, A> Extend<T> for BSScrapArray<T, A>
where
    A: Allocator,
{
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for elem in iter {
            self.push(elem);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Standard derive

impl<T, A> core::fmt::Debug for BSScrapArray<T, A>
where
    T: core::fmt::Debug,
    A: Allocator,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.as_slice()).finish()
    }
}

impl<T, A> Default for BSScrapArray<T, A>
where
    A: Allocator,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T, A> Clone for BSScrapArray<T, A>
where
    A: Allocator + Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        if self.__base.capacity() == 0 {
            return Self::new();
        }

        Self {
            __base: self.__base.clone(),
            __base1: BSTArrayBase { size: self.__base1.size },
            _marker: PhantomData,
        }
    }
}

impl<T, A> PartialEq for BSScrapArray<T, A>
where
    T: PartialEq,
    A: Allocator,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}
impl<T, A> PartialEq<Vec<T>> for BSScrapArray<T, A>
where
    T: PartialEq,
    A: Allocator,
{
    #[inline]
    fn eq(&self, other: &Vec<T>) -> bool {
        self.as_slice() == *other
    }
}

impl<T, A> PartialEq<&[T]> for BSScrapArray<T, A>
where
    T: PartialEq,
    A: Allocator,
{
    #[inline]
    fn eq(&self, other: &&[T]) -> bool {
        self.as_slice() == *other
    }
}

impl<T, A> Eq for BSScrapArray<T, A>
where
    T: Eq,
    A: Allocator,
{
}

impl<T, A> PartialOrd for BSScrapArray<T, A>
where
    T: PartialOrd,
    A: Allocator,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl<T, A> Ord for BSScrapArray<T, A>
where
    T: Ord,
    A: Allocator,
{
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<T, A> core::hash::Hash for BSScrapArray<T, A>
where
    T: core::hash::Hash,
    A: Allocator,
{
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

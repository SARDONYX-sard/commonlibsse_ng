use core::{
    alloc::Layout,
    marker::PhantomData,
    ops::{Index, IndexMut, Range, RangeBounds},
    ptr::{self, NonNull},
};
use std::alloc::handle_alloc_error;

use stdx::unique::Unique;

use crate::re::MemoryManager::{TESGlobalAlloc, selfless_alloc::allocator::SelflessAllocator};

/// A binary-compatible, growable array used in Havok serialization.
///
/// `BSTArray<T, A>` is a contiguous, heap-allocated collection of elements of type `T`
/// with memory layout designed to match Havok's native `BSTArray`. This type is similar
/// to [`Vec<T>`] in usage but may differ in layout due to alignment, padding, or
/// platform-specific serialization constraints.
///
/// This array uses a custom allocator `A` (which implements [`Allocator`]) to control
/// memory allocation. In contexts where allocation is fixed (e.g., read-only arrays or
/// statically-sized blocks), capacity may be fixed or omitted entirely.
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
/// # Safety
///
/// This type may be `#[repr(C)]` or `#[repr(transparent)]` depending on your implementation,
/// and is intended to be FFI-safe and bitwise-deserializable when used correctly. Use
/// caution when modifying the layout or manually constructing instances.
///
/// # Example
///
/// ```rust
/// # use commonlibsse_ng::re::BSTArray::BSTArray as BSTArray_;
/// # type BSTArray<T> = BSTArray_<T, stdx::alloc::Global>;
///
/// let mut array = BSTArray::<i32>::new();
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
/// - Havok documentation for `BSTArray<T>` layout.
#[repr(C)]
pub struct BSTArray<T, A = TESGlobalAlloc>
where
    A: SelflessAllocator,
{
    // BSTArrayHeapAllocator
    /// null check safe `*mut T`.
    data: Option<Unique<T>>, // 0x000
    capacity: u32, // 0x004,

    // Unlike the C++ implementation, the pad must be manually inserted
    // because BSTArrayHeapAllocator was inlined.
    pad08: u32, // 0x008,

    // BSTArrayBase
    // length of elements
    size: u32, // 0x010

    // Assumed Zero size type.
    alloc: PhantomData<A>,
}
const _: () = assert!(core::mem::size_of::<BSTArray<()>>() == 0x18);

impl<T, A> BSTArray<T, A>
where
    A: SelflessAllocator,
{
    /// Creates a new, empty `BSTArray<T, A>` with the specified allocator.
    ///
    /// The array will not allocate until elements are pushed.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::BSTArray::BSTArray as BSTArray_;
    /// # type BSTArray<T> = BSTArray_<T, stdx::alloc::Global>;
    ///
    /// let array = BSTArray::<i32>::new();
    /// assert!(array.is_empty());
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self { data: None, capacity: 0, pad08: 0, size: 0, alloc: PhantomData }
    }

    /// Creates a new, empty `BSTArray<T, A>` with the capacity.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::BSTArray::BSTArray as BSTArray_;
    /// # type BSTArray<T> = BSTArray_<T, stdx::alloc::Global>;
    ///
    /// let array = BSTArray::<i32>::with_capacity(5);
    /// assert_eq!(array.capacity(), 5);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        let new_data = A::allocate(Self::new_layout(capacity)).ok();
        let data = new_data.map(|p| unsafe { Unique::new_unchecked(p.cast::<T>().as_ptr()) });
        let capacity = capacity as u32;

        Self { data, capacity, pad08: 0, size: 0, alloc: PhantomData }
    }

    /// Returns the number of elements in the array.
    ///
    /// This is also referred to as the array’s "length".
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::BSTArray::BSTArray as BSTArray_;
    /// # type BSTArray<T> = BSTArray_<T, stdx::alloc::Global>;
    ///
    /// let array = BSTArray::<i32>::new();
    /// assert_eq!(array.len(), 0);
    /// ```
    #[inline]
    pub const fn len(&self) -> usize {
        self.size as usize
    }

    /// Returns `true` if the array contains no elements.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::BSTArray::BSTArray as BSTArray_;
    /// # type BSTArray<T> = BSTArray_<T, stdx::alloc::Global>;
    ///
    /// let array = BSTArray::<i32>::new();
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
    /// ```
    /// # use commonlibsse_ng::re::BSTArray::BSTArray as BSTArray_;
    /// # type BSTArray<T> = BSTArray_<T, stdx::alloc::Global>;
    ///
    /// let mut array = BSTArray::<i32>::with_capacity(10);
    /// assert!(array.capacity() >= 10);
    /// ```
    #[inline]
    pub const fn capacity(&self) -> usize {
        self.capacity as usize
    }

    /// Shrinks the capacity of the array as much as possible.
    ///
    /// It will drop any excess capacity not used by the current elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use commonlibsse_ng::re::BSTArray::BSTArray as BSTArray_;
    /// # type BSTArray<T> = BSTArray_<T, stdx::alloc::Global>;
    ///
    /// let mut array = BSTArray::<i32>::with_capacity(10);
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
    /// ```
    /// # use commonlibsse_ng::re::BSTArray::BSTArray as BSTArray_;
    /// # type BSTArray<T> = BSTArray_<T, stdx::alloc::Global>;
    ///
    /// let mut array = BSTArray::<i32>::new();
    /// array.push(5);
    /// assert_eq!(array[0], 5);
    /// ```
    #[inline]
    pub fn push(&mut self, value: T) {
        let size = self.size;
        if size == self.capacity {
            self.grow();
        }
        unsafe {
            if let Some(ptr) = self.as_non_null_ptr() {
                ptr.add(size as usize).write(value);
            }
        }

        self.size += 1;
    }

    /// Removes the last element from the array and returns it, or `None` if it's empty.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::BSTArray::BSTArray as BSTArray_;
    /// # type BSTArray<T> = BSTArray_<T, stdx::alloc::Global>;
    ///
    /// let mut array = BSTArray::<i32>::new();
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
            self.size -= 1;
            unsafe { Some(ptr::read(self.as_non_null_ptr()?.add(len - 1).as_ptr())) }
        }
    }

    /// Returns a reference to the element at the given index, if it exists.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::BSTArray::BSTArray as BSTArray_;
    /// # type BSTArray<T> = BSTArray_<T, stdx::alloc::Global>;
    ///
    /// let mut array = BSTArray::<i32>::new();
    /// array.push(42);
    /// assert_eq!(array.get(0), Some(&42));
    /// assert_eq!(array.get(1), None);
    /// ```
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len() {
            return unsafe { Some(self.as_non_null_ptr()?.add(index).as_ref()) };
        }
        None
    }

    /// Returns a mutable reference to the element at the given index, if it exists.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::BSTArray::BSTArray as BSTArray_;
    /// # type BSTArray<T> = BSTArray_<T, stdx::alloc::Global>;
    ///
    /// let mut array = BSTArray::<i32>::new();
    /// array.push(10);
    /// if let Some(x) = array.get_mut(0) {
    ///     *x += 1;
    /// }
    /// assert_eq!(array[0], 11);
    /// ```
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len() {
            return unsafe { Some(self.as_non_null_ptr()?.add(index).as_mut()) };
        }
        None
    }

    /// Clears the array, removing all elements but preserving the capacity.
    ///
    /// # Examples
    /// ```
    /// # use commonlibsse_ng::re::BSTArray::BSTArray as BSTArray_;
    /// # type BSTArray<T> = BSTArray_<T, stdx::alloc::Global>;
    ///
    /// let mut array = BSTArray::<i32>::with_capacity(10);
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
        if let Some(non_null) = self.as_non_null_ptr() {
            for i in 0..self.len() {
                unsafe {
                    // SAFETY: we're dropping each element in place
                    ptr::drop_in_place(non_null.add(i).as_ptr());
                }
            }
        }

        self.size = 0; // Reset the length, but keep the allocated capacity
    }

    /// Returns a non null pointer of the array’s buffer.
    #[inline]
    pub const fn as_non_null_ptr(&self) -> Option<NonNull<T>> {
        match self.data {
            Some(p) => Some(p.as_non_null_ptr()),
            None => None,
        }
    }

    /// Checks if the array contains the given element.
    ///
    /// Returns `true` if the element is present in the array, and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use commonlibsse_ng::re::BSTArray::BSTArray as BSTArray_;
    /// # type BSTArray<T> = BSTArray_<T, stdx::alloc::Global>;
    ///
    /// let mut array = BSTArray::<i32>::with_capacity(10);
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
    /// # use commonlibsse_ng::re::BSTArray::BSTArray as BSTArray_;
    /// # type BSTArray<T> = BSTArray_<T, stdx::alloc::Global>;
    ///
    /// let mut array = BSTArray::<i32>::with_capacity(10);
    /// array.push(1);
    /// array.push(2);
    /// array.push(3);
    /// array.retain(|&x| x > 1);
    /// assert_eq!(array.len(), 2);
    /// assert!(array.contains(&2));
    /// assert!(array.contains(&3));
    /// ```
    ///
    /// # Panics
    /// array ptr is null
    #[inline]
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut retained = 0;

        for i in 0..self.len() {
            let src = match self.as_non_null_ptr() {
                Some(elem) => unsafe { elem.add(i) },
                None => continue,
            };

            if f(unsafe { src.as_ref() }) {
                if retained != i {
                    unsafe {
                        let dst = self.as_non_null_ptr().unwrap().add(retained).as_ptr();
                        ptr::copy_nonoverlapping(src.as_ptr(), dst, 1);
                    }
                }
                retained += 1;
            } else {
                unsafe { ptr::drop_in_place(src.as_ptr()) }; // Drop elements that do not match the predicate
            }
        }

        self.size = retained as u32;
    }

    /// Resizes the array to the specified length.
    ///
    /// If the array is resized to a larger length, the new elements will be initialized
    /// using the default constructor for `T`. If the array is resized to a smaller length,
    /// elements at the end will be dropped.
    ///
    /// # Examples
    ///
    /// ```
    /// # use commonlibsse_ng::re::BSTArray::BSTArray as BSTArray_;
    /// # type BSTArray<T> = BSTArray_<T, stdx::alloc::Global>;
    ///
    /// let mut array = BSTArray::<i32>::with_capacity(10);
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
                if let Some(src) = self.as_non_null_ptr() {
                    unsafe { ptr::drop_in_place(src.add(i).as_ptr()) };
                }
            }
        }
        self.size = new_size as u32;
    }

    /// Removes a range of elements from the array, returning them as a vector.
    ///
    /// This method removes the elements within the specified range and returns them as
    /// a `Vec<T>`. The range must be within the bounds of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// # use commonlibsse_ng::re::BSTArray::BSTArray as BSTArray_;
    /// # type BSTArray<T> = BSTArray_<T, stdx::alloc::Global>;
    ///
    /// let mut array = BSTArray::<i32>::with_capacity(10);
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
        self.size = start as u32;

        let iter = match self.as_non_null_ptr() {
            Some(src) => unsafe {
                core::slice::from_raw_parts(src.add(start).as_ptr(), end - start)
            },
            None => &[],
        }
        .iter();

        BSTDrain {
            iter,
            tail_start: end,
            tail_len: len - end,
            array: unsafe { NonNull::new_unchecked(self as *mut Self) },
        }
    }

    /// Returns a slice of all elements in the array.
    #[inline]
    pub const fn as_slice(&self) -> &[T] {
        let ptr = self.as_non_null_ptr();
        let len = self.len();

        if ptr.is_none() || (len == 0) {
            return &[];
        }

        if let Some(src) = self.as_non_null_ptr() {
            unsafe { core::slice::from_raw_parts(src.as_ptr(), len) }
        } else {
            &[]
        }
    }

    /// Returns a mutable slice of all elements in the array.
    #[inline]
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        let ptr = self.as_non_null_ptr();
        let len = self.len();

        if ptr.is_none() || (len == 0) {
            return &mut [];
        }

        if let Some(src) = self.as_non_null_ptr() {
            unsafe { core::slice::from_raw_parts_mut(src.as_ptr(), len) }
        } else {
            &mut []
        }
    }

    /// Returns an iterator over the elements of the array.
    ///
    /// This iterator yields references to the elements in the array.
    ///
    /// # Examples
    ///
    /// ```
    /// # use commonlibsse_ng::re::BSTArray::BSTArray as BSTArray_;
    /// # type BSTArray<T> = BSTArray_<T, stdx::alloc::Global>;
    ///
    /// let mut array = BSTArray::<i32>::with_capacity(10);
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
        if new_capacity == 0 {
            return;
        }
        let new_data = {
            let layout = Self::new_layout(new_capacity);
            A::allocate(layout).map_or_else(|_| handle_alloc_error(layout), |data| data.cast::<T>())
        };

        if let Some(old_data) = self.as_non_null_ptr() {
            let old_capacity = self.capacity();
            let copy_count = core::cmp::min(old_capacity, new_capacity);

            unsafe { ptr::copy_nonoverlapping(old_data.as_ptr(), new_data.as_ptr(), copy_count) };

            let old_layout = self.layout();
            unsafe { A::deallocate(old_data.cast(), old_layout) };
        }

        self.data = Some(Unique::from(new_data));
        self.capacity = new_capacity as u32;
    }

    /// Creates a layout describing the record for a `[T; n]`.
    ///
    /// # Panics
    /// On arithmetic overflow or when the total size would exceed
    /// `isize::MAX`, panic.
    fn new_layout(n: usize) -> Layout {
        Layout::array::<T>(n).expect("BSTArray need: alloc size < isize::MAX")
    }

    /// Gets a layout self.
    ///
    /// # Panics
    /// On arithmetic overflow or when the total size would exceed
    /// `isize::MAX`, panic.
    fn layout(&self) -> Layout {
        Self::new_layout(self.capacity())
    }
}

impl<T, A> Index<usize> for BSTArray<T, A>
where
    A: SelflessAllocator,
{
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len(), "Index out of bounds");
        unsafe { self.as_non_null_ptr().unwrap().add(index).as_ref() }
    }
}

impl<T, A> IndexMut<usize> for BSTArray<T, A>
where
    A: SelflessAllocator,
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.len(), "Index out of bounds");
        unsafe { self.as_non_null_ptr().unwrap().add(index).as_mut() }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Iterator

/// Iterator returned by `BSTArray::drain()`
pub struct BSTDrain<'a, T, A>
where
    A: SelflessAllocator,
{
    tail_start: usize, // = range.end
    tail_len: usize,   // = original_len - range.end
    iter: core::slice::Iter<'a, T>,
    array: NonNull<BSTArray<T, A>>,
}

impl<T, A> Iterator for BSTDrain<'_, T, A>
where
    A: SelflessAllocator,
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
    A: SelflessAllocator,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|item| unsafe { ptr::read(item) })
    }
}

impl<T, A> ExactSizeIterator for BSTDrain<'_, T, A>
where
    A: SelflessAllocator,
{
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T, A: SelflessAllocator> Drop for BSTDrain<'_, T, A> {
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
                if let Some(ptr) = array.as_non_null_ptr() {
                    if tail != start {
                        let src = ptr.add(tail).as_ptr();
                        let dst = ptr.add(start).as_ptr();
                        ptr::copy(src, dst, self.tail_len);
                    }
                }
                array.size = (start + self.tail_len) as u32;
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct BSTArrayIterator<'a, T, A>
where
    A: SelflessAllocator,
{
    array: &'a BSTArray<T, A>,
    index: usize,
}

impl<'a, T, A> Iterator for BSTArrayIterator<'a, T, A>
where
    A: SelflessAllocator,
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.len() {
            let item = unsafe { self.array.as_non_null_ptr()?.add(self.index).as_ref() };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

pub struct BSTArrayIntoIterator<T, A>
where
    A: SelflessAllocator,
{
    array: BSTArray<T, A>,
    index: usize,
}

impl<T, A> Iterator for BSTArrayIntoIterator<T, A>
where
    A: SelflessAllocator,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.len() {
            let item = unsafe { ptr::read(self.array.as_non_null_ptr()?.add(self.index).as_ptr()) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<T, A> IntoIterator for BSTArray<T, A>
where
    A: SelflessAllocator,
{
    type Item = T;
    type IntoIter = BSTArrayIntoIterator<T, A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        BSTArrayIntoIterator { array: self, index: 0 }
    }
}

impl<'a, T, A> IntoIterator for &'a BSTArray<T, A>
where
    A: SelflessAllocator,
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
    A: SelflessAllocator,
{
    array: &'a mut BSTArray<T, A>,
    index: usize,
}

impl<'a, T, A> Iterator for BSTArrayIterMut<'a, T, A>
where
    A: SelflessAllocator,
{
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.len() {
            unsafe {
                let mut ptr = self.array.as_non_null_ptr()?.add(self.index);
                self.index += 1;
                Some(ptr.as_mut())
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

impl<'a, T, A> IntoIterator for &'a mut BSTArray<T, A>
where
    A: SelflessAllocator,
{
    type Item = &'a mut T;
    type IntoIter = BSTArrayIterMut<'a, T, A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        BSTArrayIterMut { array: self, index: 0 }
    }
}

impl<T, A> Extend<T> for BSTArray<T, A>
where
    A: SelflessAllocator,
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

impl<T, A> core::fmt::Debug for BSTArray<T, A>
where
    T: core::fmt::Debug,
    A: SelflessAllocator,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.as_slice()).finish()
    }
}

impl<T> Default for BSTArray<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T, A> Clone for BSTArray<T, A>
where
    A: SelflessAllocator + Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        // heap clone
        let data = self.as_non_null_ptr().map(|src_ptr| {
            let layout = self.layout();
            let Ok(dst_ptr) = A::allocate(layout) else { handle_alloc_error(layout) };
            let dst_ptr = dst_ptr.cast::<T>();
            unsafe { ptr::copy_nonoverlapping(src_ptr.as_ptr(), dst_ptr.as_ptr(), layout.size()) };

            Unique::from(dst_ptr)
        });

        Self { data, capacity: self.capacity, pad08: 0, size: self.size, alloc: PhantomData }
    }
}

impl<T, A> PartialEq for BSTArray<T, A>
where
    T: PartialEq,
    A: SelflessAllocator,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}
impl<T, A> PartialEq<Vec<T>> for BSTArray<T, A>
where
    T: PartialEq,
    A: SelflessAllocator,
{
    #[inline]
    fn eq(&self, other: &Vec<T>) -> bool {
        self.as_slice() == *other
    }
}

impl<T, A> PartialEq<&[T]> for BSTArray<T, A>
where
    T: PartialEq,
    A: SelflessAllocator,
{
    #[inline]
    fn eq(&self, other: &&[T]) -> bool {
        self.as_slice() == *other
    }
}

impl<T, A> Eq for BSTArray<T, A>
where
    T: Eq,
    A: SelflessAllocator,
{
}

impl<T, A> PartialOrd for BSTArray<T, A>
where
    T: PartialOrd,
    A: SelflessAllocator,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl<T, A> Ord for BSTArray<T, A>
where
    T: Ord,
    A: SelflessAllocator,
{
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<T, A> core::hash::Hash for BSTArray<T, A>
where
    T: core::hash::Hash,
    A: SelflessAllocator,
{
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use stdx::alloc::Global;

    use super::BSTArray as BSTArray_;

    type BSTArray<T> = BSTArray_<T, Global>;

    #[test]
    fn test_drain() {
        let mut array = BSTArray::with_capacity(10);
        array.push(1);
        array.push(2);
        array.push(3);
        array.push(4);
        array.push(5);

        let drained = array.drain(1..3);
        // for i in drained {
        //     println!("{:?}", i);
        // }
        assert_eq!(drained.collect::<Vec<_>>(), vec![2, 3]);
        assert_eq!(array.len(), 3);
        assert_eq!(array[0], 1);
        assert_eq!(array[1], 4);
        assert_eq!(array[2], 5);
    }
}

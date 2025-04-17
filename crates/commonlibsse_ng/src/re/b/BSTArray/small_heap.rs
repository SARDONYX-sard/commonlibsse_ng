// Copyright (c) 2018 The Servo Project Developers
// SPDX-License-Identifier: Apache-2.0 OR MIT
// https://github.com/servo/rust-smallvec/blob/v2/src/lib.rs#L3

use core::{
    alloc::Layout,
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Index, IndexMut, Range, RangeBounds},
    ptr::{self, NonNull},
};
use std::alloc::handle_alloc_error;

use stdx::unique::Unique;

use crate::re::MemoryManager::{TESGlobalAlloc, selfless_alloc::allocator::SelflessAllocator};

///Use stack while within the specified size, and use heap if it is larger.
///
///This is the same purpose as `smallvec` crate and other optimizations, except that the memory layout is for TES.
///
///It is effective when most of the memory can be fit in the stack, except for some exceptions, but it slows down the process if there are frequent fallbacks to the heap.
///
/// - [`smallvec` crate](https://crates.io/crates/smallvec)
///
/// - `N`: element length (not bytes size)
#[repr(C)]
pub struct BSTSmallArray<T, const N: usize = 1, A = TESGlobalAlloc>
where
    A: SelflessAllocator,
{
    // BSTSmallArrayHeapAllocator
    capacity: u32, // 0x00,

    /// Indicates whether the data is stored locally (on the stack).
    storage_type: StorageType_CEnum, // 0x04,

    // The union of local stack data and heap pointer.
    data: RawBSTSmallArray<T, N>, // 0x08

    // BSTArrayBase
    // length of elements
    size: u32, // 0x10

    // Assumed Zero size type.
    alloc: PhantomData<A>,
}
const _: () = assert!(core::mem::size_of::<BSTSmallArray<(), 10>>() == 0x18);

/// Indicates how the data in `BSTSmallArray` is stored.
///
/// - `Heap`: Data is allocated on the heap.
/// - `Inline`: Data is stored inline (e.g., on the stack or within the object).
#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum StorageType {
    /// Heap-allocated storage.
    Heap = 0,
    /// Inline storage (e.g., stack or inline buffer).
    Inline = 1,
}

/// A union that stores either a heap pointer or a fixed-size array for local storage.
#[repr(C)]
union RawBSTSmallArray<T, const N: usize> {
    /// Pointer to heap memory. Same as `*mut T` memory layout
    heap: Option<Unique<T>>,
    /// Fixed-size array for local (stack) storage.
    inline: ManuallyDrop<MaybeUninit<[T; N]>>,
}

impl<T, const N: usize> RawBSTSmallArray<T, N> {
    #[inline]
    const fn new() -> Self {
        Self::new_inline(MaybeUninit::uninit())
    }
    #[inline]
    const fn new_inline(inline: MaybeUninit<[T; N]>) -> Self {
        Self { inline: ManuallyDrop::new(inline) }
    }
    #[inline]
    const fn new_heap(ptr: NonNull<T>) -> Self {
        Self { heap: Some(unsafe { Unique::new_unchecked(ptr.as_ptr()) }) }
    }
}

const _: () = {
    const SIZE: usize = core::mem::size_of::<RawBSTSmallArray<u32, 4>>();
    assert!(SIZE == 0x10);
};

impl<T, const N: usize, A> BSTSmallArray<T, N, A>
where
    A: SelflessAllocator,
{
    /// Creates a new, empty `BSTArray<T, N, A>` with the specified allocator.
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
        Self {
            data: RawBSTSmallArray::new(),
            capacity: N as u32,
            storage_type: StorageType_CEnum::from_enum(StorageType::Inline),
            size: 0,
            alloc: PhantomData,
        }
    }

    /// Creates a new, empty `BSTArray<T, N, A>` with the capacity.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::BSTArray::BSTArray as BSTArray_;
    /// # type BSTArray<T> = BSTArray_<T, stdx::alloc::Global>;
    ///
    /// let array = BSTArray::<i32>::with_capacity(5);
    /// assert_eq!(array.capacity(), 5);
    /// ```
    pub fn with_capacity(capacity: u32) -> Self {
        let data = if capacity > (N as u32) {
            let layout = Self::new_layout(capacity);
            let Ok(heap_ptr) = A::allocate(layout) else { handle_alloc_error(layout) };
            RawBSTSmallArray::new_heap(heap_ptr.cast())
        } else {
            RawBSTSmallArray::new() // stack
        };

        Self {
            data,
            capacity,
            storage_type: StorageType_CEnum::from_enum(StorageType::Inline),
            size: 0,
            alloc: PhantomData,
        }
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
        self.change_capacity(self.size);
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
    #[allow(clippy::missing_const_for_fn)] // Wrong lint. cannot use const when use deref
    #[allow(clippy::as_ptr_cast_mut)]
    pub fn as_non_null_ptr(&self) -> Option<NonNull<T>> {
        match self.storage_type() {
            StorageType::Heap => unsafe {
                self.data.heap.map(|ptr| NonNull::new_unchecked(ptr.as_ptr()))
            },
            StorageType::Inline => {
                // `&MaybeUninit<[T; N]>` → `*mut T`
                let ptr = unsafe { (*self.data.inline).as_ptr() as *mut T };
                Some(unsafe { NonNull::new_unchecked(ptr) })
            }
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
    pub fn drain<R>(&mut self, range: R) -> BSTDrain<'_, T, N, A>
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
    pub fn as_slice(&self) -> &[T] {
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
    pub fn as_mut_slice(&mut self) -> &mut [T] {
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
    pub const fn iter(&self) -> BSTArrayIterator<'_, T, N, A> {
        BSTArrayIterator { array: self, index: 0 }
    }

    fn grow(&mut self) {
        const MIN_CAPACITY: u32 = 4;
        const GROWTH_FACTOR: u32 = 2;

        let old_capacity = self.capacity;
        let new_capacity =
            if old_capacity == 0 { MIN_CAPACITY } else { old_capacity * GROWTH_FACTOR };
        self.change_capacity(new_capacity);
    }

    fn change_capacity(&mut self, new_capacity: u32) {
        if new_capacity == 0 {
            return;
        }

        let storage_type = self.storage_type();

        if new_capacity <= (N as u32) && storage_type == StorageType::Inline {
            return;
        }

        let copy_count = core::cmp::min(self.size, new_capacity) as usize;

        let new_layout = Self::new_layout(new_capacity);
        let new_data = A::allocate(new_layout)
            .map_or_else(|_| handle_alloc_error(new_layout), |data| data.cast::<T>());

        unsafe {
            let dst = new_data.as_ptr();

            match storage_type {
                StorageType::Inline => {
                    // Inline -> heap
                    let src = self.data.inline.as_ptr().cast::<T>();
                    ptr::copy_nonoverlapping(src, dst, copy_count);
                }
                StorageType::Heap => {
                    if let Some(old_ptr) = self.data.heap {
                        ptr::copy_nonoverlapping(old_ptr.as_ptr(), dst, copy_count);
                        A::deallocate(old_ptr.as_non_null_ptr().cast::<u8>(), self.layout());
                    }
                }
            }
        }

        self.data = RawBSTSmallArray::new_heap(new_data);
        self.storage_type = StorageType::Heap.into();
        self.capacity = new_capacity;
    }

    /// Creates a layout describing the record for a `[T; n]`.
    ///
    /// # Panics
    /// On arithmetic overflow or when the total size would exceed
    /// `isize::MAX`, panic.
    fn new_layout(n: u32) -> Layout {
        Layout::array::<T>(n as usize).expect("BSTArray need: alloc size < isize::MAX")
    }

    /// Gets a current layout self.
    ///
    /// # Panics
    /// On arithmetic overflow or when the total size would exceed
    /// `isize::MAX`, panic.
    fn layout(&self) -> Layout {
        Self::new_layout(self.capacity)
    }

    const fn storage_type(&self) -> StorageType {
        match self.storage_type.to_enum() {
            Some(value) => value,
            None => StorageType::Inline,
        }
    }
}

impl<T, const N: usize, A> Index<usize> for BSTSmallArray<T, N, A>
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

impl<T, const N: usize, A> IndexMut<usize> for BSTSmallArray<T, N, A>
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
pub struct BSTDrain<'a, T, const N: usize, A>
where
    A: SelflessAllocator,
{
    tail_start: usize, // = range.end
    tail_len: usize,   // = original_len - range.end
    iter: core::slice::Iter<'a, T>,
    array: NonNull<BSTSmallArray<T, N, A>>,
}

impl<T, const N: usize, A> Iterator for BSTDrain<'_, T, N, A>
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

impl<T, const N: usize, A> DoubleEndedIterator for BSTDrain<'_, T, N, A>
where
    A: SelflessAllocator,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|item| unsafe { ptr::read(item) })
    }
}

impl<T, const N: usize, A> ExactSizeIterator for BSTDrain<'_, T, N, A>
where
    A: SelflessAllocator,
{
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T, const N: usize, A: SelflessAllocator> Drop for BSTDrain<'_, T, N, A> {
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

pub struct BSTArrayIterator<'a, T, const N: usize, A>
where
    A: SelflessAllocator,
{
    array: &'a BSTSmallArray<T, N, A>,
    index: usize,
}

impl<'a, T, const N: usize, A> Iterator for BSTArrayIterator<'a, T, N, A>
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

pub struct BSTArrayIntoIterator<T, const N: usize, A>
where
    A: SelflessAllocator,
{
    array: BSTSmallArray<T, N, A>,
    index: usize,
}

impl<T, const N: usize, A> Iterator for BSTArrayIntoIterator<T, N, A>
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

impl<T, const N: usize, A> IntoIterator for BSTSmallArray<T, N, A>
where
    A: SelflessAllocator,
{
    type Item = T;
    type IntoIter = BSTArrayIntoIterator<T, N, A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        BSTArrayIntoIterator { array: self, index: 0 }
    }
}

impl<'a, T, const N: usize, A> IntoIterator for &'a BSTSmallArray<T, N, A>
where
    A: SelflessAllocator,
{
    type Item = &'a T;
    type IntoIter = BSTArrayIterator<'a, T, N, A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        BSTArrayIterator { array: self, index: 0 }
    }
}

pub struct BSTArrayIterMut<'a, T, const N: usize, A>
where
    A: SelflessAllocator,
{
    array: &'a mut BSTSmallArray<T, N, A>,
    index: usize,
}

impl<'a, T, const N: usize, A> Iterator for BSTArrayIterMut<'a, T, N, A>
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

impl<'a, T, const N: usize, A> IntoIterator for &'a mut BSTSmallArray<T, N, A>
where
    A: SelflessAllocator,
{
    type Item = &'a mut T;
    type IntoIter = BSTArrayIterMut<'a, T, N, A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        BSTArrayIterMut { array: self, index: 0 }
    }
}

impl<T, const N: usize, A> Extend<T> for BSTSmallArray<T, N, A>
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

impl<T, const N: usize, A> core::fmt::Debug for BSTSmallArray<T, N, A>
where
    T: core::fmt::Debug,
    A: SelflessAllocator,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.as_slice()).finish()
    }
}

impl<T, const N: usize> Default for BSTSmallArray<T, N> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone, const N: usize, A> Clone for BSTSmallArray<T, N, A>
where
    A: SelflessAllocator + Clone,
{
    fn clone(&self) -> Self {
        let mut new = if self.capacity as usize > N {
            // allocate heap
            let layout = Self::new_layout(self.capacity);
            let new_ptr = A::allocate(layout).map_or_else(
                |_| handle_alloc_error(layout),
                |p| unsafe { NonNull::new_unchecked(p.cast::<T>().as_ptr()) },
            );

            Self {
                data: RawBSTSmallArray::new_heap(new_ptr),
                capacity: self.capacity,
                storage_type: StorageType::Heap.into(),
                size: self.size,
                alloc: PhantomData,
            }
        } else {
            Self {
                data: RawBSTSmallArray::new(),
                capacity: self.capacity,
                storage_type: StorageType::Inline.into(),
                size: self.size,
                alloc: PhantomData,
            }
        };

        // Clone elements
        let count = self.size as usize;
        for i in 0..count {
            let src = &self[i];
            new.push(src.clone());
        }

        new
    }
}

impl<T, const N: usize, A> PartialEq for BSTSmallArray<T, N, A>
where
    T: PartialEq,
    A: SelflessAllocator,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}
impl<T, const N: usize, A> PartialEq<Vec<T>> for BSTSmallArray<T, N, A>
where
    T: PartialEq,
    A: SelflessAllocator,
{
    #[inline]
    fn eq(&self, other: &Vec<T>) -> bool {
        self.as_slice() == *other
    }
}

impl<T, const N: usize, A> PartialEq<&[T]> for BSTSmallArray<T, N, A>
where
    T: PartialEq,
    A: SelflessAllocator,
{
    #[inline]
    fn eq(&self, other: &&[T]) -> bool {
        self.as_slice() == *other
    }
}

impl<T, const N: usize, A> Eq for BSTSmallArray<T, N, A>
where
    T: Eq,
    A: SelflessAllocator,
{
}

impl<T, const N: usize, A> PartialOrd for BSTSmallArray<T, N, A>
where
    T: PartialOrd,
    A: SelflessAllocator,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl<T, const N: usize, A> Ord for BSTSmallArray<T, N, A>
where
    T: Ord,
    A: SelflessAllocator,
{
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<T, const N: usize, A> core::hash::Hash for BSTSmallArray<T, N, A>
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

    use super::BSTSmallArray as BSTSmallArray_;

    type BSTSmallArray<T, const N: usize = 10, A = Global> = BSTSmallArray_<T, N, A>;

    #[test]
    fn test_drain() {
        let mut array = BSTSmallArray::<_>::with_capacity(10);
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

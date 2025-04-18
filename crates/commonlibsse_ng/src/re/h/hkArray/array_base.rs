use core::{
    alloc::Layout,
    marker::PhantomData,
    ptr::{self, NonNull},
};
use std::alloc::handle_alloc_error;

use crate::re::MemoryManager::{TESGlobalAlloc, selfless_alloc::allocator::SelflessAllocator};

/// Represents a base class for a dynamic array of type `T`.
#[repr(C)]
pub struct hkArrayBase<T, A: SelflessAllocator = TESGlobalAlloc> {
    pub(super) data: Option<NonNull<T>>,
    size: i32,
    capacityAndFlags: i32,
    allocator: PhantomData<A>,
}
const _: () = assert!(core::mem::size_of::<hkArrayBase<*mut ()>>() == 0x10);

unsafe impl<T: Send, A> Send for hkArrayBase<T, A> where A: SelflessAllocator {}
unsafe impl<T: Sync, A> Sync for hkArrayBase<T, A> where A: SelflessAllocator {}

impl<T, A> Default for hkArrayBase<T, A>
where
    A: SelflessAllocator,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T, A> hkArrayBase<T, A>
where
    A: SelflessAllocator,
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
        // .expect("hkArrayBase has expected new_capacity < isize::MAX");
        let new_layout = Self::new_layout(new_capacity);

        let Ok(new_mem) = A::allocate_zeroed(new_layout) else { handle_alloc_error(new_layout) };
        let new_mem = new_mem.cast();

        if let Some(data) = self.data {
            if copy_len > 0 {
                unsafe {
                    ptr::copy_nonoverlapping(data.as_ptr(), new_mem.as_ptr(), copy_len as usize);
                }
            }
            if self.need_dealloc() {
                let old_layout = self.layout();
                unsafe { A::deallocate(data.cast(), old_layout) };
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
    pub const fn len(&self) -> usize {
        self.size as usize
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

    #[inline]
    pub fn with_capacity(capacity: i32) -> Self {
        let mut ret = Self::new();
        ret.reserve(capacity);
        ret
    }

    /// Reserves memory for the array to hold new capacity elements.
    ///
    /// # Panics
    #[inline]
    pub fn reserve(&mut self, new_capacity: i32) {
        assert!((new_capacity) <= Self::CAPACITY_MASK);
        if new_capacity <= self.capacity() {
            return;
        }

        let new_mem = self.allocate_new_memory(new_capacity, self.size);
        self.data = Some(new_mem);
        self.clear_capacity_bits();
        self.capacityAndFlags |= new_capacity & Self::CAPACITY_MASK;
    }

    /// Pushes a new element to the end of the array.
    ///
    /// - Equivalent C++ method: `push_back`
    #[inline]
    pub fn push(&mut self, value: T) {
        if self.size == self.capacity() {
            let new_capacity = if self.is_empty() {
                1
            } else {
                (self.len() as f32 * Self::GROWTH_FACTOR).ceil() as i32
            };
            self.reserve(new_capacity);
        }

        unsafe {
            let end = self.as_ptr().add(self.len());
            ptr::write(end, value);
        }
        self.size += 1;
    }

    /// Removes the last element from the array and returns it, or `None` if it's empty.
    #[inline]
    pub const fn pop(&mut self) -> Option<T> {
        let len = self.len();
        if len == 0 {
            None
        } else {
            self.size -= 1;
            unsafe { Some(ptr::read(self.as_ptr().add(len - 1))) }
        }
    }

    /// Returns a reference to the element at the given index, if it exists.
    #[inline]
    pub const fn get(&self, index: usize) -> Option<&T> {
        if index < self.len() {
            return unsafe { self.as_ptr().add(index).as_ref() };
        }
        None
    }

    /// Returns a mutable reference to the element at the given index, if it exists.
    #[inline]
    pub const fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len() {
            return unsafe { self.as_ptr().add(index).as_mut() };
        }
        None
    }

    /// Clears the array, removing all elements but preserving the capacity.
    #[inline]
    pub fn clear(&mut self) {
        // Drop all elements in the array without changing capacity
        for elem in self.as_mut_slice() {
            unsafe {
                ptr::drop_in_place(elem); // SAFETY: we're dropping each element in place
            }
        }

        self.size = 0; // Reset the length, but keep the allocated capacity
    }

    #[inline]
    pub const fn iter(&self) -> hkArrayRefIterator<'_, T, A> {
        hkArrayRefIterator { array: self, index: 0 }
    }

    #[inline]
    pub const fn as_slice(&self) -> &[T] {
        match self.data {
            Some(data) => unsafe { core::slice::from_raw_parts(data.as_ptr(), self.len()) },
            None => &[],
        }
    }

    #[inline]
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        match self.data {
            Some(data) => unsafe { core::slice::from_raw_parts_mut(data.as_ptr(), self.len()) },
            None => &mut [],
        }
    }

    /// Checks if the array contains the given element.
    ///
    /// Returns `true` if the element is present in the array, and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use commonlibsse_ng::re::hkArray::{hkArrayBase, RustAllocator};
    ///
    /// let mut array = hkArrayBase::<i32, RustAllocator>::with_capacity(10);
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
    /// use commonlibsse_ng::re::hkArray::{hkArrayBase, RustAllocator};
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
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut retained = 0;

        for i in 0..(self.len()) {
            let elem = match unsafe { self.as_ptr().add(i).as_ref() } {
                Some(elem) => elem,
                None => continue,
            };

            if f(elem) {
                if retained != i {
                    unsafe {
                        let src = self.as_ptr().add(i);
                        let dst = self.as_ptr().add(retained);
                        ptr::copy_nonoverlapping(src, dst, 1);
                    }
                }
                retained += 1;
            } else {
                // Drop elements that do not match the predicate
                unsafe { ptr::drop_in_place(self.as_ptr().add(i)) };
            }
        }

        self.size = retained as i32;
    }

    /// Removes a range of elements from the array, returning them as a vector.
    ///
    /// This method removes the elements within the specified range and returns them as
    /// a `Vec<T>`. The range must be within the bounds of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use commonlibsse_ng::re::hkArray::{hkArrayBase, RustAllocator};
    ///
    /// let mut array = hkArrayBase::<i32, RustAllocator>::with_capacity(10);
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
        let len = self.len();
        let core::ops::Range { start, end } = stdx::slice::range(range, ..len);
        debug_assert!(start <= end);
        debug_assert!(end <= len);

        // Need this.
        // If the size is not changed before creating iter for Drain, inconsistencies will occur.
        self.size = start as i32;

        hkArrayDrain {
            iter: unsafe { core::slice::from_raw_parts(self.as_ptr().add(start), end - start) }
                .iter(),
            tail_start: end,
            tail_len: len - end,
            array: unsafe { NonNull::new_unchecked(self as *mut Self) },
        }
    }

    /// Creates a layout describing the record for a `[T; n]`.
    ///
    /// # Panics
    /// On arithmetic overflow or when the total size would exceed
    /// `isize::MAX`, panic.
    fn new_layout(n: i32) -> Layout {
        Layout::array::<T>(n as usize).expect("hkArrayBase need: alloc size < isize::MAX")
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

impl<T, A> Drop for hkArrayBase<T, A>
where
    A: SelflessAllocator,
{
    #[inline]
    fn drop(&mut self) {
        self.clear();
        if let Some(data) = self.data {
            unsafe { A::deallocate(data.cast(), self.layout()) }
        }
    }
}

impl<T, A> hkArrayBase<T, A>
where
    T: Clone,
    A: SelflessAllocator,
{
    /// Resizes the array to hold `a_count` elements.
    ///
    /// # Panics
    #[inline]
    pub fn resize(&mut self, new_len: i32, value: T) {
        assert!((0..=Self::CAPACITY_MASK).contains(&new_len));
        if new_len == self.size {
            return;
        }

        if new_len < self.size {
            for i in new_len..self.size {
                unsafe {
                    let elem = self.as_ptr().add(i as usize);
                    ptr::drop_in_place(elem);
                }
            }
        }

        let new_mem = self.allocate_new_memory(new_len, self.size.min(new_len));

        if new_len > self.size {
            for i in self.len()..(new_len as usize) {
                unsafe {
                    let elem = new_mem.add(i);
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

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Iterator returned by `hkArray::drain()`
pub struct hkArrayDrain<'a, T, A>
where
    A: SelflessAllocator,
{
    tail_start: usize, // = range.end
    tail_len: usize,   // = original_len - range.end
    iter: core::slice::Iter<'a, T>,
    array: NonNull<hkArrayBase<T, A>>,
}

impl<T, A> Iterator for hkArrayDrain<'_, T, A>
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

impl<T, A> DoubleEndedIterator for hkArrayDrain<'_, T, A>
where
    A: SelflessAllocator,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|item| unsafe { ptr::read(item) })
    }
}

impl<T, A> ExactSizeIterator for hkArrayDrain<'_, T, A>
where
    A: SelflessAllocator,
{
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T, A: SelflessAllocator> Drop for hkArrayDrain<'_, T, A> {
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
                    let ptr = array.as_ptr();
                    let src = ptr.add(tail);
                    let dst = ptr.add(start);
                    ptr::copy(src, dst, self.tail_len);
                }
                array.size = (start + self.tail_len) as i32;
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Iterator

pub struct hkArrayIterator<T, A>
where
    A: SelflessAllocator,
{
    array: hkArrayBase<T, A>,
    index: usize,
}

impl<T, A> Iterator for hkArrayIterator<T, A>
where
    A: SelflessAllocator,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.len() {
            let item = unsafe { self.array.data?.add(self.index).read() };
            self.index += 1;
            Some(item)
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

pub struct hkArrayRefIterator<'a, T, A>
where
    A: SelflessAllocator,
{
    array: &'a hkArrayBase<T, A>,
    index: usize,
}

impl<'a, T, A> Iterator for hkArrayRefIterator<'a, T, A>
where
    A: SelflessAllocator,
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.len() {
            let item = unsafe { self.array.data?.add(self.index).as_ref() };
            self.index += 1;
            Some(item)
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

pub struct hkArrayIterMut<'a, T, A>
where
    A: SelflessAllocator,
{
    array: &'a mut hkArrayBase<T, A>,
    index: usize,
}

impl<'a, T, A> Iterator for hkArrayIterMut<'a, T, A>
where
    A: SelflessAllocator,
{
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.len() {
            unsafe {
                let ptr = self.array.as_ptr().add(self.index);
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

impl<T, A> IntoIterator for hkArrayBase<T, A>
where
    A: SelflessAllocator,
{
    type Item = T;
    type IntoIter = hkArrayIterator<T, A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        hkArrayIterator { array: self, index: 0 }
    }
}

impl<'a, T, A> IntoIterator for &'a hkArrayBase<T, A>
where
    A: SelflessAllocator,
{
    type Item = &'a T;
    type IntoIter = hkArrayRefIterator<'a, T, A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        hkArrayRefIterator { array: self, index: 0 }
    }
}

impl<'a, T, A> IntoIterator for &'a mut hkArrayBase<T, A>
where
    A: SelflessAllocator,
{
    type Item = &'a mut T;
    type IntoIter = hkArrayIterMut<'a, T, A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        hkArrayIterMut { array: self, index: 0 }
    }
}

impl<T, A> Extend<T> for hkArrayBase<T, A>
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
// Implement standard trait

impl<T: core::fmt::Debug, A: SelflessAllocator> core::fmt::Debug for hkArrayBase<T, A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("hkArrayBase")
            .field("data", &self.iter().collect::<Vec<_>>())
            .field("size", &self.size)
            .field("capacityAndFlags", &self.capacityAndFlags)
            .finish()
    }
}

impl<T: Clone, A: SelflessAllocator> Clone for hkArrayBase<T, A> {
    fn clone(&self) -> Self {
        let mut new = Self::new();
        new.reserve(self.capacity());
        for i in 0..self.len() {
            unsafe {
                let ptr = self.as_ptr().add(i);
                new.push((*ptr).clone());
            }
        }
        new
    }
}

impl<T: PartialEq, A: SelflessAllocator> PartialEq for hkArrayBase<T, A> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for i in 0..self.len() {
            unsafe {
                let a = self.as_ptr().add(i);
                let b = other.as_ptr().add(i);
                if *a != *b {
                    return false;
                }
            }
        }
        true
    }
}

impl<T: Eq, A: SelflessAllocator> Eq for hkArrayBase<T, A> {}

impl<T: PartialOrd, A: SelflessAllocator> PartialOrd for hkArrayBase<T, A> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        for i in 0..self.len().min(other.len()) {
            unsafe {
                let a = self.as_ptr().add(i);
                let b = other.as_ptr().add(i);
                match (*a).partial_cmp(&*b) {
                    Some(core::cmp::Ordering::Equal) => {}
                    non_eq => return non_eq,
                }
            }
        }
        self.len().partial_cmp(&other.len())
    }
}

impl<T: Ord, A: SelflessAllocator> Ord for hkArrayBase<T, A> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        for i in 0..self.len().min(other.len()) {
            unsafe {
                let a = self.as_ptr().add(i);
                let b = other.as_ptr().add(i);
                match (*a).cmp(&*b) {
                    core::cmp::Ordering::Equal => {}
                    non_eq => return non_eq,
                }
            }
        }
        self.len().cmp(&other.len())
    }
}

impl<T: core::hash::Hash, A: SelflessAllocator> core::hash::Hash for hkArrayBase<T, A> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        for i in 0..self.len() {
            unsafe {
                let ptr = self.as_ptr().add(i);
                ptr.hash(state);
            }
        }
    }
}

impl<T, A> core::ops::Index<usize> for hkArrayBase<T, A>
where
    A: SelflessAllocator,
{
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < (self.len()), "Index out of bounds");
        unsafe { self.as_ptr().wrapping_add(index).as_ref().expect("Index out of bounds") }
    }
}

impl<T, A> core::ops::IndexMut<usize> for hkArrayBase<T, A>
where
    A: SelflessAllocator,
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < (self.len()), "Index out of bounds");
        unsafe { self.as_ptr().wrapping_add(index).as_mut().expect("Index out of bounds") }
    }
}

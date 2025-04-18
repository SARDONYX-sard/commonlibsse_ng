use core::alloc::Layout;
use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use core::mem::MaybeUninit;
use core::ptr::{self, NonNull};
use core::slice;
use std::alloc::handle_alloc_error;

use crate::re::MemoryManager::TESGlobalAlloc;
use stdx::alloc::Allocator;
use stdx::ptr::const_non_null::ConstNonNull;
use stdx::unique::Unique;

/// Array whose first pointer is only a pointer to the length.
///
/// Memory layout:
/// ```txt
/// ┌────────────┬────────────┬────────────┬──────┬────────────┐
/// │ len: usize │ T[0]       │ T[1]       │ ...  │ T[N - 1]   │
/// └────────────┴────────────┴────────────┴──────┴────────────┘
///                   ↑
///   data: Unique<T> ┘
/// ```
/// # Example
/// ```rust
/// use commonlibsse_ng::re::MemoryManager::SimpleArray::SimpleArray;
/// let array: SimpleArray<i32> = SimpleArray::new();
/// assert_eq!(array.len(), 0);
/// ```
pub struct SimpleArray<T, A: Allocator = TESGlobalAlloc> {
    data: Option<Unique<T>>,
    alloc: A,
}
const _: () = assert!(core::mem::size_of::<SimpleArray<i32>>() == core::mem::size_of::<usize>());

impl<T> SimpleArray<T> {
    /// Creates a new empty `SimpleArray`.
    ///
    /// # Example
    /// ```rust
    /// use commonlibsse_ng::re::MemoryManager::SimpleArray::SimpleArray;
    /// let array: SimpleArray<i32> = SimpleArray::new();
    /// assert_eq!(array.len(), 0);
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self { data: None, alloc: TESGlobalAlloc }
    }

    /// Creates a new `SimpleArray` with the specified capacity.
    ///
    /// Uninitialized memory leads to undefined operation, so 0 is entered.
    ///
    /// # Panics
    /// This function may panic if the allocation fails.
    ///
    /// # Example
    /// ```rust
    /// use commonlibsse_ng::re::MemoryManager::SimpleArray::SimpleArray;
    /// let array: SimpleArray<i32> = SimpleArray::with_capacity(5);
    /// assert_eq!(array.len(), 5);
    /// ```
    #[inline]
    pub fn with_capacity(count: usize) -> Self {
        let mut array = Self::new();
        array.resize(count);
        array
    }
}

impl<T, A> SimpleArray<T, A>
where
    A: Allocator,
{
    /// Creates a new empty `SimpleArray` with Allocator.
    ///
    /// # Example
    /// ```rust
    /// use commonlibsse_ng::re::MemoryManager::SimpleArray::SimpleArray;
    /// use stdx::alloc::Global; // Since TESAllocator is not available for CI, use Rust's.
    ///
    /// let array: SimpleArray<i32, Global> = SimpleArray::new_in(None, Global);
    /// assert_eq!(array.len(), 0);
    /// ```
    #[inline]
    pub const fn new_in(data: Option<Unique<T>>, alloc: A) -> Self {
        Self { data, alloc }
    }

    /// Returns the length of the array (the number of elements currently stored).
    ///
    /// Equivalent C++ `size()`
    ///
    /// # Example
    /// ```rust
    /// use commonlibsse_ng::re::MemoryManager::SimpleArray::SimpleArray;
    /// let array: SimpleArray<i32> = SimpleArray::with_capacity(5);
    /// assert_eq!(array.len(), 5);
    /// ```
    #[inline]
    pub const fn len(&self) -> usize {
        match self.len_ptr() {
            Some(ptr) => unsafe { ptr.read() },
            None => 0,
        }
    }

    /// Checks if the array is empty.
    ///
    /// # Example
    /// ```rust
    /// use commonlibsse_ng::re::MemoryManager::SimpleArray::SimpleArray;
    /// let array: SimpleArray<i32> = SimpleArray::new();
    /// assert!(array.is_empty());
    /// ```
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears the array by deallocating its memory.
    ///
    /// # Safety
    /// This function performs unsafe operations to deallocate the array memory.
    ///
    /// # Example
    /// ```rust
    /// use commonlibsse_ng::re::MemoryManager::SimpleArray::SimpleArray;
    /// let mut array: SimpleArray<i32> = SimpleArray::with_capacity(3);
    /// array.clear();
    /// assert!(array.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        if self.data.is_some() && !self.is_empty() {
            unsafe { ptr::drop_in_place(self.as_mut_slice()) };
            self.set_len(0);
            // reuse data memory. Therefore, no need to deallocate.
        }
    }

    /// Resizes the array to the specified count.
    ///
    /// # Panics
    /// This function may panic if the allocation fails.
    ///
    /// # Example
    /// ```rust
    /// use commonlibsse_ng::re::MemoryManager::SimpleArray::SimpleArray;
    /// let mut array = SimpleArray::<u32>::with_capacity(3);
    /// array.resize(5);
    /// assert_eq!(array.len(), 5);
    /// ```
    pub fn resize(&mut self, count: usize) {
        let old_size = self.len();
        if old_size == count {
            return;
        }

        unsafe {
            let layout = Self::layout(count);
            let Ok(new_head) = self.alloc.allocate(layout) else { handle_alloc_error(layout) };
            let new_head = new_head.cast::<usize>();
            new_head.write(count); // The first pointer is the length.

            let new_data = new_head.add(1).cast::<T>();

            // clone to new array.
            if let Some(prev_data) = self.data {
                let prev_data = prev_data.as_ptr();
                let new_data = new_data.as_ptr();
                if count < old_size {
                    ptr::copy_nonoverlapping(prev_data, new_data, count);
                } else {
                    ptr::copy_nonoverlapping(prev_data, new_data, old_size);
                    for i in old_size..count {
                        ptr::write(new_data.add(i), MaybeUninit::zeroed().assume_init());
                    }
                }
            } else {
                ptr::write_bytes(new_data.as_ptr(), MaybeUninit::zeroed().assume_init(), count);
            }

            self.clear(); // clear prev data array.

            self.data = Some(Unique::from(new_data));
        }
    }

    /// Gets a reference to the element at the given index.
    ///
    /// Return `None` if the index is out of bounds.
    ///
    /// # Example
    /// ```rust
    /// use commonlibsse_ng::re::MemoryManager::SimpleArray::SimpleArray;
    /// let mut array = SimpleArray::with_capacity(3);
    /// array[0] = 1;
    /// assert_eq!(array.get(0), Some(&1));
    /// assert_eq!(array.get(5), None);
    /// ```
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len() { unsafe { Some(self.data()?.add(index).as_ref()) } } else { None }
    }

    /// Gets a mutable reference to the element at the given index.
    ///
    /// Return `None` if the index is out of bounds.
    ///
    /// # Example
    /// ```rust
    /// use commonlibsse_ng::re::MemoryManager::SimpleArray::SimpleArray;
    /// let mut array = SimpleArray::<i32>::with_capacity(3);
    /// if let Some(value) = array.get_mut(1) {
    ///     *value = 42;
    /// };
    /// assert_eq!(array[1], 42);
    /// assert_eq!(array.get_mut(5), None);
    /// ```
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len() { unsafe { Some(self.data()?.add(index).as_mut()) } } else { None }
    }

    /// Returns a raw pointer to the data.
    ///
    /// Equivalent C++ `data()`
    ///
    /// # Example
    /// ```rust
    /// use commonlibsse_ng::re::MemoryManager::SimpleArray::SimpleArray;
    /// let array: SimpleArray<i32> = SimpleArray::new();
    /// let data = array.as_ptr();
    /// ```
    #[inline]
    pub const fn as_ptr(&self) -> *mut T {
        match self.data {
            Some(non_null) => non_null.as_ptr(),
            None => ptr::null_mut(),
        }
    }

    /// Returns a mutable slice of the array.
    ///
    /// # Safety
    /// This function assumes that the array is properly initialized.
    ///
    /// # Example
    /// ```rust
    /// use commonlibsse_ng::re::MemoryManager::SimpleArray::SimpleArray;
    /// let mut array = SimpleArray::with_capacity(4);
    /// let slice = array.as_mut_slice();
    /// slice.copy_from_slice(&[1, 2, 3, 4]);
    /// ```
    #[inline]
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        let len = self.len();
        if len == 0 {
            return &mut [];
        }

        match self.data {
            Some(ptr) => unsafe { slice::from_raw_parts_mut(ptr.as_ptr(), len) },
            None => &mut [],
        }
    }

    /// Returns a slice of the array.
    ///
    /// # Safety
    /// This function assumes that the array is properly initialized.
    ///
    /// # Example
    /// ```rust
    /// use commonlibsse_ng::re::MemoryManager::SimpleArray::SimpleArray;
    /// let array = SimpleArray::<i32>::with_capacity(4);
    /// let slice = array.as_slice();
    /// assert_eq!(slice, &[0, 0, 0, 0]);
    /// ```
    #[inline]
    pub const fn as_slice(&self) -> &[T] {
        let len = self.len();
        if len == 0 {
            return &[];
        }

        match self.data {
            Some(ptr) => unsafe { slice::from_raw_parts(ptr.as_ptr(), len) },
            None => &[],
        }
    }

    /// Return len storage ptr.
    #[inline]
    const fn len_ptr(&self) -> Option<ConstNonNull<usize>> {
        match self.data {
            Some(data) => unsafe {
                let ptr = data.as_non_null_ptr().cast::<usize>().sub(1);
                ConstNonNull::new(ptr.as_ptr())
            },
            None => None,
        }
    }

    /// Return len storage ptr.(allocated top pointer)
    #[inline]
    const fn len_ptr_mut(&mut self) -> Option<NonNull<usize>> {
        match self.data.as_mut() {
            Some(data) => unsafe { Some(data.as_non_null_ptr().cast::<usize>().sub(1)) },
            None => None,
        }
    }

    /// Set length to self storage.
    #[inline]
    const fn set_len(&mut self, len: usize) {
        // Safety: allocated size in allocate function.
        if let Some(data) = self.data.as_mut() {
            let len_ptr = unsafe { data.as_non_null_ptr().cast::<usize>().sub(1) };
            unsafe { len_ptr.write(len) };
        }
    }

    /// Return data ptr.
    #[inline]
    const fn data(&self) -> Option<NonNull<T>> {
        match self.data {
            Some(data) => Some(data.as_non_null_ptr()),
            None => None,
        }
    }

    /// Size + Element * N
    ///
    /// # Error
    /// If need count(alloc size) > isize::MAX.
    fn layout(count: usize) -> Layout {
        let layout = {
            const LEN_SIZE: usize = core::mem::size_of::<usize>(); // 8
            const LEN_ALIGN: usize = core::mem::align_of::<usize>(); // x64 => 8

            // Heap head is filled with usize len information
            let alloc_size = LEN_SIZE + (core::mem::size_of::<T>() * count);

            // IMPORTANT: Avoid undefined behavior.
            // When T alignment is less than or equal to usize, undefined behavior occurs when storing usize if alignment is made based on T criteria.
            // Therefore, the alignment must be kept above usize.
            let alignment = LEN_ALIGN.max(core::mem::align_of::<T>());

            Layout::from_size_align(alloc_size, alignment)
        };

        match layout {
            Ok(layout) => layout,
            Err(err) => panic!("SimpleArray alloc overflow: need size > isize::MAX: {err}"),
        }
    }

    /// Creates an iterator that borrows the elements of the `SimpleArray`.
    ///
    /// This method returns an iterator over references to the elements of the array. The iterator
    /// allows you to traverse the array without consuming it, meaning the array remains usable after
    /// the iteration.
    ///
    /// # Example
    ///
    /// ```rust
    /// use commonlibsse_ng::re::MemoryManager::SimpleArray::SimpleArray;
    /// let mut array = SimpleArray::with_capacity(3);
    /// let slice = array.as_mut_slice();
    /// slice[0] = 10;
    /// slice[1] = 20;
    /// slice[2] = 30;
    ///
    /// let sum: i32 = array.iter().sum();
    /// assert_eq!(sum, 60);
    /// ```
    #[inline]
    pub const fn iter(&self) -> SimpleArrayIterator<'_, T, A> {
        SimpleArrayIterator { array: self, index: 0, len: self.len() }
    }
}

impl<T, A: Allocator> Drop for SimpleArray<T, A> {
    fn drop(&mut self) {
        if let Some(len_ptr) = self.len_ptr_mut() {
            self.clear();
            unsafe { self.alloc.deallocate(len_ptr.cast(), Self::layout(self.len())) };
        }
    }
}

impl<T> Default for SimpleArray<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialEq, A: Allocator> PartialEq for SimpleArray<T, A> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for i in 0..self.len() {
            if self.get(i) != other.get(i) {
                return false;
            }
        }
        true
    }
}

impl<T: Eq, A: Allocator> Eq for SimpleArray<T, A> {}

impl<T: PartialOrd, A: Allocator> PartialOrd for SimpleArray<T, A> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.len() != other.len() {
            return None;
        }
        for i in 0..self.len() {
            match self[i].partial_cmp(&other[i]) {
                Some(Ordering::Equal) => {}
                Some(ordering) => return Some(ordering),
                None => return None,
            }
        }
        Some(Ordering::Equal)
    }
}

impl<T: Ord, A: Allocator> Ord for SimpleArray<T, A> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        if self.len() != other.len() {
            return self.len().cmp(&other.len());
        }
        for i in 0..self.len() {
            match self[i].cmp(&other[i]) {
                Ordering::Equal => {}
                ordering => return ordering,
            }
        }
        Ordering::Equal
    }
}

impl<T: Hash, A: Allocator> Hash for SimpleArray<T, A> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in 0..self.len() {
            self.get(i).hash(state);
        }
    }
}

impl<T, A: Allocator> core::ops::Index<usize> for SimpleArray<T, A> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len(), "Index out of bounds");
        unsafe { self.data().expect("Accessing empty array").add(index).as_ref() }
    }
}

impl<T, A: Allocator> core::ops::IndexMut<usize> for SimpleArray<T, A> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.len(), "Index out of bounds");
        unsafe { self.data().expect("Accessing empty array").add(index).as_mut() }
    }
}

// Iterator for borrowing `SimpleArray`
pub struct SimpleArrayIterator<'a, T, A>
where
    A: Allocator,
{
    array: &'a SimpleArray<T, A>,
    index: usize,
    len: usize, // store the length to avoid redundant calculations
}

impl<'a, T, A: Allocator> Iterator for SimpleArrayIterator<'a, T, A> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let item = unsafe { self.array.data()?.add(self.index).as_ref() };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

// IntoIterator for consuming `SimpleArray`
pub struct SimpleArrayIntoIterator<T, A: Allocator> {
    array: SimpleArray<T, A>,
    index: usize,
    len: usize, // store the length to avoid redundant calculations
}

impl<T, A: Allocator> Iterator for SimpleArrayIntoIterator<T, A> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let item = unsafe { ptr::read(self.array.data()?.add(self.index).as_ptr()) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<T, A: Allocator> IntoIterator for SimpleArray<T, A> {
    type Item = T;
    type IntoIter = SimpleArrayIntoIterator<T, A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        SimpleArrayIntoIterator { array: self, index: 0, len }
    }
}

impl<'a, T, A: Allocator> IntoIterator for &'a SimpleArray<T, A> {
    type Item = &'a T;
    type IntoIter = SimpleArrayIterator<'a, T, A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let array: SimpleArray<i32> = SimpleArray::new();
        assert_eq!(array.len(), 0);
        assert!(array.is_empty());
    }

    #[test]
    fn test_with_capacity() {
        let array: SimpleArray<i32> = SimpleArray::with_capacity(5);
        assert_eq!(array.len(), 5);
        assert!(!array.is_empty());
    }

    #[test]
    fn test_resize_grow() {
        let mut array = SimpleArray::<u32>::with_capacity(3);
        array.resize(5);
        assert_eq!(array.len(), 5);
    }

    #[test]
    fn test_resize_shrink() {
        let mut array = SimpleArray::<u32>::with_capacity(5);
        array.resize(2);
        assert_eq!(array.len(), 2);
    }

    #[test]
    fn test_data_access() {
        let mut array = SimpleArray::with_capacity(3);
        let slice = array.as_mut_slice();
        slice[0] = 10;
        slice[1] = 20;
        slice[2] = 30;

        let new_slice = array.as_slice();
        assert_eq!(new_slice, &[10, 20, 30]);
    }

    #[test]
    fn test_clear() {
        let mut array = SimpleArray::<u32>::with_capacity(3);
        array.clear();
        assert_eq!(array.len(), 0);
        assert!(array.is_empty());
    }

    #[test]
    fn test_as_slice() {
        let mut array = SimpleArray::with_capacity(4);
        let slice = array.as_mut_slice();
        slice.copy_from_slice(&[1, 2, 3, 4]);

        let new_slice = array.as_slice();
        assert_eq!(new_slice, &[1, 2, 3, 4]);
    }

    #[test]
    fn test_into_iterator() {
        let mut array = SimpleArray::with_capacity(5);
        let slice = array.as_mut_slice();
        slice[0] = 10;
        slice[1] = 20;
        slice[2] = 30;
        slice[3] = 40;
        slice[4] = 50;

        let collected: Vec<_> = array.into_iter().collect();
        assert_eq!(collected, vec![10, 20, 30, 40, 50]);
    }

    #[test]
    fn test_iterator() {
        let mut array = SimpleArray::with_capacity(5);
        array[0] = 10;
        array[1] = 20;
        array[2] = 30;
        array[3] = 40;
        array[4] = 50;

        let mut iter = array.iter();
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&20));
        assert_eq!(iter.next(), Some(&30));
        assert_eq!(iter.next(), Some(&40));
        assert_eq!(iter.next(), Some(&50));
        assert_eq!(iter.next(), None); // No more elements
    }
}

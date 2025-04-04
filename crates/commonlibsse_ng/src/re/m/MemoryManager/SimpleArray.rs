use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::mem::{self, MaybeUninit};
use core::ptr::{self, NonNull};
use core::slice;
use std::alloc::{Layout, alloc, dealloc};

/// Array whose first pointer is only a pointer to the length.
///
/// The array follows a specific layout:
///
/// | Index | Value      |
/// |-------|------------|
/// |       | Length (N) |
/// | 0     | Element 1  | <- Ptr pointed `self.data`
/// | 1     | Element 2  |
/// | 2     | Element 3  |
/// | ...   | ...        |
/// | N     | Element N  |
///
/// # Example
/// ```rust
/// use commonlibsse_ng::re::MemoryManager::SimpleArray::SimpleArray;
/// let array: SimpleArray<i32> = SimpleArray::new();
/// assert_eq!(array.len(), 0);
/// ```
pub struct SimpleArray<T> {
    data: Option<NonNull<T>>,
    marker: PhantomData<T>,
}

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
        Self { data: None, marker: PhantomData }
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
        match self.data {
            Some(ptr) => unsafe { *Self::head(ptr) },
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
        if let Some(ptr) = self.data.take() {
            unsafe {
                ptr::drop_in_place(slice::from_raw_parts_mut(ptr.as_ptr(), self.len()));
                dealloc(Self::head(ptr).cast(), Self::layout(self.len()));
            }
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
            let new_head = alloc(Self::layout(count)).cast::<usize>();
            if new_head.is_null() {
                panic!("Allocation failed");
            }
            *new_head = count; // The first pointer is the length.

            let new_data = new_head.add(1).cast::<T>();

            if let Some(ptr) = self.data {
                let ptr = ptr.as_ptr();
                if count < old_size {
                    ptr::copy_nonoverlapping(ptr, new_data, count);
                } else {
                    ptr::copy_nonoverlapping(ptr, new_data, old_size);
                    for i in old_size..count {
                        ptr::write(new_data.add(i), MaybeUninit::zeroed().assume_init());
                    }
                }
            } else {
                ptr::write_bytes(new_data, MaybeUninit::zeroed().assume_init(), count);
            }

            self.clear();
            self.data = Some(NonNull::new_unchecked(new_data));
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
        self.data.and_then(|ptr| {
            if index < self.len() { unsafe { Some(ptr.add(index).as_ref()) } } else { None }
        })
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
        self.data.and_then(|ptr| {
            if index < self.len() { Some(unsafe { ptr.add(index).as_mut() }) } else { None }
        })
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
        match self.data {
            Some(ptr) => unsafe { slice::from_raw_parts_mut(ptr.as_ptr(), self.len()) },
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
        match self.data {
            Some(ptr) => unsafe { slice::from_raw_parts(ptr.as_ptr(), self.len()) },
            None => &[],
        }
    }

    const unsafe fn head(ptr: NonNull<T>) -> *mut usize {
        unsafe { ptr.as_ptr().cast::<usize>().sub(1) }
    }

    /// Size + Element * N
    fn layout(count: usize) -> Layout {
        const LEN_SIZE: usize = mem::size_of::<usize>();
        let size = LEN_SIZE + (mem::size_of::<T>() * count);
        Layout::from_size_align(size, mem::align_of::<T>()).unwrap()
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
    pub const fn iter(&self) -> SimpleArrayIterator<'_, T> {
        SimpleArrayIterator { array: self, index: 0, len: self.len() }
    }
}

impl<T> Drop for SimpleArray<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T> Default for SimpleArray<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialEq> PartialEq for SimpleArray<T> {
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

impl<T: Eq> Eq for SimpleArray<T> {}

impl<T: PartialOrd> PartialOrd for SimpleArray<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.len() != other.len() {
            return None;
        }
        for i in 0..self.len() {
            match self[i].partial_cmp(&other[i]) {
                Some(Ordering::Equal) => continue,
                Some(ordering) => return Some(ordering),
                None => return None,
            }
        }
        Some(Ordering::Equal)
    }
}

impl<T: Ord> Ord for SimpleArray<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        if self.len() != other.len() {
            return self.len().cmp(&other.len());
        }
        for i in 0..self.len() {
            match self[i].cmp(&other[i]) {
                Ordering::Equal => continue,
                ordering => return ordering,
            }
        }
        Ordering::Equal
    }
}

impl<T: Hash> Hash for SimpleArray<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in 0..self.len() {
            self.get(i).hash(state);
        }
    }
}

impl<T> core::ops::Index<usize> for SimpleArray<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len(), "Index out of bounds");
        unsafe { self.data.expect("Accessing empty array").add(index).as_ref() }
    }
}

impl<T> core::ops::IndexMut<usize> for SimpleArray<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.len(), "Index out of bounds");
        unsafe { self.data.expect("Accessing empty array").add(index).as_mut() }
    }
}

// Iterator for borrowing `SimpleArray`
pub struct SimpleArrayIterator<'a, T> {
    array: &'a SimpleArray<T>,
    index: usize,
    len: usize, // store the length to avoid redundant calculations
}

impl<'a, T> Iterator for SimpleArrayIterator<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let item = unsafe { self.array.data?.add(self.index).as_ref() };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

// IntoIterator for consuming `SimpleArray`
pub struct SimpleArrayIntoIterator<T> {
    array: SimpleArray<T>,
    index: usize,
    len: usize, // store the length to avoid redundant calculations
}

impl<T> Iterator for SimpleArrayIntoIterator<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let item = unsafe { ptr::read(self.array.data?.add(self.index).as_ptr()) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<T> IntoIterator for SimpleArray<T> {
    type Item = T;
    type IntoIter = SimpleArrayIntoIterator<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        SimpleArrayIntoIterator { array: self, index: 0, len }
    }
}

impl<'a, T> IntoIterator for &'a SimpleArray<T> {
    type Item = &'a T;
    type IntoIter = SimpleArrayIterator<'a, T>;

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

use core::marker::PhantomData;
use core::ptr::NonNull;

/// Data storage unidirectional linked list of hashmaps formed on the heap or stack.
///
/// There is a possibility that this storage may always be zero
/// because of the timing of the zero initialization when the `reserve` method of the hashmap is called.
///
/// Therefore, use [`Option`].
#[repr(C)]
#[derive(Debug)]
pub struct EntryType<Pair> {
    /// key, value pair
    pub(super) value_data: Option<Pair>,
    pub(super) next: Option<NonNull<EntryType<Pair>>>,
}
const _: () = {
    // To avoid memory access violations, the smallest type (e.g., u8) other than the zero size
    // type must be larger than the sentinel size (4 bytes). or larger.
    const SIZE: usize = core::mem::size_of::<EntryType<u8>>();
    assert!(SIZE == 0x10);
};

impl<Pair> EntryType<Pair> where Pair: Default {}

impl<Pair> Default for EntryType<Pair> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Pair> EntryType<Pair> {
    /// Used as a sentinel value(a special value indicating the end of a table) used within a table
    pub const BST_SCATTER_TABLE_SENTINEL: [u8; 4] = [0xde, 0xad, 0xbe, 0xef];

    /// Create a new `EntryType<V>` as alloc zeroed
    #[inline]
    pub const fn new() -> Self {
        Self { value_data: None, next: None }
    }

    /// Is iter end
    pub fn is_sentinel(&self) -> bool {
        let is_zst = core::mem::size_of::<Self>() == 0;
        if is_zst {
            return false;
        }

        // Safety: Except for the zero size type, the size is 16 bytes even for the smallest type u8.
        // SENTINEL is 4bytes, so it is not an access violation.
        let entry_bytes = unsafe {
            let ptr: *const u8 = (self as *const Self).cast();
            core::slice::from_raw_parts(ptr, Self::BST_SCATTER_TABLE_SENTINEL.len())
        };
        Self::BST_SCATTER_TABLE_SENTINEL == entry_bytes
    }

    /// Has `next`?
    pub const fn has_next(&self) -> bool {
        self.next.is_some()
    }

    pub fn destroy(&mut self) {
        if self.has_next() {
            let _ = self.value_data.take();
            self.next = None;
        }
        debug_assert!(!self.has_next());
    }

    /// Set value_data & next
    pub fn push(&mut self, value: Pair, next: Option<NonNull<Self>>) {
        self.value_data = Some(value);
        self.next = next;
        // debug_assert!(self.has_next());
    }

    #[inline]
    pub const fn steal(&mut self) -> Option<Pair> {
        self.value_data.take()
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, Pair> {
        Iter { current: Some(NonNull::from(self)), _marker: PhantomData }
    }
}

#[derive(Debug)]
pub struct Iter<'a, Pair> {
    pub(crate) current: Option<NonNull<EntryType<Pair>>>,
    _marker: PhantomData<&'a Pair>,
}

impl<Pair> Iter<'_, Pair> {
    #[inline]
    pub const fn new(current: Option<NonNull<EntryType<Pair>>>) -> Self {
        Self { current, _marker: PhantomData }
    }
}

impl<'a, Pair> Iterator for Iter<'a, Pair> {
    type Item = &'a EntryType<Pair>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_ptr = self.current?;
        let current_ref = unsafe { current_ptr.as_ref() };
        if current_ref.is_sentinel() {
            return None;
        }
        self.current = current_ref.next;
        Some(current_ref)
    }
}

impl<'a, Pair> IntoIterator for &'a EntryType<Pair> {
    type Item = &'a EntryType<Pair>;
    type IntoIter = Iter<'a, Pair>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { current: Some(NonNull::from(self)), _marker: PhantomData }
    }
}

pub struct IterMut<'a, Pair> {
    current: Option<NonNull<EntryType<Pair>>>,
    _marker: PhantomData<&'a mut Pair>,
}

impl<'a, Pair> Iterator for IterMut<'a, Pair> {
    type Item = &'a mut EntryType<Pair>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_ptr = self.current?;
        let current_mut = unsafe { current_ptr.as_mut() };
        if current_mut.is_sentinel() {
            return None;
        }
        self.current = current_mut.next;
        Some(current_mut)
    }
}

impl<'a, Pair> IntoIterator for &'a mut EntryType<Pair> {
    type Item = &'a mut EntryType<Pair>;
    type IntoIter = IterMut<'a, Pair>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut { current: Some(NonNull::from(self)), _marker: PhantomData }
    }
}

pub struct IntoIter<Pair> {
    current: Option<NonNull<EntryType<Pair>>>,
    _marker: PhantomData<EntryType<Pair>>,
}

impl<Pair> Iterator for IntoIter<Pair> {
    type Item = Pair;

    fn next(&mut self) -> Option<Self::Item> {
        let ptr = self.current?;
        let owned_entry = unsafe { ptr.as_ptr().read() };
        if owned_entry.is_sentinel() {
            return None;
        }
        self.current = owned_entry.next;
        owned_entry.value_data
    }
}

impl<Pair> IntoIterator for EntryType<Pair> {
    type Item = Pair;
    type IntoIter = IntoIter<Pair>;

    fn into_iter(self) -> Self::IntoIter {
        let ptr = NonNull::new(&self as *const _ as *mut Self);
        #[allow(clippy::mem_forget)]
        core::mem::forget(self); // prevent drop
        IntoIter { current: ptr, _marker: PhantomData }
    }
}

impl<Pair> core::ops::Index<u32> for EntryType<Pair> {
    type Output = Self;

    fn index(&self, mut index: u32) -> &Self::Output {
        let mut current = self;

        while index > 0 {
            let next_ptr = current.next.expect("Index out of bounds");
            current = unsafe { next_ptr.as_ref() };
            index -= 1;
        }

        current
    }
}

impl<Pair> core::ops::IndexMut<u32> for EntryType<Pair> {
    fn index_mut(&mut self, mut index: u32) -> &mut Self::Output {
        let mut current = self;

        while index > 0 {
            let mut next_ptr = current.next.expect("Index out of bounds");
            current = unsafe { next_ptr.as_mut() };
            index -= 1;
        }

        current
    }
}

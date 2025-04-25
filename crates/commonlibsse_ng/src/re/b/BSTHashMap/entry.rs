use core::marker::PhantomData;
use core::num::NonZeroUsize;
use core::ptr::NonNull;

/// Used as a sentinel value(a special value indicating the end of a table) used within a table
const SENTINEL_ADDRESS: usize = 0xdeadbeef;

/// Sentinel-aware unique pointer
#[derive(Debug)]
#[repr(transparent)]
pub struct SentinelPtr<T: ?Sized> {
    ptr: NonNull<T>,
}

impl<T: ?Sized> Copy for SentinelPtr<T> {}
impl<T: ?Sized> Clone for SentinelPtr<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Default for SentinelPtr<T> {
    #[inline]
    fn default() -> Self {
        Self::sentinel()
    }
}

impl<T: ?Sized> PartialEq for SentinelPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        core::ptr::addr_eq(self.ptr.as_ptr(), other.ptr.as_ptr())
    }
}

impl<T> SentinelPtr<T> {
    /// Create sentinel ptr
    #[inline]
    pub const fn sentinel() -> Self {
        let ptr = core::ptr::without_provenance_mut(SENTINEL_ADDRESS);
        Self { ptr: unsafe { NonNull::new_unchecked(ptr) } }
    }
}

impl<T: ?Sized> SentinelPtr<T> {
    /// Create from NonNull
    pub const fn new(ptr: *mut T) -> Option<Self> {
        match NonNull::new(ptr) {
            Some(ptr) => Some(Self { ptr }),
            None => None,
        }
    }

    pub const fn from_non_null(ptr: NonNull<T>) -> Self {
        Self { ptr }
    }

    /// Is this a sentinel?
    pub fn is_sentinel(&self) -> bool {
        self.ptr.addr() == unsafe { NonZeroUsize::new_unchecked(SENTINEL_ADDRESS) }
    }

    /// Returns Some(ptr) if not sentinel
    pub fn as_unique(&self) -> Option<NonNull<T>> {
        if self.is_sentinel() { None } else { Some(self.ptr) }
    }

    /// Adds an offset to the pointer
    ///
    /// # Safety
    /// Caller must ensure pointer arithmetic is valid
    pub const unsafe fn add(self, offset: usize) -> Self
    where
        T: Sized,
    {
        Self { ptr: unsafe { self.ptr.add(offset) } }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    #[inline]
    pub const fn as_non_null(&self) -> NonNull<T> {
        self.ptr
    }

    /// Returns the reference
    ///
    /// Return `None` if this is a sentinel address.
    ///
    /// # Safety
    /// Caller must ensure this is not a sentinel
    pub unsafe fn as_non_sentinel_ref<'a>(&self) -> Option<&'a T> {
        if self.is_sentinel() {
            return None;
        }

        unsafe { Some(self.ptr.as_ref()) }
    }

    /// Returns the mutable reference
    ///
    /// Return `None` if this is a sentinel address.
    /// # Safety
    /// Caller must ensure this is not a sentinel
    pub unsafe fn as_non_sentinel_mut<'a>(&mut self) -> Option<&'a mut T> {
        if self.is_sentinel() {
            return None;
        }

        unsafe { Some(self.ptr.as_mut()) }
    }

    /// Returns reference regardless of sentinel. Use with care.
    ///
    /// # Safety
    /// - May cause UB if this is a sentinel.
    #[inline]
    pub const unsafe fn as_ref<'a>(&self) -> &'a T {
        unsafe { self.ptr.as_ref() }
    }

    /// Returns mutable reference regardless of sentinel. Use with care.
    ///
    /// # Safety
    /// - May cause UB if this is a sentinel.
    #[inline]
    pub const unsafe fn as_mut<'a>(&mut self) -> &'a mut T {
        unsafe { self.ptr.as_mut() }
    }

    pub fn addr(&self) -> NonZeroUsize {
        self.ptr.addr()
    }
}

impl<T> SentinelPtr<[T]> {
    /// Returns the length of the slice
    pub const fn len(&self) -> usize {
        self.ptr.len()
    }

    #[inline]
    pub const fn slice_from_raw_parts(data: NonNull<T>, len: usize) -> Self {
        Self { ptr: NonNull::slice_from_raw_parts(data, len) }
    }
}

impl<T: ?Sized> From<&T> for SentinelPtr<T> {
    /// Converts a `&T` to a `NonNull<T>`.
    ///
    /// This conversion is safe and infallible since references cannot be null.
    #[inline]
    fn from(r: &T) -> Self {
        Self { ptr: NonNull::from(r) }
    }
}

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
    pub(super) value_data: Pair,
    pub(super) next: Option<SentinelPtr<EntryType<Pair>>>,
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
        unsafe { core::mem::zeroed() }
    }
}

impl<Pair> EntryType<Pair> {
    /// Return `true` if already occupied.
    ///
    /// C++: `has_value`
    ///
    /// # How do we know if it's occupied by looking at the next pointer?
    ///
    /// The assumption is that `alloc_zeroed` is used.
    /// In this case, whether `has_next` is a null pointer or exists can be used to determine if it has been pushed or not.
    /// Since next is chained or a sentinel pointer is inserted at the time of `push`, the presence of a null pointer automatically proves that it is empty.
    #[inline]
    pub const fn is_occupied(&self) -> bool {
        self.next.is_some()
    }

    #[inline]
    pub fn destroy(&mut self) {
        if self.next.take().is_some() {
            unsafe { core::ptr::drop_in_place(&mut self.value_data) };
        }
        debug_assert!(!self.is_occupied());
    }

    /// Set value_data & next
    #[inline]
    pub fn push(&mut self, value: Pair, next: Option<SentinelPtr<Self>>) {
        self.destroy();
        self.value_data = value;
        self.next = next;
        debug_assert!(self.is_occupied());
    }

    #[inline]
    pub const fn steal(&mut self) -> Option<Pair> {
        Some(core::mem::replace(&mut self.value_data, unsafe { core::mem::zeroed() }))
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, Pair> {
        Iter { current: Some(SentinelPtr::from(self)), _marker: PhantomData }
    }
}

#[derive(Debug)]
pub struct Iter<'a, Pair> {
    pub(crate) current: Option<SentinelPtr<EntryType<Pair>>>,
    _marker: PhantomData<&'a Pair>,
}

impl<Pair> Iter<'_, Pair> {
    #[inline]
    pub const fn new(current: Option<SentinelPtr<EntryType<Pair>>>) -> Self {
        Self { current, _marker: PhantomData }
    }
}

impl<'a, Pair> Iterator for Iter<'a, Pair> {
    type Item = &'a EntryType<Pair>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_ptr = self.current?;
        let current_ref = unsafe { current_ptr.as_non_sentinel_ref()? };
        self.current = current_ref.next;
        Some(current_ref)
    }
}

impl<'a, Pair> IntoIterator for &'a EntryType<Pair> {
    type Item = &'a EntryType<Pair>;
    type IntoIter = Iter<'a, Pair>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { current: Some(SentinelPtr::from(self)), _marker: PhantomData }
    }
}

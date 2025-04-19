use core::marker::PhantomData;
use core::{alloc::Layout, fmt};
use std::ffi::{CStr, CString};

use std_fork::alloc::SelflessAllocator;
use stdx::unique::Unique;

use crate::re::MemoryManager::TESGlobalAlloc;

// NOTE: The `SStaticStringT` is omitted because it is not used.

/// `BSString` represents a string that stores its data as raw bytes from FFI with an undefined encoding.
///
/// It provides methods for managing the string's data and converting it to/from `CStr`.
/// This struct is designed for situations where string encoding isn't guaranteed, such as when dealing with FFI.
///
/// # Encoding
///
/// It has been confirmed that this string is also UTF-8 when esp. etc. are saved in UTF-8.
///
/// # Examples
///
/// ```rust
/// # use commonlibsse_ng::re::BSString::BSString;
/// let mut bs = BSString::new();
/// assert_eq!(bs.len(), 0);
///
/// bs.set_c_str(&c"Hello, Rust!");
/// assert_eq!(bs.len(), 13);
/// assert_eq!(bs.as_c_str().to_str(), Ok("Hello, Rust!"));
/// ```
#[repr(C)]
pub struct BSString<A = TESGlobalAlloc>
where
    A: SelflessAllocator,
{
    /// data coming from ffi should be `*mut c_char` because it could be null
    data: Option<Unique<u8>>,
    /// The number of bytes that the string currently contains that are valid. (Including null-terminated characters.)
    size: u16,
    /// Number of bytes allocated by the allocator
    capacity: u16,

    pad0C: u32,

    /// allocator API
    alloc: PhantomData<A>,
}
const _: () = assert!(core::mem::size_of::<BSString>() == 0x10);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BSStringError {
    /// string is too long to fit in a u16
    TooLong,
    /// allocation failed
    AllocFailed,
    /// string contains interior null bytes
    InteriorNul,
}

impl core::error::Error for BSStringError {}
impl fmt::Display for BSStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooLong => f.write_str("string is too long. max is u16::MAX(65535) bytes"),
            Self::AllocFailed => f.write_str("allocation failed"),
            Self::InteriorNul => f.write_str("string contains interior null bytes"),
        }
    }
}

impl BSString {
    /// Creates a new `BSString` instance with no data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use commonlibsse_ng::re::BSString::BSString;
    /// let bs = BSString::new();
    /// assert_eq!(bs.len(), 0);
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self::new_in()
    }
}

impl<A> BSString<A>
where
    A: SelflessAllocator,
{
    /// Creates a new, empty `BSString`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use commonlibsse_ng::re::BSString::BSString;
    /// use stdx::alloc::Global;
    ///
    /// let bs = BSString::<Global>::new_in();
    /// assert!(bs.is_empty());
    /// ```
    #[inline]
    pub const fn new_in() -> Self {
        Self { data: None, size: 0, capacity: 0, pad0C: 0, alloc: PhantomData }
    }

    /// Allocate a new `Self` from `&CStr` argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use commonlibsse_ng::re::BSString::BSString;
    /// let bs = BSString::from_c_str(c"Hello");
    /// assert_eq!(bs.as_c_str(), Ok(c"Hello"));
    /// ```
    ///
    /// # Errors
    /// - If the string is too long to fit in a `u16`, or if allocation fails.
    /// - If allocations fail.
    pub fn from_c_str(s: &CStr) -> Result<Self, BSStringError> {
        let mut string = Self::new_in();
        string.set_c_str(s)?;
        Ok(string)
    }

    /// Clears the string's data, resetting its size and capacity.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use commonlibsse_ng::re::BSString::BSString;
    /// let mut bs = BSString::new();
    /// bs.set_c_str(&c"Hello, Rust!");
    /// assert!(!bs.is_empty());
    /// bs.clear();
    /// assert!(bs.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        if self.capacity == 0 {
            return;
        }

        // Safety: avoid double free by `take`
        if let Some(ptr) = self.data.take() {
            unsafe { A::deallocate(ptr.as_non_null_ptr(), self.layout()) };
        }
        self.size = 0;
        self.capacity = 0;
    }

    /// Sets the content of the `BSString` from a `CStr`.
    ///
    /// This method will overwrite the current data, resizing if necessary.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use commonlibsse_ng::re::BSString::BSString;
    /// let mut bs = BSString::new();
    /// bs.set_c_str(&c"Hello, Rust!");
    /// assert_eq!(bs.len(), 13);
    /// ```
    ///
    /// # Errors
    /// - If the string is too long to fit in a `u16`, or if allocation fails.
    /// - If allocations fail.
    pub fn set_c_str(&mut self, cstr: &CStr) -> Result<(), BSStringError> {
        let bytes = cstr.to_bytes_with_nul();
        let len = bytes.len();

        if len == 0 {
            self.clear();
            return Ok(());
        }
        if len > (u16::MAX as usize) {
            return Err(BSStringError::TooLong);
        }

        let len = len as u16;

        let new_layout = Self::new_layout(len);

        let mut reuse = false;
        let new_ptr = unsafe {
            match self.data {
                Some(old_ptr) => {
                    let old_layout = self.layout();
                    if new_layout.size() > old_layout.size() {
                        A::grow(old_ptr.as_non_null_ptr(), old_layout, new_layout)
                            .map_err(|_| BSStringError::AllocFailed)?
                            .cast()
                    } else {
                        reuse = true;
                        old_ptr.as_non_null_ptr() // Current buffer is sufficient, so reuse as is
                    }
                }
                None => A::allocate(new_layout).map_err(|_| BSStringError::AllocFailed)?.cast(),
            }
        };

        unsafe { core::ptr::copy_nonoverlapping(bytes.as_ptr(), new_ptr.as_ptr(), len as usize) };

        self.data = Some(Unique::from(new_ptr));
        self.size = len;
        if !reuse {
            self.capacity = len;
        }
        Ok(())
    }

    /// Checks if the string is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use commonlibsse_ng::re::BSString::BSString;
    /// let bs = BSString::new();
    /// assert!(bs.is_empty());
    /// ```
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns the bytes length of the string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use commonlibsse_ng::re::BSString::BSString;
    /// let mut bs = BSString::new();
    /// bs.set_c_str(&c"Hello"); // Contains `\0`
    /// assert_eq!(bs.len(), 6);
    /// ```
    #[inline]
    pub const fn len(&self) -> usize {
        self.size as usize
    }

    /// Returns the capacity of the string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use commonlibsse_ng::re::BSString::BSString;
    /// let mut bs = BSString::new();
    /// bs.set_c_str(&c"Hello");
    /// assert!(bs.capacity() >= 5);
    /// ```
    #[inline]
    pub const fn capacity(&self) -> u16 {
        self.capacity
    }

    /// Returns the underlying bytes of the string, including the null terminator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use commonlibsse_ng::re::BSString::BSString;
    /// let mut bs = BSString::new();
    /// bs.set_c_str(&c"Hello");
    /// assert_eq!(bs.as_bytes_with_null(), b"Hello\0");
    /// ```
    #[inline]
    pub const fn as_bytes_with_null(&self) -> &[u8] {
        if self.size == 0 {
            return &[];
        }

        match self.data {
            Some(ref data) => unsafe { core::slice::from_raw_parts(data.as_ptr(), self.len()) },
            None => &[],
        }
    }

    /// Returns the string as a `CStr`, which is suitable for FFI.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use commonlibsse_ng::re::BSString::BSString;
    /// let mut bs = BSString::new();
    /// bs.set_c_str(&c"Hello");
    /// assert_eq!(bs.as_c_str().to_str(), Ok("Hello"));
    /// ```
    #[inline]
    pub const fn as_c_str(&self) -> &CStr {
        if self.size == 0 {
            return c"";
        }
        unsafe { CStr::from_bytes_with_nul_unchecked(self.as_bytes_with_null()) }
    }

    /// Gets a current layout self.
    ///
    /// # Panics
    /// On arithmetic overflow or when the total size would exceed
    /// `isize::MAX`, panic.
    fn layout(&self) -> Layout {
        Self::new_layout(self.capacity)
    }

    /// Creates a layout describing the record for a `[T; n]`.
    ///
    /// # Panics
    /// On arithmetic overflow or when the total size would exceed
    /// `isize::MAX`, panic.
    fn new_layout(n: u16) -> Layout {
        Layout::array::<u8>(n as usize).expect("BSTString need: alloc size < isize::MAX")
    }
}

impl<A> Drop for BSString<A>
where
    A: SelflessAllocator,
{
    #[inline]
    fn drop(&mut self) {
        self.clear();
    }
}

impl Default for BSString {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<A> fmt::Debug for BSString<A>
where
    A: SelflessAllocator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BSString")
            .field("data", &self.as_c_str())
            .field("size", &self.size)
            .field("capacity", &self.capacity)
            .finish()
    }
}

impl<A> Clone for BSString<A>
where
    A: SelflessAllocator,
{
    fn clone(&self) -> Self {
        if self.size == 0 {
            return Self::new_in();
        }
        let Some(data) = self.data else {
            return Self::new_in();
        };

        let len = self.size;
        let layout = Self::new_layout(len);

        let ptr = unsafe {
            let new_ptr = A::allocate(layout).expect("allocation failed in clone").cast();
            core::ptr::copy_nonoverlapping(data.as_ptr(), new_ptr.as_ptr(), len as usize);
            new_ptr
        };

        Self {
            data: Some(unsafe { Unique::new_unchecked(ptr.as_ptr()) }),
            size: len,
            capacity: len,
            pad0C: 0,
            alloc: PhantomData,
        }
    }
}

impl<A, B> PartialEq<BSString<B>> for BSString<A>
where
    A: SelflessAllocator,
    B: SelflessAllocator,
{
    fn eq(&self, other: &BSString<B>) -> bool {
        self.as_c_str() == other.as_c_str()
    }
}

impl<A> Eq for BSString<A> where A: SelflessAllocator {}

impl<A, B> PartialOrd<BSString<B>> for BSString<A>
where
    A: SelflessAllocator,
    B: SelflessAllocator,
{
    fn partial_cmp(&self, other: &BSString<B>) -> Option<core::cmp::Ordering> {
        self.as_c_str().partial_cmp(other.as_c_str())
    }
}

impl<A> Ord for BSString<A>
where
    A: SelflessAllocator,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_c_str().cmp(other.as_c_str())
    }
}

impl<A> core::hash::Hash for BSString<A>
where
    A: SelflessAllocator,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_c_str().hash(state);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

impl<A: SelflessAllocator> TryFrom<CString> for BSString<A> {
    type Error = BSStringError;

    fn try_from(c_string: CString) -> Result<Self, Self::Error> {
        let bytes = c_string.into_bytes_with_nul();
        let len = bytes.len();

        if len > u16::MAX as usize {
            return Err(BSStringError::TooLong);
        }

        let len_u16 = len as u16;
        let layout = Self::new_layout(len_u16);

        let ptr = unsafe {
            let ptr = A::allocate(layout).map_err(|_| BSStringError::AllocFailed)?.cast();
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr.as_ptr(), len);
            ptr
        };

        Ok(Self {
            data: Some(Unique::from(ptr)),
            size: len_u16,
            capacity: len_u16,
            pad0C: 0,
            alloc: PhantomData,
        })
    }
}

impl<A: SelflessAllocator> TryFrom<&str> for BSString<A> {
    type Error = BSStringError;

    #[inline]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let c_string = CString::new(s).map_err(|_| BSStringError::InteriorNul)?;
        Self::try_from(c_string)
    }
}

impl<A: SelflessAllocator> TryFrom<String> for BSString<A> {
    type Error = BSStringError;

    #[inline]
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type BSString = super::BSString<stdx::alloc::Global>;

    #[test]
    fn test_bs_string() {
        let mut bs = BSString::new_in();

        assert_eq!(bs.len(), 0);
        assert_eq!(bs.as_c_str().to_str(), Ok(""));

        let input = c"Hello, Rust!";
        bs.set_c_str(input).unwrap();

        assert_eq!(bs.len(), 13);
        assert_eq!(bs.as_c_str().to_str(), Ok("Hello, Rust!"));

        let input2 = c"Short";
        bs.set_c_str(input2).unwrap();

        assert_eq!(bs.len(), 6);
        assert_eq!(bs.as_c_str().to_str(), Ok("Short"));
    }

    #[test]
    fn test_too_long_cstr() {
        let too_large_vec = {
            let mut v = vec![b'a'; u16::MAX as usize + 1];
            v.push(0); // null terminator
            v
        };

        let too_large_c_string =
            CString::from_vec_with_nul(too_large_vec).expect("CString too long for CStr");

        let too_large_cstr = too_large_c_string.as_c_str();

        let mut bs = BSString::new_in();
        assert_eq!(bs.set_c_str(too_large_cstr), Err(BSStringError::TooLong));
    }
}

use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::ffi::{CStr, CString};

/// `BSString` represents a string that stores its data as raw bytes from FFI with an undefined encoding.
///
/// It provides methods for managing the string's data and converting it to/from `CStr`.
/// This struct is designed for situations where string encoding isn't guaranteed, such as when dealing with FFI.
///
/// The string is stored as a raw pointer, and its length and capacity are tracked separately. It can be cleared,
/// set with a C-style string, and accessed as raw bytes or a `CStr`.
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
pub struct BSString {
    // FIXME: data coming from ffi should be *mut c_char because it could be null
    /// Raw pointer from `Vec`
    data: NonNull<u8>,
    size: u16,
    capacity: u16,
    _pad0C: u32,
    marker: PhantomData<Vec<u8>>,
}

impl BSString {
    /// Creates a new, empty `BSString`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use commonlibsse_ng::re::BSString::BSString;
    /// let bs = BSString::new();
    /// assert!(bs.is_empty());
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self { data: NonNull::dangling(), size: 0, capacity: 0, _pad0C: 0, marker: PhantomData }
    }

    /// Allocate a new `Self` from `&CStr` argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use commonlibsse_ng::re::BSString::BSString;
    /// let bs = BSString::from_c_str(c"Hello");
    /// assert_eq!(bs.as_c_str(), c"Hello");
    /// ```
    pub fn from_c_str(s: &CStr) -> Self {
        let mut string = Self::new();
        string.set_c_str(s);
        string
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
        if self.capacity > 0 {
            let mut vec = unsafe {
                Vec::from_raw_parts(self.data.as_ptr(), self.size as usize, self.capacity as usize)
            };
            vec.clear();
            #[allow(clippy::mem_forget)]
            core::mem::forget(vec);
        }
        self.data = NonNull::dangling();
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
    pub fn set_c_str(&mut self, cstr: &CStr) {
        let bytes = cstr.to_bytes_with_nul();
        let len = bytes.len() as u16;

        if len == 0 {
            self.clear();
            return;
        }

        let mut vec = if self.size == 0 {
            Vec::with_capacity(len as usize)
        } else {
            unsafe {
                Vec::from_raw_parts(self.data.as_ptr(), self.size as usize, self.capacity as usize)
            }
        };

        if len > self.capacity {
            vec.reserve((len - self.capacity) as usize);
            self.capacity = vec.capacity() as u16;
        }

        vec.clear();
        vec.extend_from_slice(bytes);
        self.data = unsafe { NonNull::new_unchecked(vec.as_mut_ptr()) };
        #[allow(clippy::mem_forget)]
        core::mem::forget(vec);

        self.size = len;
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
    pub const fn len(&self) -> u16 {
        self.size
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
        unsafe { core::slice::from_raw_parts(self.data.as_ptr(), self.size as usize) }
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
}

impl Drop for BSString {
    fn drop(&mut self) {
        if self.capacity > 0 {
            drop(unsafe {
                Vec::from_raw_parts(self.data.as_ptr(), self.size as usize, self.capacity as usize)
            });
        }
    }
}

impl Default for BSString {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for BSString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BSString")
            .field("data", &self.as_c_str())
            .field("size", &self.size)
            .field("capacity", &self.capacity)
            .finish()
    }
}

impl From<CString> for BSString {
    #[inline]
    fn from(c_string: CString) -> Self {
        let mut vec = c_string.into_bytes_with_nul();
        let ptr = vec.as_mut_ptr();
        let size = vec.len() as u16;
        let capacity = vec.capacity() as u16;

        #[allow(clippy::mem_forget)]
        core::mem::forget(vec);

        Self {
            data: unsafe { NonNull::new_unchecked(ptr) },
            size,
            capacity,
            _pad0C: 0,
            marker: PhantomData,
        }
    }
}

#[test]
fn test_bs_string() {
    let mut bs = BSString::new();

    assert_eq!(bs.len(), 0);
    assert_eq!(bs.as_c_str().to_str(), Ok(""));

    let input = c"Hello, Rust!";
    bs.set_c_str(input);

    assert_eq!(bs.len(), 13);
    assert_eq!(bs.as_c_str().to_str(), Ok("Hello, Rust!"));

    let input2 = c"Short";
    bs.set_c_str(input2);

    assert_eq!(bs.len(), 6);
    assert_eq!(bs.as_c_str().to_str(), Ok("Short"));

    let input3 = c"Much longer string!";
    bs.set_c_str(input3);

    assert_eq!(bs.len(), 20);
    assert_eq!(bs.as_c_str().to_str(), Ok("Much longer string!"));
}

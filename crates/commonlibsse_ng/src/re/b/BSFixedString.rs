use crate::re::BSStringPool::{self, StringFormat, U16};
use core::{marker::PhantomData, mem, ptr};

/// A fixed-length internal string representation used for interacting with
/// the `BSStringPool`.
///
/// This is similar to `::core::ffi::CStr`, but supports both `char` and `wchar_t` formats
/// through the `StringFormat` trait.
///
/// # Type Parameters
/// - `T`: The string format (`U8` for `char` or `U16` for `wchar_t`).
///
/// # Safety
/// Since this is an FFI type, the encoding is not guaranteed. It may be UTF-8, ANSI,
/// UTF-16LE, or platform-specific wide encoding.
#[repr(transparent)]
pub struct BSFixedStringInternal<T>
where
    T: StringFormat,
{
    /// Pointer to the string data (null-terminated).
    data: *const T::Unit,
    marker: PhantomData<Box<T::Unit>>,
}

impl<T> BSFixedStringInternal<T>
where
    T: StringFormat,
{
    /// Tries to acquire a reference to the string in the string pool.
    ///
    /// If the string is already in the pool, its reference count is incremented.
    pub fn try_acquire(&self) {
        if let Some(proxy) = unsafe { self.get_proxy() } {
            proxy.acquire();
        }
    }

    /// Tries to release the string from the string pool.
    ///
    /// If the string is not null, it decrements its reference count.
    /// If the reference count reaches zero, the string is removed from the pool.
    pub fn try_release(&mut self) {
        if !self.data.is_null() {
            unsafe { BSStringPool::Entry::<T>::release(&self.data) };
            self.data = ptr::null();
        }
    }

    /// Gets a mutable reference to the `BSStringPool::Entry`.
    ///
    /// Returns `None` if the string data is null.
    ///
    /// # Safety
    /// This dereferences a pointer that may point to invalid memory.
    pub const unsafe fn get_proxy(&self) -> Option<&mut BSStringPool::Entry<T>> {
        if self.data.is_null() {
            return None;
        }

        let proxy_ptr = unsafe { self.data.sub(mem::size_of::<BSStringPool::Entry<T>>()) }
            as *mut BSStringPool::Entry<T>;
        unsafe { proxy_ptr.as_mut() }
    }

    /// Returns the length of the string.
    ///
    /// Returns `0` if the string data is null or the proxy is invalid.
    #[inline]
    pub fn count_bytes_with_null(&self) -> u32 {
        unsafe { self.get_proxy().map_or(0, |proxy| proxy.len()) }
    }

    /// Checks whether the string is empty.
    ///
    /// Returns `true` if the string length is zero.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.count_bytes_with_null() == 0
    }

    /// Converts this C string as a byte slice with null.
    #[inline]
    pub fn as_bytes_with_null(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self.data.cast::<u8>(),
                self.count_bytes_with_null() as usize,
            )
        }
    }
}

impl<T: StringFormat> Clone for BSFixedStringInternal<T> {
    #[inline]
    fn clone(&self) -> Self {
        let cloned = Self { data: self.data, marker: PhantomData };
        cloned.try_acquire();
        cloned
    }
}

impl PartialOrd for BSFixedString {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BSFixedString {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_bytes_with_null().cmp(other.as_bytes_with_null())
    }
}

impl core::hash::Hash for BSFixedString {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_bytes_with_null().hash(state);
    }
}

impl<T: StringFormat> Drop for BSFixedStringInternal<T> {
    #[inline]
    fn drop(&mut self) {
        self.try_release();
    }
}

pub use u8_bytes::{BSFixedString, ctor8};
pub use u16_wide::{BSFixedStringW, ctor16};

mod u8_bytes {
    use super::*;
    use crate::re::BSStringPool::U8;
    use core::ffi::{CStr, c_char};
    use core::ops::Deref;
    use core::{fmt, str};

    /// A fixed-length C string.
    ///
    /// This type is string-interlaced and reference-shares memory with the same string.
    ///
    /// # Encoding
    /// Since this is an FFI type, the encoding is not guaranteed. It may be UTF-8, ANSI,
    /// or some other platform-specific encoding, as it uses `char` internally.
    ///
    /// Therefore, when using this string, it is recommended to convert it to `&str`
    /// to handle it safely.
    ///
    /// - ref: [`Converting esp/string Encoding`](https://www.nexusmods.com/skyrimspecialedition/articles/32/)
    pub type BSFixedString = BSFixedStringInternal<U8>;

    /// Creates a new `BSFixedStringInternal` from a raw pointer.
    ///
    /// # Safety
    /// - `data` must be a valid null-terminated string pointer.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 67819, ae_id = 69161)]
    #[allow(clippy::use_self)]
    pub unsafe fn ctor8(data: *const c_char) -> BSFixedString {}

    /// A constant pointer to an empty string.
    pub const EMPTY_C_CHAR: *const c_char = c"".as_ptr();

    impl BSFixedString {
        /// const default
        pub const DEFAULT: Self = Self { data: EMPTY_C_CHAR, marker: PhantomData };

        /// Creates a new `BSFixedString` from a `&CStr`.
        ///
        /// ## Memory allocation
        /// - If the same string exists inside the allocator, no new memory allocation occurs.
        #[inline]
        pub fn new(data: &CStr) -> Self {
            unsafe { Self::new_unchecked(data.as_ptr()) }
        }

        /// Creates a new `BSFixedString` from a raw pointer.
        ///
        /// # Safety
        /// - `data` must be a valid null-terminated `char` string.
        ///
        /// ## Memory allocation
        /// - If the same string exists inside the allocator, no new memory allocation occurs.
        #[inline]
        pub unsafe fn new_unchecked(data: *const c_char) -> Self {
            unsafe { ctor8(data) }
        }

        /// Gets the string as a `CStr`.
        ///
        /// # Encoding
        /// The returned string may not be UTF-8 due to its FFI nature.
        /// It is safe to use this as a raw `CStr`, but converting to `str` should be done carefully.
        pub fn as_c_str(&self) -> &CStr {
            if let Some(proxy) = unsafe { self.get_proxy() } {
                unsafe {
                    return CStr::from_ptr(proxy.as_raw());
                }
            }
            unsafe { CStr::from_ptr(EMPTY_C_CHAR) }
        }

        /// Converts the string to `&str` if it is valid UTF-8.
        #[inline]
        pub fn to_str(&self) -> Option<&str> {
            core::str::from_utf8(self.as_bytes_with_null()).ok()
        }

        /// Returns true if `CStr` passed as argument is contained in this string or not.
        #[inline]
        pub fn contains(&self, rhs: &CStr) -> bool {
            let self_bytes = self.as_bytes_with_null();
            let rhs_bytes = rhs.to_bytes();
            let rhs_len = rhs_bytes.len();

            if rhs_len > self_bytes.len() {
                return false;
            }

            self_bytes.windows(rhs_len).any(|window| window == rhs_bytes)
        }
    }

    impl PartialEq for BSFixedString {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            if self.is_empty() && other.is_empty() {
                true
            } else {
                self.as_c_str() == other.as_c_str()
            }
        }
    }

    impl Eq for BSFixedString {}

    impl Default for BSFixedString {
        #[inline]
        fn default() -> Self {
            Self { data: EMPTY_C_CHAR, marker: PhantomData }
        }
    }

    impl Deref for BSFixedString {
        type Target = CStr;

        #[inline]
        fn deref(&self) -> &Self::Target {
            self.as_c_str()
        }
    }

    impl fmt::Display for BSFixedString {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.as_c_str().to_string_lossy())
        }
    }

    impl fmt::Debug for BSFixedString {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{:?}",
                str::from_utf8(self.as_bytes_with_null()).unwrap_or("<Invalid UTF-8>")
            )
        }
    }

    impl From<&CStr> for BSFixedString {
        fn from(value: &CStr) -> Self {
            Self::new(value)
        }
    }
}

mod u16_wide {
    use super::*;
    use core::fmt;
    use std::string::FromUtf16Error;

    /// Fixed length wide C string.
    ///
    /// # Encoding
    /// Since this is an FFI type, the encoding is not guaranteed. It may be UTF-16LE,
    /// UTF-32LE, or platform-specific wide encoding, as it uses `wchar_t` internally.
    ///
    /// Therefore, when using this string, it is recommended to convert it to `&str`
    /// to handle it safely.
    pub type BSFixedStringW = BSFixedStringInternal<U16>;

    /// Creates a new `BSFixedStringW` from a pointer
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 67834, ae_id = 69176)]
    #[allow(clippy::use_self)]
    pub unsafe fn ctor16(data: *const u16) -> BSFixedStringW {}

    impl BSFixedStringW {
        /// A constant pointer to an empty string.
        pub const EMPTY: &'static [u16] = &[];

        /// # Safety
        /// `data` must be a pointer to a null-terminated UTF-16LE string.
        #[inline]
        pub const unsafe fn new_unchecked(data: *const u16) -> Self {
            if !data.is_null() {
                Self { data, marker: PhantomData }
            } else {
                Self { data: Self::EMPTY.as_ptr(), marker: PhantomData }
            }
        }

        /// Convert as `[u16]`
        #[inline]
        pub fn as_wide(&self) -> &[u16] {
            unsafe { self.get_proxy() }.map_or(&[], |proxy| unsafe {
                core::slice::from_raw_parts(proxy.as_raw(), proxy.len() as usize)
            })
        }

        /// Decode a UTF-16–encoded slice v into a String
        #[inline]
        pub fn to_string_lossy(&self) -> String {
            String::from_utf16_lossy(self.as_wide())
        }

        /// Decode a UTF-16–encoded vector v into a String.
        /// # Errors
        /// Invalid UTF-16 encoding
        #[inline]
        pub fn to_string(&self) -> Result<String, FromUtf16Error> {
            String::from_utf16(self.as_wide())
        }
    }

    impl PartialEq for BSFixedStringW {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.as_wide() == other.as_wide()
        }
    }

    impl Default for BSFixedStringW {
        #[inline]
        fn default() -> Self {
            Self { data: Self::EMPTY.as_ptr(), marker: PhantomData }
        }
    }

    impl fmt::Display for BSFixedStringW {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.to_string_lossy())
        }
    }

    impl fmt::Debug for BSFixedStringW {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self.to_string_lossy())
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_bs_string() {
//         let mut bs_fixed_string = unsafe { BSFixedString::new_unchecked(c"Hello World".as_ptr()) };
//         assert_eq!(bs_fixed_string.count_bytes(), 12);
//         assert_eq!(bs_fixed_string.to_str(), Some("Hello World"));
//         assert!(bs_fixed_string.contains(c"World"));

//         bs_fixed_string.try_release();
//         assert!(bs_fixed_string.data.is_null());
//     }
// }

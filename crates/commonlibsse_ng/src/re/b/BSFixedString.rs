use crate::re::BSStringPool::{self, StringFormat, Utf16LE};
use core::{mem, ptr};

/// Like `::core::ffi::CStr`
#[repr(transparent)]
pub struct BSFixedStringInternal<T>
where
    T: StringFormat,
{
    // Pointer to string data (null-terminated)
    data: *const T::Unit,
}

impl<T> BSFixedStringInternal<T>
where
    T: StringFormat,
{
    pub fn try_acquire(&self) {
        if let Some(proxy) = self.get_proxy() {
            proxy.acquire();
        }
    }

    pub fn try_release(&mut self) {
        if !self.data.is_null() {
            unsafe { BSStringPool::Entry::<T>::release(&self.data) };
            self.data = ptr::null();
        }
    }

    pub fn get_proxy(&self) -> Option<&mut BSStringPool::Entry<T>> {
        if !self.data.is_null() {
            let proxy_ptr = unsafe {
                self.data.sub(mem::size_of::<BSStringPool::Entry<T>>())
                    as *mut BSStringPool::Entry<T>
            };
            unsafe { proxy_ptr.as_mut() }
        } else {
            None
        }
    }

    #[inline]
    pub fn len(&self) -> u32 {
        self.get_proxy().map_or(0, |proxy| proxy.len())
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: StringFormat> Clone for BSFixedStringInternal<T> {
    #[inline]
    fn clone(&self) -> Self {
        let cloned = Self { data: self.data };
        cloned.try_acquire();
        cloned
    }
}

impl<T: StringFormat> Drop for BSFixedStringInternal<T> {
    #[inline]
    fn drop(&mut self) {
        self.try_release();
    }
}

pub use utf8::{BSFixedString, ctor8};
pub use utf16le::{BSFixedStringW, ctor16};

mod utf8 {
    use super::*;
    use crate::re::BSStringPool::Utf8;
    use core::ffi::{CStr, c_char};
    use core::fmt;
    use core::ops::Deref;

    ///  I am not certain as to whether the internal [`c_char`] is ASCII only or UTF-8, but SkyrimSE seems to treat it as UTF-8 with i18n in mind.
    /// - ref: [`Converting esp/string Encoding`](https://www.nexusmods.com/skyrimspecialedition/articles/32/)
    pub type BSFixedString = BSFixedStringInternal<Utf8>;

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 67819, ae_id = 69161)]
    #[allow(clippy::use_self)]
    pub fn ctor8(data: *const c_char) -> BSFixedString {}

    impl BSFixedString {
        pub const EMPTY: *const c_char = c"".as_ptr();

        /// # Safety
        /// `data` is valid null terminated ASCII string
        #[inline]
        pub const unsafe fn new_unchecked(data: *const c_char) -> Self {
            if !data.is_null() {
                return Self { data };
            }
            Self { data: Self::EMPTY }
        }

        /// C++ `data` method
        pub fn as_c_str(&self) -> &CStr {
            if let Some(proxy) = self.get_proxy() {
                unsafe {
                    return CStr::from_ptr(proxy.as_raw());
                }
            }
            unsafe { CStr::from_ptr(Self::EMPTY) }
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

    impl Default for BSFixedString {
        #[inline]
        fn default() -> Self {
            Self { data: Self::EMPTY }
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
            write!(f, "{:?}", self.as_c_str())
        }
    }
}

mod utf16le {
    use super::*;
    use core::fmt;
    use windows::core::PCWSTR;

    /// The `wchar_t` type is an implementation-defined wide character type.
    ///
    /// In Microsoft compilers, it represents a 16-bit wide character used to store Unicode encoded as UTF-16LE.
    /// - ref: [`char、wchar_t、char8_t、char16_t、char32_t`](https://learn.microsoft.com/cpp/cpp/char-wchar-t-char16-t-char32-t?view=msvc-170)
    pub type BSFixedStringW = BSFixedStringInternal<Utf16LE>;

    /// Creates a new `BSFixedStringW` from a pointer to a null-terminated UTF-16LE string.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 67834, ae_id = 69176)]
    #[allow(clippy::use_self)]
    pub fn ctor16(data: *const u16) -> BSFixedStringW {}

    impl BSFixedStringW {
        pub const EMPTY: PCWSTR = PCWSTR::null();

        /// # Safety
        /// `data` must be a pointer to a null-terminated UTF-16LE string.
        #[inline]
        pub const unsafe fn new_unchecked(data: *const u16) -> Self {
            if !data.is_null() { Self { data } } else { Self { data: Self::EMPTY.0 } }
        }

        /// Convert `PCWSTR` to `[u16]`
        pub fn as_wide_pcwstr(&self) -> PCWSTR {
            self.get_proxy().map_or(Self::EMPTY, |proxy| PCWSTR(proxy.as_raw()))
        }
    }

    impl PartialEq for BSFixedStringW {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.as_wide_pcwstr() == other.as_wide_pcwstr()
        }
    }

    impl Default for BSFixedStringW {
        #[inline]
        fn default() -> Self {
            Self { data: Self::EMPTY.0 }
        }
    }

    impl fmt::Display for BSFixedStringW {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let wide_str = self.as_wide_pcwstr();
            write!(f, "{}", String::from_utf16_lossy(unsafe { wide_str.as_wide() }))
        }
    }

    impl fmt::Debug for BSFixedStringW {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self.as_wide_pcwstr())
        }
    }
}

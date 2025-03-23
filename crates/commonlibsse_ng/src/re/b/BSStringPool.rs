use crate::re::BSAtomic::BSSpinLock;
use core::{
    ffi::c_char,
    sync::atomic::{AtomicU16, Ordering},
};

#[derive(Debug)]
#[repr(C)]
pub struct Entry<T: StringFormat> {
    /// Pointer to the previous entry in the list.
    pub(crate) left: *mut Entry<T>,
    /// Flags for entry status and reference count.
    pub(crate) flags: AtomicU16,
    /// CRC checksum for the entry.
    pub(crate) crc: u16,
    /// Union that holds either length or a pointer to the next entry.
    pub(crate) length_or_right: LengthOrRight<T>,
}
const _: () = {
    assert!(core::mem::offset_of!(Entry<U8>, left) == 0x0);
    assert!(core::mem::offset_of!(Entry<U8>, flags) == 0x8);
    assert!(core::mem::offset_of!(Entry<U8>, crc) == 0xa);
    assert!(core::mem::offset_of!(Entry<U8>, length_or_right) == 0x10);
    assert!(core::mem::size_of::<Entry<U8>>() == 0x18);
};

/// A union representing either the length of the entry or a pointer to the next entry.
/// This allows for flexible storage of either data (length) or a pointer (right).
#[repr(C)]
pub union LengthOrRight<T: StringFormat> {
    /// Holds the length of the entry.
    length: u32,
    /// Holds a pointer to the next entry.
    right: *mut Entry<T>,
}
const _: [(); core::mem::size_of::<LengthOrRight<U8>>()] = [(); 0x8];

impl<T> core::fmt::Debug for LengthOrRight<T>
where
    T: StringFormat,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        unsafe {
            f.debug_struct("LengthOrRight")
                .field("length", &self.length)
                .field("right", &self.right)
                .finish()
        }
    }
}

impl<T: StringFormat> Entry<T> {
    /// Constant to represent a "wide" entry. This is used for determining if the entry is wide.
    pub const WIDE: u16 = 1 << 15;

    /// Mask used for reference count in flags.
    pub const REF_COUNT_MASK: u16 = 0x7FFF;

    /// Mask used to extract the length value from the `length_or_right` union.
    pub const LENGTH_MASK: u32 = 0xFFFFFF;

    /// Acquire the entry by incrementing its reference count.
    ///
    /// This method ensures atomicity by loading the current value of the reference count, and then
    /// attempts to increment the reference count. The operation is performed in a loop to ensure
    /// that the reference count does not overflow.
    pub fn acquire(&self) {
        let flags = &self.flags;
        let mut expected;
        loop {
            expected = flags.load(Ordering::Relaxed);
            if (expected & Self::REF_COUNT_MASK) >= Self::REF_COUNT_MASK {
                break;
            }
            if flags
                .compare_exchange_weak(
                    expected,
                    expected.wrapping_add(1),
                    Ordering::AcqRel,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                break;
            }
        }
    }

    /// Get the CRC checksum of the entry.
    #[inline]
    pub const fn crc(&self) -> u16 {
        self.crc
    }

    /// Returns the length of the entry, extracting it from the `length_or_right` union.
    ///
    /// This corresponds to the `length` or `size` function in the C++ API.
    ///
    /// # Returns
    ///
    /// Returns the length stored in the `length_or_right` union after applying the `LENGTH_MASK`.
    #[allow(clippy::len_without_is_empty)]
    #[inline]
    pub const fn len(&self) -> u32 {
        unsafe { self.length_or_right.length & Self::LENGTH_MASK }
    }

    /// Returns a raw pointer to the entry's data.
    ///
    /// This corresponds to the `data` function in the C++ API.
    #[inline]
    pub fn as_raw(&self) -> *const T::Unit {
        T::is_valid(self.is_wide());
        unsafe { (self as *const Self).add(1).cast::<T::Unit>() }
    }

    /// Releases the entry and performs the necessary cleanup. This function is unsafe because it
    /// involves dereferencing raw pointers.
    ///
    /// # Safety
    /// - The caller must ensure that the entry is properly allocated and not already released.
    /// - Not having a double free.
    #[inline]
    pub unsafe fn release(entry: &*const T::Unit) {
        unsafe { T::release(entry) };
    }

    /// Checks whether the entry is "wide" based on the flags.
    #[inline]
    fn is_wide(&self) -> bool {
        (self.flags.load(Ordering::Relaxed) & Self::WIDE) != 0
    }
}

/// The `StringFormat` trait defines the methods required to interact with a specific string format type.
///
/// This includes the type of unit (e.g., `u8` for `c_char` or `u16` for `w_char_t`) and methods for validation
/// and releasing memory associated with entries.
pub trait StringFormat {
    /// U8/U16
    type Unit;

    /// Releases the entry associated with the string format type.
    ///
    /// # Safety
    ///
    /// - The caller must ensure the entry is valid and allocated.
    /// - Not having a double free.
    unsafe fn release(entry: &*const Self::Unit);

    /// Validates the entry, ensuring the proper string format (e.g., UTF-8 or UTF-16).
    ///
    /// # Panics
    ///
    /// If the format does not match the expected type (wide or not), this function will panic.
    fn is_valid(is_wide: bool);
}

#[derive(Debug)]
pub enum U8 {}

impl StringFormat for U8 {
    type Unit = c_char;

    #[inline]
    unsafe fn release(entry: &*const Self::Unit) {
        unsafe { release8(entry) }
    }

    #[inline]
    fn is_valid(is_wide: bool) {
        assert!(!is_wide);
    }
}

/// The `wchar_t` type is an implementation-defined wide character type.
///
/// In Microsoft compilers, it represents a 16-bit wide character used to store Unicode encoded as UTF-16LE.
/// - ref: [`char、wchar_t、char8_t、char16_t、char32_t`](https://learn.microsoft.com/cpp/cpp/char-wchar-t-char16-t-char32-t?view=msvc-170)
#[derive(Debug)]
pub enum U16 {}

impl StringFormat for U16 {
    type Unit = u16;

    #[inline]
    unsafe fn release(entry: &*const Self::Unit) {
        unsafe { release16(entry) }
    }

    #[inline]
    fn is_valid(is_wide: bool) {
        assert!(is_wide);
    }
}

/// Releases a `c_char` entry. This is marked as `unsafe` because it operates directly on raw pointers.
#[commonlibsse_ng_derive_internal::relocate_fn(se_id = 67847, ae_id = 69192)]
pub unsafe fn release8(entry: &*const c_char) {}

/// Releases a `wchar_t` entry. This is marked as `unsafe` because it operates directly on raw pointers.
#[commonlibsse_ng_derive_internal::relocate_fn(se_id = 67848, ae_id = 69193)]
pub unsafe fn release16(entry: &*const u16) {}

#[repr(C)]
pub struct BucketTable {
    buckets: [*mut Entry<U8>; 0x10000], // 00000 - index using hash & kEntryIndexMask
    locks: [BSSpinLock; 0x10000 / 0x800], // 80000 - index using hash & kLockIndexMask
    initialized: bool,                  // 80100
}
const _: [(); core::mem::size_of::<BucketTable>()] = [(); 0x80108];

impl BucketTable {
    /// Returns the singleton instance of the `BucketTable`.
    ///
    /// This function is used to get a global instance for the table.
    #[allow(clippy::use_self)]
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 67855, ae_id = 69200)]
    pub fn get_singleton() -> *mut BucketTable {}
}

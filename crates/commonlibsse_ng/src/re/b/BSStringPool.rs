use crate::re::BSAtomic::BSSpinLock;
use core::{
    ffi::c_char,
    sync::atomic::{AtomicU16, Ordering},
};

#[derive(Debug)]
#[repr(C)]
pub struct Entry<T: StringFormat> {
    left: *mut Entry<T>,               // 00
    flags: AtomicU16,                  // 08
    crc: u16,                          // 0A
    length_or_right: LengthOrRight<T>, // 10
}
const _: [(); core::mem::size_of::<Entry<Utf8>>()] = [(); 0x18];

#[repr(C)]
union LengthOrRight<T: StringFormat> {
    _length: u32,
    _right: *mut Entry<T>,
}
const _: [(); core::mem::size_of::<LengthOrRight<Utf8>>()] = [(); 0x8];

impl<T> core::fmt::Debug for LengthOrRight<T>
where
    T: StringFormat,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        unsafe {
            f.debug_struct("LengthOrRight")
                .field("length", &self._length)
                .field("right", &self._right)
                .finish()
        }
    }
}

impl<T: StringFormat> Entry<T> {
    pub const WIDE: u16 = 1 << 15;
    pub const REF_COUNT_MASK: u16 = 0x7FFF;
    pub const LENGTH_MASK: u32 = 0xFFFFFF;

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

    pub const fn crc(&self) -> u16 {
        self.crc
    }

    /// C++ API `length`, `size`
    #[allow(clippy::len_without_is_empty)]
    #[inline]
    pub const fn len(&self) -> u32 {
        unsafe { self.length_or_right._length & Self::LENGTH_MASK }
    }

    /// C++ API `data`
    #[inline]
    pub fn as_raw(&self) -> *const T::Unit {
        T::is_valid(self.is_wide());
        unsafe { (self as *const Self).add(1).cast::<T::Unit>() }
    }

    /// # Safety
    #[inline]
    pub unsafe fn release(entry: &*const T::Unit) {
        T::release(entry);
    }

    #[inline]
    fn is_wide(&self) -> bool {
        (self.flags.load(Ordering::Relaxed) & Self::WIDE) != 0
    }
}

pub trait StringFormat {
    type Unit; // UTF-8: u8, UTF-16: u16

    fn release(entry: &*const Self::Unit);
    fn is_valid(is_wide: bool);
}

#[derive(Debug)]
pub enum Utf8 {}

impl StringFormat for Utf8 {
    type Unit = c_char;

    #[inline]
    fn release(entry: &*const Self::Unit) {
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
pub enum Utf16LE {}

impl StringFormat for Utf16LE {
    type Unit = u16;

    #[inline]
    fn release(entry: &*const Self::Unit) {
        unsafe { release16(entry) }
    }

    #[inline]
    fn is_valid(is_wide: bool) {
        assert!(is_wide);
    }
}

#[commonlibsse_ng_derive_internal::relocate_fn(se_id = 67847, ae_id = 69192)]
pub unsafe fn release8(entry: &*const c_char) {}
#[commonlibsse_ng_derive_internal::relocate_fn(se_id = 67848, ae_id = 69193)]
pub unsafe fn release16(entry: &*const u16) {}

#[repr(C)]
pub struct BucketTable {
    buckets: [*mut Entry<Utf8>; 0x10000], // 00000 - index using hash & kEntryIndexMask
    locks: [BSSpinLock; 0x10000 / 0x800], // 80000 - index using hash & kLockIndexMask
    initialized: bool,                    // 80100
}
const _: [(); core::mem::size_of::<BucketTable>()] = [(); 0x80108];

impl BucketTable {
    #[allow(clippy::use_self)]
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 67855, ae_id = 69200)]
    pub fn get_singleton() -> *mut BucketTable {}
}

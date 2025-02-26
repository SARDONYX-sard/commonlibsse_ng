// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// # Forked rust (ver. 1.84.0)
// See: https://github.com/rust-lang/rust/blob/1.84.0/library/std/src/sys/pal/windows/futex.rs
// See Rust license detail: https://github.com/rust-lang/rust/pull/43498

mod c {
    use windows::Win32::Foundation::GetLastError;
    pub use windows::Win32::System::Threading::{
        WaitOnAddress, WakeByAddressAll, WakeByAddressSingle, INFINITE,
    };

    use std::time::Duration;

    pub fn dur2timeout(dur: Duration) -> u32 {
        // Note that a duration is a (u64, u32) (seconds, nanoseconds) pair, and the
        // timeouts in windows APIs are typically u32 milliseconds. To translate, we
        // have two pieces to take care of:
        //
        // * Nanosecond precision is rounded up
        // * Greater than u32::MAX milliseconds (50 days) is rounded up to INFINITE
        //   (never time out).
        dur.as_secs()
            .checked_mul(1000)
            .and_then(|ms| ms.checked_add((dur.subsec_nanos() as u64) / 1_000_000))
            .and_then(|ms| {
                ms.checked_add(if dur.subsec_nanos() % 1_000_000 > 0 {
                    1
                } else {
                    0
                })
            })
            .map_or(INFINITE, |ms| {
                if ms > <u32>::MAX as u64 {
                    INFINITE
                } else {
                    ms as u32
                }
            })
    }

    /// Gets the error from the last function.
    /// This must be called immediately after the function that sets the error to
    /// avoid the risk of another function overwriting it.
    pub fn get_last_error() -> u32 {
        // SAFETY: This just returns a thread-local u32 and has no other effects.
        unsafe { GetLastError().0 }
    }

    pub const TIMEOUT: u32 = 1460;
}

use core::ffi::c_void;
use core::sync::atomic::{
    AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicPtr, AtomicU16,
    AtomicU32, AtomicU64, AtomicU8, AtomicUsize,
};
use core::time::Duration;
use core::{mem, ptr};

use c::dur2timeout;

/// An atomic for use as a futex that is at least 32-bits but may be larger
pub type Futex = AtomicU32;
/// Must be the underlying type of Futex
pub type Primitive = u32;

/// # Safety
/// inner trait
pub unsafe trait Futexable {}
/// # Safety
/// inner trait
pub unsafe trait Waitable {
    type Futex;
}
macro_rules! unsafe_waitable_int {
    ($(($int:ty, $atomic:ty)),*$(,)?) => {
        $(
            unsafe impl Waitable for $int {
                type Futex = $atomic;
            }
            unsafe impl Futexable for $atomic {}
        )*
    };
}
unsafe_waitable_int! {
    (bool, AtomicBool),
    (i8, AtomicI8),
    (i16, AtomicI16),
    (i32, AtomicI32),
    (i64, AtomicI64),
    (isize, AtomicIsize),
    (u8, AtomicU8),
    (u16, AtomicU16),
    (u32, AtomicU32),
    (u64, AtomicU64),
    (usize, AtomicUsize),
}
unsafe impl<T> Waitable for *const T {
    type Futex = AtomicPtr<T>;
}
unsafe impl<T> Waitable for *mut T {
    type Futex = AtomicPtr<T>;
}
unsafe impl<T> Futexable for AtomicPtr<T> {}

pub fn wait_on_address<W: Waitable>(
    address: &W::Futex,
    compare: W,
    timeout: Option<Duration>,
) -> bool {
    unsafe {
        let addr = ptr::from_ref(address).cast::<c_void>();
        let size = mem::size_of::<W>();
        let compare_addr = (&raw const compare).cast::<c_void>();
        let timeout = timeout.map_or(c::INFINITE, dur2timeout);
        c::WaitOnAddress(addr, compare_addr, size, Some(timeout)).is_ok()
    }
}

pub fn wake_by_address_single<T: Futexable>(address: &T) {
    unsafe {
        let addr = ptr::from_ref(address).cast::<c_void>();
        c::WakeByAddressSingle(addr);
    }
}

pub fn wake_by_address_all<T: Futexable>(address: &T) {
    unsafe {
        let addr = ptr::from_ref(address).cast::<c_void>();
        c::WakeByAddressAll(addr);
    }
}

pub fn futex_wait<W: Waitable>(futex: &W::Futex, expected: W, timeout: Option<Duration>) -> bool {
    // return false only on timeout
    wait_on_address(futex, expected, timeout) || c::get_last_error() != c::TIMEOUT
}

pub fn futex_wake<T: Futexable>(futex: &T) -> bool {
    wake_by_address_single(futex);
    false
}

pub fn futex_wake_all<T: Futexable>(futex: &T) {
    wake_by_address_all(futex);
}

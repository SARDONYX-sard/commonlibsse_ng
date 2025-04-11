use core::ffi::c_char;
use core::fmt;
use std::ffi::CString;

use crate::re::{BSString::BSString, TLSData::TLSData};

/// We have confirmed that any attempt to `print` beyond this size will result in a definite crash.
const LAST_MESSAGE_BUFFER_SIZE: usize = 0x400;

/// # Note
/// The console has also confirmed that it does not support ANSI Color.
#[repr(C)]
pub struct Console {
    pub lastMessage: [c_char; LAST_MESSAGE_BUFFER_SIZE],
    pub pad401: u8,
    pad402: u16,
    pad404: u32,
    buffer: BSString,
}
const _: () = assert!(core::mem::size_of::<Console>() == 0x418);

impl Console {
    /// Returns the singleton instance of `Self`.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut *mut Console",
        default = "None",
        deref_once,
        id(se = 515064, ae = 401203)
    )]
    #[inline]
    pub fn get_singleton() -> Option<&'static Console> {
        |deref_type: DerefType| unsafe { deref_type.as_ref() }
    }

    /// Returns the mutable singleton instance of `Self`.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut *mut Console",
        default = "None",
        deref_once,
        id(se = 515064, ae = 401203)
    )]
    #[inline]
    pub unsafe fn get_singleton_mut() -> Option<&'static mut Console> {
        |deref_type: DerefType| unsafe { deref_type.as_mut() }
    }

    #[inline]
    pub fn is_console_mode() -> bool {
        TLSData::get_static_tls_data().is_some_and(|tls| unsafe { tls.as_ref().consoleMode })
    }

    /// Print argument c-string.
    /// # Example
    /// ```no_run
    /// use commonlibsse_ng::re::Console::Console;
    ///
    /// if let Some(console) = Console::get_singleton_mut() {
    ///     console.print(c"Hello World!".as_ptr());
    /// };
    /// ```
    ///
    /// # Note
    /// More precisely, args: va_list follows after fmt, but since Rust does not support variadic arguments, it is omitted.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 50180, ae_id = 51110)]
    #[inline]
    pub fn print(&mut self, fmt: *const c_char) {}
}

/// Prints a formatted string to the in-game console (Skyrim).
///
/// This function takes a [`fmt::Arguments`] object and formats it as a string,
/// then passes it to the native `Console::print` function.
///
/// # Examples
///
/// ```no_run
/// use commonlibsse_ng::re::Console::print_fmt;
/// print_fmt(format_args!("Health: {}", 100));
/// ```
///
/// # Notes
/// - This function allocates on the heap due to `CString::new`.
/// - If the input string contains null bytes (`\0`), it will return early and not print anything.
/// - This is intended for internal use by [`console_print!`] and [`console_println!`] macros.
///
/// # Safety
/// Internally uses a mutable reference to the singleton `Console`, accessed via `unsafe`.
#[inline]
pub fn print_fmt(args: fmt::Arguments) {
    if let Some(console) = unsafe { Console::get_singleton_mut() } {
        let s = format!("{}", args);

        // We have confirmed that any attempt to `print` beyond this size will result in a definite crash.
        for chunk in s.as_bytes().chunks(LAST_MESSAGE_BUFFER_SIZE) {
            // Only try to print valid UTF-8 chunks
            if let Ok(chunk_str) = core::str::from_utf8(chunk) {
                if let Ok(c_str) = CString::new(chunk_str) {
                    console.print(c_str.as_ptr());
                }
            }
        }
    }
}

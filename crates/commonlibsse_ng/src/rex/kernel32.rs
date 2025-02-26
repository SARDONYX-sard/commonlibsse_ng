use std::ffi::c_void;

use windows::Win32::{Foundation::HMODULE, System::SystemServices::IMAGE_DOS_HEADER};

unsafe extern "C" {
    /// The memory-mapped first relative position address of the dll, which is automatically assigned by ms linker to
    /// the dll created by this library.
    /// - see: https://devblogs.microsoft.com/oldnewthing/20041025-00/?p=37483
    pub static __ImageBase: IMAGE_DOS_HEADER;
}

pub fn get_current_module() -> HMODULE {
    unsafe { HMODULE((&__ImageBase) as *const IMAGE_DOS_HEADER as *mut c_void) }
}

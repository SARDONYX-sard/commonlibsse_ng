// C++ Original code
// - https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/REL/Module.h
// - load_segments, clear: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/REL/Module.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MI

//! Module handling library for Skyrim SE/AE/VR .
//!
//! This module provides functionality to interact with loaded modules (executables and DLLs),
//! extract segment information, and parse NT headers.

use core::ffi::c_void;
use core::ptr::NonNull;

/// A handle that obtains and holds the address of the surviving dll/exe until the end of program execution.
///
/// # undefined behavior
/// If `Self::new` specifies a dll/exe that does not live until the end of program execution
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModuleHandle(pub(crate) NonNull<c_void>);

unsafe impl Send for ModuleHandle {}
unsafe impl Sync for ModuleHandle {}

impl Default for ModuleHandle {
    #[inline]
    fn default() -> Self {
        Self::const_default()
    }
}

impl ModuleHandle {
    #[inline]
    pub(crate) const fn const_default() -> Self {
        Self(NonNull::dangling())
    }

    /// Gets the module handle of a module (exe, dll, etc.) that is being loaded by the calling process.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::rel::module::ModuleHandle;
    /// use windows::core::h; // `h!` is utf-16 str macro.
    ///
    /// let handle = ModuleHandle::new(h!("kernel32.dll"));
    /// assert!(handle.is_ok());
    ///
    /// // If there is no extension, a `.dll` is automatically specified.(This is the behavior of `GetModuleHandleW` function.)
    /// let handle = ModuleHandle::new(h!("kernel32"));
    /// assert!(handle.is_ok());
    /// ```
    ///
    /// # Errors
    /// - Errors if a module is specified that is not loaded by the calling process.
    /// - If the specified module handle could not be obtained.
    ///
    /// # Safety
    /// It is safe as long as specify a dll/exe that survives the `'static` life time.
    pub unsafe fn new<H>(module_name: H) -> Result<Self, ModuleHandleError>
    where
        H: windows::core::Param<windows::core::PCWSTR>,
    {
        use snafu::ResultExt as _;
        use windows::Win32::System::LibraryLoader::GetModuleHandleW;

        // GetModuleHandleW: https://learn.microsoft.com/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew
        let handle =
            unsafe { GetModuleHandleW(module_name) }.with_context(|_| HandleNotFoundSnafu)?;

        // TODO: size of module(However, it incurs the overhead of a function call.
        // let _module_size = get_module_size(handle).with_context(|_| HandleNotFoundSnafu)?;

        // If it is null, it is not null because of an error in the previous Result.
        Ok(Self(unsafe { NonNull::new_unchecked(handle.0) }))
    }

    /// Attempt to parse NT Header part.
    ///
    /// # Errors
    /// When fail to parse as valid header.
    pub const fn try_as_nt_header(
        &self,
    ) -> Result<&windows::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS64, ModuleHandleError>
    {
        use windows::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS64;
        use windows::Win32::System::SystemServices::{
            IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, IMAGE_NT_SIGNATURE,
        };

        let dos_header = self.0.cast::<IMAGE_DOS_HEADER>();

        let e_lfanew_offset = {
            let dos_header = unsafe { dos_header.as_ref() };
            // If it is a valid exe or dll, the first two bytes are the letters `MZ`
            // (inverted with little endian by u16 and containing 0x5a4d) from the designer's name.
            let dos_magic = dos_header.e_magic;
            if dos_magic != IMAGE_DOS_SIGNATURE {
                return Err(ModuleHandleError::InvalidDosHeaderSignature { actual: dos_magic });
            }

            dos_header.e_lfanew as usize
        };

        // The nt_header exists at the position e_lfanew from the start of the dos_header, i.e., the binary data of the exe.
        let nt_header = unsafe {
            dos_header
                .byte_add(e_lfanew_offset) // Be careful not to mistakenly use `.add` or `.offset`.
                .cast::<IMAGE_NT_HEADERS64>()
                .as_ref()
        };

        let nt_signature = nt_header.Signature;
        if nt_signature == IMAGE_NT_SIGNATURE {
            Ok(nt_header)
        } else {
            Err(ModuleHandleError::InvalidNtHeader64Signature { actual: nt_signature })
        }
    }
}

/// Error types for module handle operations.
#[derive(Debug, Clone, PartialEq, Eq, snafu::Snafu)]
pub enum ModuleHandleError {
    /// Invalid module handle.
    NullHandle,

    /// Failed to get module handle for '{source}'
    HandleNotFound { source: windows::core::Error },
    /// Invalid dos header of this exe/dll. Expected `0x5a4d`, but got `{actual}`
    InvalidDosHeaderSignature { actual: u16 },
    /// Invalid NT header64.  Expected `PE\0\0`(0x4550), but got `{actual:X}`
    InvalidNtHeader64Signature { actual: u32 },
}

// fn get_module_size(handle: windows::Win32::Foundation::HMODULE) -> windows::core::Result<u32> {
//     use windows::Win32::System::ProcessStatus::GetModuleInformation;
//     use windows::Win32::System::ProcessStatus::MODULEINFO;
//     use windows::Win32::System::Threading::GetCurrentProcess;

//     const MODULEINFO_SIZE: u32 = core::mem::size_of::<MODULEINFO>() as u32;

//     let mut module_info = MODULEINFO::default();
//     unsafe {
//         GetModuleInformation(GetCurrentProcess(), handle, &mut module_info, MODULEINFO_SIZE)?
//     };

//     Ok(module_info.SizeOfImage)
// }

#[cfg(test)]
mod tests {
    use super::*;
    use windows::core::h;

    #[test]
    fn test_module_handle_nt_header() {
        let handle =
            unsafe { ModuleHandle::new(h!("msvcrt.dll")).unwrap_or_else(|err| panic!("{err}")) };
        let nt_header = handle.try_as_nt_header().unwrap_or_else(|err| panic!("{err}"));
        assert_ne!(nt_header.Signature, 0);
    }
}

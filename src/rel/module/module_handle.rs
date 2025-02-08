// C++ Original code
// - https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/REL/Module.h
// - load_segments, clear: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/REL/Module.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MI

// NOTE: If we implement `Drop` in ModuleHandle and call FreeLibrary in it, it will overflow the stack.
//
/// Wrapper type to safely hold and handle valid handle addresses provided by `GetModuleHandleW`.
///
/// # Unsafe this implementation
/// The module handle is the start of the exe, but if you don't know the end, you don't know how far is the valid memory range.
/// The current implementation could crash at any time.
/// It holds void ptr internally, but can be handled null-safely by getter by method.
///
/// # Why not use `HMODULE` as it is?
/// It is not thread-safe as it is because it holds raw_pointer.
///
/// Therefore, we can keep it safe by creating another type that is valid as long as it holds the pointer, with the restriction that it is only invalidated on drop.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModuleHandle(core::num::NonZeroUsize);

impl ModuleHandle {
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
    pub fn new<H>(module_name: H) -> Result<Self, ModuleHandleError>
    where
        H: windows::core::Param<windows::core::PCWSTR>,
    {
        use core::num::NonZeroUsize;
        use snafu::ResultExt as _;
        use windows::Win32::System::LibraryLoader::GetModuleHandleW;

        // GetModuleHandleW: https://learn.microsoft.com/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew
        let handle =
            unsafe { GetModuleHandleW(module_name) }.with_context(|_| HandleNotFoundSnafu)?;

        // TODO: size of module
        // let _module_size = {
        //     let mut module_info = windows::Win32::System::ProcessStatus::MODULEINFO::default();
        //     if let Err(err) = unsafe {
        //         windows::Win32::System::ProcessStatus::GetModuleInformation(
        //             windows::Win32::System::Threading::GetCurrentProcess(),
        //             handle,
        //             &mut module_info,
        //             core::mem::size_of::<windows::Win32::System::ProcessStatus::MODULEINFO>()
        //                 as u32,
        //         )
        //     } {
        //         panic!("Couldn't get module information: {err}");
        //     }

        //     dbg!(module_info.SizeOfImage)
        // };

        // If it is null, it is not null because of an error in the previous Result.
        // Therefore, we use `.unwrap()`.
        let handle = NonZeroUsize::new(handle.0 as usize).ok_or(ModuleHandleError::NullHandle)?;
        Ok(Self(handle))
    }

    /// Returns the raw HMODULE handle.
    #[inline]
    pub const fn to_hmodule(&self) -> windows::Win32::Foundation::HMODULE {
        windows::Win32::Foundation::HMODULE(self.0.get() as *mut core::ffi::c_void)
    }

    /// Returns the module handle itself (i.e., the virtual address of the exe located in the DRAM).
    #[inline]
    pub const fn as_raw(&self) -> usize {
        self.0.get()
    }

    /// Attempt to parse NT Header part.
    ///
    /// # Lifetime
    /// The reference is invalid if the module handle itself is dropped because it refers to the subsequent address of the module handle.
    ///
    /// # Errors
    /// When fail to parse as valid header.
    pub fn try_as_nt_header(
        &self,
    ) -> Result<&windows::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS64, ModuleHandleError>
    {
        use windows::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS64;
        use windows::Win32::System::SystemServices::{
            IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, IMAGE_NT_SIGNATURE,
        };

        let dos_header = {
            let module_handle_address = self.0.get();
            module_handle_address as *const IMAGE_DOS_HEADER
        };

        let e_lfanew_offset = {
            let dos_header = unsafe { *dos_header };
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
            // NOTE: &* is special and means treating a raw pointer as a reference.
            &*dos_header
                // Be careful not to mistakenly use `.add` or `.offset`.
                .byte_add(e_lfanew_offset)
                .cast::<IMAGE_NT_HEADERS64>()
        };

        let nt_signature = nt_header.Signature;
        if nt_signature == IMAGE_NT_SIGNATURE {
            Ok(nt_header)
        } else {
            Err(ModuleHandleError::InvalidNtHeader64Signature {
                actual: nt_signature,
            })
        }
    }
}

/// Error types for module handle operations.
#[derive(Debug, snafu::Snafu)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use windows::core::h;

    #[test]
    fn test_module_handle_as_raw() {
        let handle = ModuleHandle::new(h!("msvcrt.dll")).unwrap_or_else(|err| panic!("{err}"));
        assert!(handle.as_raw() > 0);
    }

    #[test]
    fn test_module_handle_nt_header() {
        let handle = ModuleHandle::new(h!("msvcrt.dll")).unwrap_or_else(|err| panic!("{err}"));
        let nt_header = handle
            .try_as_nt_header()
            .unwrap_or_else(|err| panic!("{err}"));
        assert_ne!(nt_header.Signature, 0);
    }
}

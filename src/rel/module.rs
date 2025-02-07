// C++ Original code
// - https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/REL/Module.h
// - load_segments, clear: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/REL/Module.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::rel::version::{get_file_version, FileVersionError, Version};
use snafu::ResultExt as _;
use std::{
    num::NonZeroUsize,
    sync::{atomic::Ordering, LazyLock, RwLock, RwLockWriteGuard, TryLockResult},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Segment {
    pub proxy_base: usize,
    pub address: u32,
    pub size: u32,
}

#[allow(unused)]
#[repr(usize)]
enum Name {
    Textx,
    Idata,
    Rdata,
    Data,
    Pdata,
    Tls,
    Textw,
    Gfids,
    Total,
}

impl Segment {
    #[inline]
    pub const fn new(proxy_base: usize, address: u32, size: u32) -> Self {
        Self {
            proxy_base,
            address,
            size,
        }
    }

    #[inline]
    pub const fn offset(&self) -> usize {
        (self.address as usize).wrapping_sub(self.proxy_base)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub filename: windows::core::HSTRING,
    pub file_path: String,
    segments: [Segment; 8],
    pub version: Version,
    pub base: Option<ModuleHandle>,
    pub runtime: Runtime,
}

/// Wrapper type to safely hold and handle valid handle addresses provided by `GetModuleHandleW`.
///
/// It holds void ptr internally, but can be handled null-safely by getter by method.
///
/// # Lifetime
/// - As long as this structure is alive, the module handler is valid.
/// - Call `FreeLibrary` at drop to invalidate the address.
///
/// # Why not use `HMODULE` as it is?
/// It is not thread-safe as it is because it holds raw_pointer.
///
/// Therefore, we can keep it safe by creating another type that is valid as long as it holds the pointer, with the restriction that it is only invalidated on drop.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModuleHandle(NonZeroUsize);

impl ModuleHandle {
    /// # Example
    /// ```
    /// use commonlibsse_ng::rel::module::ModuleHandle;
    /// use windows::core::h; // `h!` is utf-16 str macro.
    ///
    /// let handle = ModuleHandle::new(h!("C:\\Windows\\splwow64.exe")).unwrap();
    /// ```
    #[inline]
    pub fn new<H>(module_name: H) -> Result<Self, ModuleError>
    where
        H: windows::core::Param<windows::core::PCWSTR>,
    {
        let handle =
            unsafe { windows::Win32::System::LibraryLoader::GetModuleHandleW(module_name) }
                .with_context(|_| HandleNotFoundSnafu)?;

        // If it is null, it is not null because of an error in the previous Result.
        // Therefore, we use `.unwrap()`.
        Ok(Self(NonZeroUsize::new(handle.0 as usize).unwrap()))
    }

    #[inline]
    pub fn to_hmodule(self) -> windows::Win32::Foundation::HMODULE {
        windows::Win32::Foundation::HMODULE(self.0.get() as *mut core::ffi::c_void)
    }

    /// Returns the module handle itself (i.e., the virtual address of the exe located in the DRAM).
    #[inline]
    pub fn as_raw(&self) -> usize {
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
    ) -> Result<&windows::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS64, ModuleError> {
        use windows::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS64;
        use windows::Win32::System::SystemServices::{
            IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, IMAGE_NT_SIGNATURE,
        };

        let dos_header = {
            let module_handle_address = self.0.get();
            module_handle_address as *const IMAGE_DOS_HEADER
        };

        {
            // If it is a valid exe or dll, the first two bytes are the letters `MZ`
            // (inverted with little endian by u16 and containing 0x5a4d) from the designer's name.
            let signature = unsafe { *dos_header }.e_magic;
            if (unsafe { *dos_header }).e_magic != IMAGE_DOS_SIGNATURE {
                return Err(ModuleError::InvalidDosHeaderSignature { actual: signature });
            }
        }

        // The nt_header exists at the position e_lfanew from the start of the dos_header, i.e., the binary data of the exe.
        let nt_header = unsafe {
            &*(dos_header.add((*dos_header).e_lfanew as usize) as *const IMAGE_NT_HEADERS64)
        };

        let nt_signature = nt_header.Signature;
        if nt_signature == IMAGE_NT_SIGNATURE {
            Ok(nt_header)
        } else {
            Err(ModuleError::InvalidNtHeader64Signature {
                actual: nt_signature,
            })
        }
    }
}

impl Drop for ModuleHandle {
    fn drop(&mut self) {
        let h_module = self.clone().to_hmodule();
        if let Err(err) = unsafe { windows::Win32::Foundation::FreeLibrary(h_module) } {
            #[cfg(feature = "tracing")]
            tracing::error!("Failed to free library: {err}",);
        }
    }
}

#[derive(Debug, snafu::Snafu)]
pub enum ModuleError {
    /// Failed to get module handle for '{source}'
    HandleNotFound { source: windows::core::Error },
    /// Invalid dos header of this exe/dll. Expected `0x5a4d`, but got `{actual}`
    InvalidDosHeaderSignature { actual: u16 },
    /// Invalid NT header64.  Expected `PE\0\0`(0x4550), but got `{actual:X}`
    InvalidNtHeader64Signature { actual: u32 },
}

static IS_CLEARED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static MODULE: LazyLock<RwLock<Module>> = LazyLock::new(|| RwLock::new(Module::init()));

impl Module {
    const ENVIRONMENT_W: &windows::core::HSTRING = windows::core::h!("SKSE_RUNTIME");

    /// TODO: phf table or mut HashMap?
    const SEGMENTS: [(&str, u32); 8] = [
        (".text", 0),
        (".idata", 0),
        (".rdata", 0),
        (".data", 0),
        (".pdata", 0),
        (".tls", 0),
        (".text", 0),
        (".gfids", 0),
    ];

    const RUNTIMES: [&'static windows::core::HSTRING; 2] = [
        windows::core::h!("SkyrimVR.exe"),
        windows::core::h!("SkyrimSE.exe"),
    ];

    // static mut INSTANCE: Option<Module> = None;
    // static INIT_LOCK: Lazy<Mutex<()>> = Lazy::new(||Mutex::new(()));

    /// Get mutable singleton instance.
    pub fn get_or_init_mut() -> TryLockResult<RwLockWriteGuard<'static, Module>> {
        if IS_CLEARED.load(Ordering::Acquire) {
            if let Ok(mut guard) = MODULE.try_write() {
                *guard = Self::init();
            };
            IS_CLEARED.store(true, Ordering::Release);
            MODULE.try_write()
        } else {
            MODULE.try_write()
        }
    }

    pub fn init() -> Self {
        use windows::core::HSTRING;
        use windows::Win32::System::Environment::GetEnvironmentVariableW;

        // buffer size: https://github.com/search?q=repo%3Arust-lang%2Frust%20GetEnvironmentVariableW&type=code
        let mut filename = vec![0; windows::Win32::Foundation::MAX_PATH as usize]; // MAX 260

        // - fn ref: https://learn.microsoft.com/windows/win32/api/processenv/nf-processenv-getenvironmentvariablew
        let filename_len =
            unsafe { GetEnvironmentVariableW(Self::ENVIRONMENT_W, Some(&mut filename)) } as usize;

        let mut filename = HSTRING::from_wide(&filename);
        let mut module_handle = None;
        let is_failed = filename_len != filename.len() - 1 || filename_len == 0;
        if is_failed {
            for runtime in Self::RUNTIMES {
                if let Ok(new_handle) = ModuleHandle::new(runtime) {
                    filename = runtime.clone();
                    module_handle = Some(new_handle);
                    break;
                }
            }
        };

        let file_path = filename.to_string();
        let mut segments = None;
        let (version, runtime) = if let Some(module_handle) = &module_handle {
            segments = Self::load_segments(module_handle).ok();
            match Self::load_version(&file_path) {
                Ok((new_version, new_runtime)) => (Some(new_version), Some(new_runtime)),
                Err(_err) => (None, None),
            }
        } else {
            (None, None)
        };

        Self {
            filename,
            file_path,
            segments: segments.unwrap_or_default(),
            version: version.unwrap_or(Version::const_default()),
            base: module_handle,
            runtime: runtime.unwrap_or_default(),
        }
    }

    #[inline]
    pub fn reset() {
        if let Err(err) = MODULE.write().map(|mut instance| {
            instance.clear();
            IS_CLEARED.store(true, std::sync::atomic::Ordering::Relaxed);
        }) {
            #[cfg(feature = "tracing")]
            tracing::error!("Couldn't clear MODULE instance.{err}");
        };
    }

    #[inline]
    pub const fn segment(&self, segment: usize) -> &Segment {
        &self.segments[segment]
    }

    #[inline]
    pub const fn get_runtime(&self) -> Runtime {
        self.runtime
    }

    /// Is the current Skyrim Runtime the AE version?
    #[inline]
    pub fn is_ae(&self) -> bool {
        self.get_runtime() == Runtime::Ae
    }

    /// Is the current Skyrim Runtime the SE version?
    #[inline]
    pub fn is_se(&self) -> bool {
        self.get_runtime() == Runtime::Se
    }

    /// Is the current Skyrim Runtime the VR version?
    #[inline]
    pub fn is_vr(&self) -> bool {
        self.get_runtime() == Runtime::Vr
    }

    fn load_segments(module_handle: &ModuleHandle) -> Result<[Segment; 8], ModuleError> {
        use windows::Win32::System::Diagnostics::Debug::{
            IMAGE_NT_HEADERS64, IMAGE_SECTION_HEADER,
        };

        let nt_header = module_handle.try_as_nt_header()?;
        let section_header_offset = {
            let optional_header_offset = core::mem::offset_of!(IMAGE_NT_HEADERS64, OptionalHeader);
            optional_header_offset + nt_header.FileHeader.SizeOfOptionalHeader as usize
        };

        let section = ((nt_header as *const _ as usize) + section_header_offset)
            as *const IMAGE_SECTION_HEADER;
        let section_len = core::cmp::min(
            nt_header.FileHeader.NumberOfSections,
            Self::SEGMENTS.len() as u16,
        );

        let mut segments = [Segment::default(); 8];
        for i in 0..section_len {
            let current_section = unsafe { &*section.add(i as usize) };

            let maybe_found = Self::SEGMENTS.iter().enumerate().find(|(_, elem)| {
                let maybe_ascii = core::str::from_utf8(&current_section.Name);
                if let Ok(section_name) = maybe_ascii {
                    elem.0 != section_name && ((current_section.Characteristics.0 & elem.1) != 0)
                } else {
                    false
                }
            });

            if let Some((idx, _)) = maybe_found {
                segments[idx] = Segment::new(
                    module_handle.as_raw(),
                    current_section.VirtualAddress,
                    current_section.SizeOfRawData,
                );
            }
        }
        Ok(segments)
    }

    #[inline]
    fn load_version(file_path: &str) -> Result<(Version, Runtime), FileVersionError> {
        let version = get_file_version(file_path)?;
        let runtime = match version.minor() {
            4 => Runtime::Vr,
            6 => Runtime::Ae,
            _ => Runtime::Se,
        };

        Ok((version, runtime))
    }

    pub fn clear(&mut self) {
        // if let Some(module) = self.injected_module {
        //     unsafe { FreeLibrary(module) };
        //     self.injected_module = None;
        // }
        self.base = None;
        self.file_path.clear();
        self.filename = windows::core::HSTRING::default();
        self.runtime = Runtime::Ae;
        self.segments = [Segment::new(0, 0, 0); 8];
        self.version = Version::const_default();

        // ID_DATABASE.lock().unwrap().clear();
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Runtime {
    /// Unknown runtime
    #[default]
    Unknown = 0,
    /// The Skyrim runtime is a post-Anniversary Edition Skyrim SE release (version 1.6.x and later).
    Ae = 1,
    /// The Skyrim runtime is a pre-Anniversary Edition Skyrim SE release (version 1.5.97 and prior).
    Se = 1 << 1,
    /// The Skyrim runtime is Skyrim VR.
    Vr = 1 << 2,
}

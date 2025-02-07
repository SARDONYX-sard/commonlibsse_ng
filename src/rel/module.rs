// C++ Original code
// - https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/REL/Module.h
// - load_segments, clear: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/REL/Module.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Module handling library for Skyrim SE/AE/VR .
//!
//! This module provides functionality to interact with loaded modules (executables and DLLs),
//! extract segment information, and parse NT headers.

mod module_handle;
mod runtime;
mod segment;

pub use self::module_handle::{ModuleError, ModuleHandle};
pub use self::runtime::Runtime;
pub use self::segment::{Segment, SegmentName};

use crate::rel::version::{get_file_version, FileVersionError, Version};
use std::sync::{atomic::Ordering, LazyLock, RwLock, RwLockWriteGuard, TryLockResult};
use windows::Win32::System::Diagnostics::Debug::{
    IMAGE_SCN_MEM_EXECUTE, IMAGE_SCN_MEM_WRITE, IMAGE_SECTION_CHARACTERISTICS,
};

/// Represents a loaded module in memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    /// Name of the module. (e.g. `"SkyrimSE.exe"`)
    pub filename: windows::core::HSTRING,
    /// File path of the module. (e.g. `"SkyrimSE.exe"`)
    pub file_path: String,
    /// Memory segments of the module.
    segments: [Segment; 8],
    /// Version information of the module.
    pub version: Version,
    /// Base module handle if available.
    pub base: Option<ModuleHandle>,
    /// Runtime type of the module.
    pub runtime: Runtime,
}

static IS_CLEARED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static MODULE: LazyLock<RwLock<Module>> = LazyLock::new(|| RwLock::new(Module::init()));

impl Module {
    const SEGMENTS: [(&str, IMAGE_SECTION_CHARACTERISTICS); 8] = [
        (".text", IMAGE_SCN_MEM_EXECUTE),
        (".idata", IMAGE_SECTION_CHARACTERISTICS(0)),
        (".rdata", IMAGE_SECTION_CHARACTERISTICS(0)),
        (".data", IMAGE_SECTION_CHARACTERISTICS(0)),
        (".pdata", IMAGE_SECTION_CHARACTERISTICS(0)),
        (".tls", IMAGE_SECTION_CHARACTERISTICS(0)),
        (".text", IMAGE_SCN_MEM_WRITE),
        (".gfids", IMAGE_SECTION_CHARACTERISTICS(0)),
    ];

    const RUNTIMES: [&'static windows::core::HSTRING; 2] = [
        // Use `msvcrt.dll` for testing since the dll is always US English and
        // always loaded in the msvc target when the test is run.
        #[cfg(feature = "debug")]
        windows::core::h!("msvcrt.dll"),
        #[cfg(not(feature = "debug"))]
        windows::core::h!("SkyrimSE.exe"),
        windows::core::h!("SkyrimVR.exe"),
    ];

    // static mut INSTANCE: Option<Module> = None;
    // static INIT_LOCK: Lazy<Mutex<()>> = Lazy::new(||Mutex::new(()));

    /// Get mutable singleton instance.
    ///
    /// # Errors
    /// If the caller who obtained the lock panics.
    pub fn get_or_init_mut() -> TryLockResult<RwLockWriteGuard<'static, Self>> {
        if IS_CLEARED.load(Ordering::Acquire) {
            if let Ok(mut guard) = MODULE.try_write() {
                *guard = Self::init();
            };
            IS_CLEARED.store(true, Ordering::Release);
        }

        MODULE.try_write()
    }

    /// Initializes a new `Module` instance by detecting the currently loaded module.
    ///
    /// This method attempts to retrieve the module information from the `SKSE_RUNTIME`
    /// or fallback to a predefined list of runtime binaries(e.g. `SkyrimSE.exe`).
    ///
    /// # Examples
    /// ```no_run
    /// use commonlibsse_ng::rel::module::Module;
    ///
    /// let module = Module::init();
    /// println!("Loaded module: {}", module.file_path);
    /// ```
    pub fn init() -> Self {
        use windows::core::{h, HSTRING};
        use windows::Win32::System::Environment::GetEnvironmentVariableW;

        // buffer size: https://github.com/search?q=repo%3Arust-lang%2Frust%20GetEnvironmentVariableW&type=code
        let mut filename = vec![0; windows::Win32::Foundation::MAX_PATH as usize]; // MAX 260

        // - fn ref: https://learn.microsoft.com/windows/win32/api/processenv/nf-processenv-getenvironmentvariablew
        let filename_len =
            unsafe { GetEnvironmentVariableW(h!("SKSE_RUNTIME"), Some(&mut filename)) } as usize;

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
        let (version, runtime) = module_handle
            .as_ref()
            .map_or((None, None), |module_handle| {
                segments = Self::load_segments(module_handle).ok();
                match Self::load_version(&file_path) {
                    Ok((new_version, new_runtime)) => (Some(new_version), Some(new_runtime)),
                    Err(_err) => (None, None),
                }
            });

        Self {
            filename,
            file_path,
            segments: segments.unwrap_or_default(),
            version: version.unwrap_or(Version::const_default()),
            base: module_handle,
            runtime: runtime.unwrap_or_default(),
        }
    }

    /// Gets a specific memory segment by [`SegmentName`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use commonlibsse_ng::rel::module::{Module, SegmentName};
    ///
    /// let module = Module::init();
    /// let text_segment = module.segment(SegmentName::Textx);
    /// ```
    #[inline]
    pub const fn segment(&self, name: SegmentName) -> Segment {
        self.segments[name as usize]
    }

    /// Resets the module instance, clearing its internal state.
    ///
    /// This function resets the state of the object by:
    /// - Disables the module handle.
    /// - Clearing file path and filename.
    /// - Resetting runtime to `Runtime::Ae`.
    /// - Resetting segments to default values.
    /// - Resetting version to the default version.
    ///
    /// # Examples
    /// ```no_run
    /// use commonlibsse_ng::rel::module::Module;
    ///
    /// Module::reset();
    /// // The module's internal state is now cleared
    /// ```
    ///
    /// # Error log
    /// log if the global module lock is poisoned.
    ///
    /// [`FreeLibrary`]: https://learn.microsoft.com/windows/win32/api/libloaderapi/nf-libloaderapi-freelibrary
    #[inline]
    pub fn reset() {
        if let Err(err) = MODULE.write().map(|mut instance| {
            instance.clear();
        }) {
            #[cfg(feature = "tracing")]
            tracing::error!("Couldn't clear MODULE instance.{err}");
        };
    }

    /// Returns the runtime type of the module.
    ///
    /// # Examples
    /// ```no_run
    /// use commonlibsse_ng::rel::module::Module;
    ///
    /// let module = Module::init();
    /// println!("Runtime: {:?}", module.get_runtime()); // e.g. Runtime::Se
    /// ```
    #[inline]
    pub const fn get_runtime(&self) -> Runtime {
        self.runtime
    }

    /// Is the current Skyrim runtime the Anniversary Edition (AE)?
    #[inline]
    pub fn is_ae(&self) -> bool {
        self.get_runtime() == Runtime::Ae
    }

    /// Is the current Skyrim runtime the Special Edition (SE).
    #[inline]
    pub fn is_se(&self) -> bool {
        self.get_runtime() == Runtime::Se
    }

    /// Is the current Skyrim runtime the VR version?
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
                maybe_ascii.is_ok_and(|section_name| {
                    elem.0 != section_name
                        && ((current_section.Characteristics & elem.1)
                            != IMAGE_SECTION_CHARACTERISTICS(0))
                })
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

    fn clear(&mut self) {
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
        IS_CLEARED.store(true, std::sync::atomic::Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_init() {
        let module = dbg!(Module::init());
        assert!(!module.file_path.is_empty());
    }

    #[test]
    fn test_module_reset() {
        Module::reset();
        assert!(IS_CLEARED.load(std::sync::atomic::Ordering::Acquire));
    }
}

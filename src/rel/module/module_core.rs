// C++ Original code
// - https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/REL/Module.h
// - load_segments, clear: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/REL/Module.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Get the memory range of the exe or dll module that the current process is loading and collect the addresses of each segment

use super::module_handle::{ModuleHandle, ModuleHandleError};
use super::runtime::Runtime;
use super::segment::{Segment, SegmentName};
use crate::rel::version::{get_file_version, FileVersionError, Version};
use snafu::ResultExt as _;
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
    pub base: ModuleHandle,
    /// Runtime type of the module.
    pub runtime: Runtime,
}

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
        windows::core::h!("SkyrimSE.exe"),
        windows::core::h!("SkyrimVR.exe"),
    ];

    #[cfg(test)]
    fn new(filename: windows::core::HSTRING) -> Result<Self, ModuleInitError> {
        let module_handle = ModuleHandle::new(&filename)
            .map_err(|_| ModuleInitError::ModuleNameAndHandleNotFound)?;

        Self::init_inner(filename, module_handle)
    }

    /// Initializes a new `Module` instance by detecting the currently loaded module.
    ///
    /// This method attempts to retrieve the module information from the `SKSE_RUNTIME`
    /// or fallback to a predefined list of runtime binaries(e.g. `SkyrimSE.exe`).
    ///
    /// # Examples
    /// ```no_run
    /// use commonlibsse_ng::rel::module::{Module, Runtime};
    ///
    /// let module = Module::from_skyrim();
    /// match module {
    ///     Ok(module) => {
    ///         assert!(!module.filename.is_empty());
    ///         assert!(!module.file_path.is_empty());
    ///         assert_eq!(module.runtime, Runtime::Se);
    ///     }
    ///     Err(err) => panic!("Failed to initialize module: {err}"),
    /// }
    /// ```
    ///
    /// # Errors
    /// An error occurs in the following cases
    /// - If the module handle could not be obtained.
    /// - Module version could not be obtained.
    pub fn from_skyrim() -> Result<Self, ModuleInitError> {
        use windows::core::{h, HSTRING};
        use windows::Win32::System::Environment::GetEnvironmentVariableW;

        #[inline]
        fn get_module_name_from_skse() -> Option<(HSTRING, ModuleHandle)> {
            let mut filename = vec![0; windows::Win32::Foundation::MAX_PATH as usize];
            let filename_len =
                unsafe { GetEnvironmentVariableW(h!("SKSE_RUNTIME"), Some(&mut filename)) }
                    as usize;

            let is_failed = filename_len != filename.len() - 1 || filename_len == 0;
            if is_failed {
                return None;
            }

            let filename = HSTRING::from_wide(&filename);
            let new_handle = ModuleHandle::new(&filename).ok()?;
            Some((filename, new_handle))
        }

        #[inline]
        fn get_module_handle_from_runtime() -> Option<(HSTRING, ModuleHandle)> {
            #[cfg(feature = "tracing")]
            tracing::info!("Failed to read the `SKSE_RUNTIME` environment variable. Trying to get it from Runtime exe (e.g. `SkyrimSE.exe`) instead...");

            let mut ret = None;
            for runtime_name in Module::RUNTIMES {
                if let Ok(new_handle) = ModuleHandle::new(runtime_name) {
                    ret = Some((runtime_name.clone(), new_handle));
                    break;
                }
            }

            ret
        }

        let (filename, module_handle) = get_module_name_from_skse()
            .or_else(get_module_handle_from_runtime)
            .ok_or(ModuleInitError::ModuleNameAndHandleNotFound)?;

        Self::init_inner(filename, module_handle)
    }

    #[inline]
    fn init_inner(
        filename: windows::core::HSTRING,
        module_handle: ModuleHandle,
    ) -> Result<Self, ModuleInitError> {
        let segments = Self::load_segments(&module_handle).context(SegmentLoadFailedSnafu)?;
        let (version, runtime) = Self::load_version(&filename).context(VersionLoadFailedSnafu)?;
        let file_path = filename.to_string();

        Ok(Self {
            filename,
            file_path,
            segments,
            version,
            base: module_handle,
            runtime,
        })
    }

    /// Gets a specific memory segment by [`SegmentName`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use commonlibsse_ng::rel::module::{Module, SegmentName};
    ///
    /// match Module::from_skyrim() {
    ///     Ok(module) => println!("{:?}", module.segment(SegmentName::Textx)),
    ///     Err(err) => tracing::error!("Failed to initialize module: {err}"),
    /// }
    /// ```
    #[inline]
    pub const fn segment(&self, name: SegmentName) -> Segment {
        self.segments[name as usize]
    }

    #[inline]
    fn load_segments(module_handle: &ModuleHandle) -> Result<[Segment; 8], ModuleHandleError> {
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

        let mut segments = [Segment::const_default(); 8];
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
    fn load_version(
        file_path: &windows::core::HSTRING,
    ) -> Result<(Version, Runtime), FileVersionError> {
        let version = get_file_version(file_path)?;
        let runtime = Runtime::from_version(&version);
        Ok((version, runtime))
    }
}

/// Errors that can occur during module initialization.
#[derive(Debug, Clone, snafu::Snafu, PartialEq, Eq)]
pub enum ModuleInitError {
    /// SKSE or Skyrim exe does not exist or is not loaded into the current process.
    ModuleNameAndHandleNotFound,
    /// Module handle operation failed during segment search -> {source}
    SegmentLoadFailed {
        source: crate::rel::module::ModuleHandleError,
    },

    /// Failed to load version information. -> {source}
    #[snafu(display("Failed to load module version"))]
    VersionLoadFailed {
        source: crate::rel::version::FileVersionError,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_init() {
        // Use `msvcrt.dll` for testing since the dll is always US English and
        // always loaded in the msvc target when the test is run.
        let filename = windows::core::h!("msvcrt.dll");

        match dbg!(Module::new(filename.clone())) {
            Ok(module) => {
                assert!(!module.file_path.is_empty());
                assert!(!module.filename.is_empty());
                assert_eq!(module.runtime, Runtime::Se);
            }
            Err(err) => panic!("Failed to initialize module: {err}"),
        }
    }
}

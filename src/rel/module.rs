// C++ Original code
// - https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/REL/Module.h
// - load_segments, clear: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/REL/Module.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::rel::version::{get_file_version, FileVersionError, Version};
use std::sync::{atomic::Ordering, LazyLock, LockResult, RwLock, RwLockWriteGuard};
use windows::core::HSTRING;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Segment {
    proxy_base: usize,
    address: usize,
    size: usize,
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
    pub const fn new(proxy_base: usize, address: usize, size: usize) -> Self {
        Self {
            proxy_base,
            address,
            size,
        }
    }

    #[inline]
    pub const fn address(&self) -> usize {
        self.address
    }

    #[inline]
    pub const fn offset(&self) -> usize {
        self.address - self.proxy_base
    }

    #[inline]
    pub const fn size(&self) -> usize {
        self.size
    }

    #[inline]
    pub const fn pointer(&self) -> *mut std::ffi::c_void {
        self.address as *mut std::ffi::c_void
    }

    #[inline]
    pub const fn pointer_as<T>(&self) -> *mut T {
        self.pointer() as *mut T
    }
}

pub struct Module {
    pub filename: windows::core::HSTRING,
    pub file_path: String,
    segments: [Segment; 8],
    pub version: Version,
    /// void pointer
    pub base: usize,
    pub runtime: Runtime,
}

static INITIALIZED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static MODULE: LazyLock<RwLock<Option<Module>>> =
    LazyLock::new(|| RwLock::new(Some(Module::init())));

impl Module {
    const ENVIRONMENT_W: &windows::core::HSTRING = windows::core::h!("SKSE_RUNTIME");

    /// TODO: phf table or mut HashMap?
    #[allow(unused)]
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

    /// Get singleton instance.
    pub fn get_mut() -> LockResult<RwLockWriteGuard<'static, Option<Module>>> {
        if INITIALIZED.load(Ordering::Relaxed) {
            MODULE.write()
        } else {
            if let Ok(mut guard) = MODULE.try_write() {
                let _ = guard.replace(Self::init());
            };
            INITIALIZED.store(true, Ordering::Relaxed);

            MODULE.write()
        }
    }

    pub fn init() -> Self {
        use windows::core::HSTRING;
        use windows::Win32::System::Environment::GetEnvironmentVariableW;
        use windows::Win32::System::LibraryLoader::GetModuleHandleW;

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
                if let Ok(new_handle) = unsafe { GetModuleHandleW(runtime) } {
                    filename = runtime.clone();
                    module_handle = Some(new_handle);
                    break;
                }
            }
        };

        let file_path = filename.to_string();
        let (version, runtime) = if let Some(module_handle) = module_handle {
            match Self::load(module_handle, &file_path) {
                Ok((version, runtime)) => (Some(version), Some(runtime)),
                Err(_) => (None, None),
            }
        } else {
            (None, None)
        };

        Self {
            filename,
            file_path,
            segments: [Segment::new(0, 0, 0); 8],
            version: version.unwrap_or(Version::const_default()),
            base: module_handle.unwrap_or_default().0 as usize,
            runtime: runtime.unwrap_or_default(),
        }
    }

    #[inline]
    pub fn reset() {
        INITIALIZED.store(false, std::sync::atomic::Ordering::Relaxed);
        if let Err(err) = MODULE.write().map(|mut instance| {
            *instance = None;
        }) {
            #[cfg(feature = "tracing")]
            tracing::error!("Couldn't clear MODULE instance.{err}");
        };
    }

    #[inline]
    pub const fn segment(&self, segment: usize) -> &Segment {
        &self.segments[segment]
    }

    /// # Safety
    ///
    #[inline]
    pub const unsafe fn pointer<T>(&self) -> *mut T {
        self.base as *mut T
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

    #[inline]
    fn load(
        a_handle: windows::Win32::Foundation::HMODULE,
        file_path: &str,
    ) -> Result<(Version, Runtime), FileVersionError> {
        Self::load_segments(a_handle);
        let (version, runtime) = Self::load_version(&file_path)?;

        Ok((version, runtime))
    }

    fn load_segments(a_handle: windows::Win32::Foundation::HMODULE) {
        let _ = a_handle;
        todo!()
        //     let dos_header = a_handle.0 as *const DosHeader;
        //     let nt_header = unsafe {
        //         self.adjust_pointer::<NtHeaders64>(dos_header, unsafe { (*dos_header).lfanew } as usize)
        //     };
        //     let sections = unsafe { (*nt_header).first_section() };
        //     let size = std::cmp::min(
        //         unsafe { (*nt_header).file_header.section_count } as usize,
        //         self.segments.len(),
        //     );

        //     for i in 0..size {
        //         let section = unsafe { sections.add(i) };
        //         if let Some((idx, _)) = Self::SEGMENTS.iter().enumerate().find(|(_, elem)| {
        //             let len = cmp::min(elem.0.len(), section_name_size());
        //             unsafe { std::ptr::read_volatile(section) }.matches(elem.0, len, elem.1)
        //         }) {
        //             self.segments[idx] = Segment::new(
        //                 self.base,
        //                 self.base + unsafe { (*section).virtual_address } as usize,
        //                 unsafe { (*section).virtual_size } as usize,
        //             );
        //         }
        //     }
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
        //     unsafe { free_library(module) };
        //     self.injected_module = None;
        // }
        self.base = 0;
        self.filename = HSTRING::default();
        self.file_path.clear();
        self.runtime = Runtime::Ae;
        self.version = Version::const_default();
        self.segments = [Segment::new(0, 0, 0); 8];
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

// C++ Original code
// - https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/REL/Module.h
// - load_segments, clear: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/REL/Module.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

use snafu::ResultExt as _;

/// A memory-mapped file and its view.
#[derive(Debug, Clone)]
pub struct MemoryMap {
    /// Mapping handle
    mapping: windows::Win32::Foundation::HANDLE,
    /// Memory mapped view address
    pub view: windows::Win32::System::Memory::MEMORY_MAPPED_VIEW_ADDRESS,
}

impl MemoryMap {
    /// FILE_MAP_READ(4) | FILE_MAP_WRITE(2) -> 0b110 -> 6
    /// - `use windows::Win32::System::Memory::{FILE_MAP_READ, FILE_MAP_WRITE};`
    const ALLOW_READ_WRITE: windows::Win32::System::Memory::FILE_MAP =
        windows::Win32::System::Memory::FILE_MAP(0b110);

    /// Creates a new `MemoryMap` at compile time.
    ///
    /// This function creates an instance of `MemoryMap` with default uninitialized values.
    ///
    /// # Example
    /// ```
    /// use commonlibsse_ng::rel::id::memory_map::MemoryMap;
    ///
    /// let memory_map = MemoryMap::new();
    /// ```
    pub const fn new() -> Self {
        Self {
            mapping: windows::Win32::Foundation::HANDLE(core::ptr::null_mut()),
            view: windows::Win32::System::Memory::MEMORY_MAPPED_VIEW_ADDRESS {
                Value: core::ptr::null_mut(),
            },
        }
    }

    /// Opens an existing memory-mapped file with the specified name and size.
    ///
    /// # Errors
    /// This method attempts to open a memory-mapped file. If the file does not exist, it will fail.
    ///
    /// # Example
    /// ```no_run
    /// use commonlibsse_ng::rel::id::memory_map::MemoryMap;
    /// use windows::core::h; // `h!` is utf-16 str macro.
    ///
    /// let mut memory_map = MemoryMap::new();
    /// memory_map.open(h!("example_mapping"), 1024).expect("Failed to open memory map");
    /// ```
    pub fn open<H>(&mut self, name: H, size: usize) -> Result<(), MemoryMapError>
    where
        H: windows::core::Param<windows::core::PCWSTR>,
    {
        use windows::Win32::System::Memory::{MapViewOfFile, OpenFileMappingW};

        self.close()?;

        // OpenFileMappingW: https://learn.microsoft.com/windows/win32/api/memoryapi/nf-memoryapi-openfilemappingw
        let handle = match unsafe { OpenFileMappingW(Self::ALLOW_READ_WRITE.0, false, name) } {
            Ok(handle) => {
                self.mapping = handle;
                handle
            }
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::error!("Failed to open mapping: {err}");
                self.close()?;
                return Err(MemoryMapError::OpenMapping { source: err });
            }
        };

        // MapViewOfFile: https://learn.microsoft.com/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile
        let view_address = unsafe { MapViewOfFile(handle, Self::ALLOW_READ_WRITE, 0, 0, size) };
        if view_address.Value.is_null() {
            self.close()?;
            return Err(MemoryMapError::MapView);
        } else {
            self.view = view_address;
        };

        Ok(())
    }

    /// Creates a new memory-mapped file if one does not exist.
    ///
    /// # Errors
    /// If failed to create the memory-mapped region.
    ///
    /// # Example
    /// ```no_run
    /// use commonlibsse_ng::rel::id::memory_map::MemoryMap;
    /// use windows::core::h; // `h!` is utf-16 str macro.
    ///
    /// let mut memory_map = MemoryMap::new();
    /// memory_map.create(h!("new_mapping"), 2048).expect("Failed to create memory map");
    /// ```
    pub fn create(
        &mut self,
        name: &windows::core::HSTRING,
        size: usize,
    ) -> Result<(), MemoryMapError> {
        use windows::Win32::Foundation::INVALID_HANDLE_VALUE;
        use windows::Win32::System::Memory::{
            CreateFileMappingW, MapViewOfFile, OpenFileMappingW, PAGE_READWRITE,
        };

        self.close()?;

        // OpenFileMappingW: https://learn.microsoft.com/windows/win32/api/memoryapi/nf-memoryapi-openfilemappingw
        let handle = match unsafe { OpenFileMappingW(Self::ALLOW_READ_WRITE.0, false, name) } {
            Ok(handle) => {
                self.mapping = handle;
                handle
            }
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::warn!("Failed to open mapping -> Fallback to create: {err}");

                let (max, min) = ((size >> 32) as u32, size as u32); // Split to high, low

                // CreateFileMappingW: https://learn.microsoft.com/windows/win32/api/memoryapi/nf-memoryapi-createfilemappingw
                match unsafe {
                    CreateFileMappingW(INVALID_HANDLE_VALUE, None, PAGE_READWRITE, max, min, name)
                } {
                    Ok(handle) => {
                        self.mapping = handle;
                        handle
                    }
                    Err(err) => {
                        #[cfg(feature = "tracing")]
                        tracing::error!("Failed to create file mapping: {err}");
                        return Err(MemoryMapError::CreateMapping { source: err });
                    }
                }
            }
        };

        // MapViewOfFile: https://learn.microsoft.com/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile
        let view_address = unsafe { MapViewOfFile(handle, Self::ALLOW_READ_WRITE, 0, 0, size) };
        if view_address.Value.is_null() {
            self.close()?;
            return Err(MemoryMapError::MapView);
        } else {
            self.view = view_address;
        };

        Ok(())
    }

    /// Closes the memory-mapped file view and releases the associated handle.
    ///
    /// # Errors
    /// This function may return an error if either `UnmapViewOfFile` or `CloseHandle` fails. Errors
    /// may occur if the underlying system fails to unmap the view or close the handle, such as invalid
    /// memory addresses or handle issues.
    ///
    /// # Example
    /// ```no_run
    /// use commonlibsse_ng::rel::id::memory_map::MemoryMap;
    ///
    /// let mut my_mapping = MemoryMap::new();
    /// my_mapping.close().expect("Failed to close the memory mapping");
    /// ```
    pub fn close(&mut self) -> Result<(), MemoryMapError> {
        use windows::Win32::System::Memory::UnmapViewOfFile;

        {
            // UnmapViewOfFile: https://learn.microsoft.com/windows/win32/api/memoryapi/nf-memoryapi-unmapviewoffile
            unsafe { UnmapViewOfFile(self.view) }.with_context(|_| UnmapViewSnafu)?;
            self.view = windows::Win32::System::Memory::MEMORY_MAPPED_VIEW_ADDRESS {
                Value: core::ptr::null_mut(),
            };
        }

        {
            // CloseHandle: https://learn.microsoft.com/windows/win32/api/handleapi/nf-handleapi-closehandle
            unsafe { windows::Win32::Foundation::CloseHandle(self.mapping) }
                .with_context(|_| CloseHandleSnafu)?;
            self.mapping = windows::Win32::Foundation::HANDLE(core::ptr::null_mut());
        }

        Ok(())
    }
}

impl Default for MemoryMap {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for MemoryMap {
    fn drop(&mut self) {
        if let Err(e) = self.close() {
            #[cfg(feature = "tracing")]
            tracing::error!("Failed to close memory map during drop: {e}");
        }
    }
}

/// Errors that may occur during operations on `MemoryMap`.
#[derive(Debug, snafu::Snafu)]
pub enum MemoryMapError {
    /// Failed to open memory mapping: {source}
    OpenMapping { source: windows::core::Error },

    /// Failed to create memory mapping: {source}
    CreateMapping { source: windows::core::Error },

    /// Failed to map view of file.
    MapView,

    /// Failed to unmap memory view: {source}
    UnmapView { source: windows::core::Error },

    /// Failed to close handle: {source}
    CloseHandle { source: windows::core::Error },
}

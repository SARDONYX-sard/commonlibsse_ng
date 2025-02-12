// C++ Original code
// - open, create, close: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/REL/ID.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Memory-mapped file handling module.
//!
//! This module provides a safe wrapper around Windows memory-mapped files,
//! allowing users to open and manipulate shared memory regions.
//!
//! This is the code to realize the data sharing of `AddressLibrary`.
//!
//! The intention is to avoid wasteful use of memory by referencing the same database.

use super::MemoryMapError;
use windows::{
    core::HSTRING,
    Win32::{Foundation::HANDLE, System::Memory::MEMORY_MAPPED_VIEW_ADDRESS},
};

pub fn open(
    name: &HSTRING,
    size: usize,
) -> Result<(HANDLE, MEMORY_MAPPED_VIEW_ADDRESS), MemoryMapError> {
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::System::Memory::{
        MapViewOfFile, OpenFileMappingW, FILE_MAP_READ, FILE_MAP_WRITE,
    };

    let handle = unsafe { OpenFileMappingW((FILE_MAP_READ | FILE_MAP_WRITE).0, false, name) }
        .map_err(|e| MemoryMapError::OpenMapping { source: e })?;

    // MapViewOfFile: https://learn.microsoft.com/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile
    let view_address = unsafe { MapViewOfFile(handle, FILE_MAP_READ | FILE_MAP_WRITE, 0, 0, size) };
    if view_address.Value.is_null() {
        let _ = unsafe { CloseHandle(handle) };
        return Err(MemoryMapError::MapView);
    }

    Ok((handle, view_address))
}

pub fn create(
    name: &windows::core::HSTRING,
    size: usize,
) -> Result<(HANDLE, MEMORY_MAPPED_VIEW_ADDRESS), MemoryMapError> {
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::Foundation::INVALID_HANDLE_VALUE;
    use windows::Win32::System::Memory::{
        CreateFileMappingW, MapViewOfFile, FILE_MAP_READ, FILE_MAP_WRITE, PAGE_READWRITE,
    };

    // CreateFileMappingW: https://learn.microsoft.com/windows/win32/api/memoryapi/nf-memoryapi-createfilemappingw

    let handle = unsafe {
        let (max, min) = ((size >> 32) as u32, size as u32); // Split to high, low
        CreateFileMappingW(INVALID_HANDLE_VALUE, None, PAGE_READWRITE, max, min, name)
    }
    .map_err(|e| MemoryMapError::CreateMapping { source: e })?;

    let view = {
        let view_address =
            unsafe { MapViewOfFile(handle, FILE_MAP_READ | FILE_MAP_WRITE, 0, 0, size) };

        if view_address.Value.is_null() {
            // CloseHandle: https://learn.microsoft.com/windows/win32/api/handleapi/nf-handleapi-closehandle
            let _ = unsafe { CloseHandle(handle) };
            return Err(MemoryMapError::MapView);
        }

        view_address
    };

    Ok((handle, view))
}

pub fn close(handle: HANDLE, view: *mut core::ffi::c_void) -> Result<(), MemoryMapError> {
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::System::Memory::UnmapViewOfFile;

    let view = MEMORY_MAPPED_VIEW_ADDRESS { Value: view };

    // view is NonNull
    unsafe { UnmapViewOfFile(view) }.map_err(|e| MemoryMapError::UnmapView { source: e })?;

    unsafe { CloseHandle(handle) }.map_err(|e| MemoryMapError::CloseHandle { source: e })?;

    Ok(())
}

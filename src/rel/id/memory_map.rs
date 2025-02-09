// C++ Original code
// - https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/REL/ID.h
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
//!
//! # Thread safety
//! The internal Windows API is used to perform locking at the kernel level, so `as_slice_mut` and others are lock-free.

use crate::rel::id::Mapping;
use std::num::NonZeroUsize;
use std::ptr::NonNull;

/// Represents a memory-mapped file in Windows.
///
/// This struct manages the creation and lifetime of a memory-mapped file view.
/// It ensures that resources are properly released when dropped.
///
/// # Thread safety
/// There were concerns about locks for inter-process shared references with MapViewOfFile, but it seems that kernel-level locks are in place.
/// In other words, there will be no conflicts when concurrently writing to the allocated memory.
///
/// We tested it on a 400_000 array and it certainly did not cause inconsistencies.
///
// source: https://devblogs.microsoft.com/oldnewthing/20210702-00/?p=105392
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemoryMap {
    /// Non-null file mapping handle
    mapping: NonZeroUsize,
    /// Non-null pointer to mapped memory
    view: NonNull<u8>,
    /// Size of the mapped region
    size: usize,
}

impl MemoryMap {
    /// Attempts to open  an existing memory-mapped file by its name.
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    ///
    /// - The mapping cannot be opened (`OpenMapping` error).
    /// - The file view cannot be mapped (`MapView` error).
    ///
    /// # Examples
    ///
    /// ```
    /// use commonlibsse_ng::rel::id::memory_map::MemoryMap;
    /// use windows::core::h; // `h!` is utf-16 string macro
    ///
    /// // It is expected to be made before it opens.
    /// let pre_alloc_map = MemoryMap::create(h!("example_mapping"), 2048).expect("Failed to create memory map");
    ///
    /// let memory_map = MemoryMap::open(h!("example_mapping"), 2048).expect("Failed to open memory map");
    /// ```
    pub fn open(name: &windows::core::HSTRING, size: usize) -> Result<Self, MemoryMapError> {
        use windows::Win32::Foundation::CloseHandle;
        use windows::Win32::System::Memory::{
            MapViewOfFile, OpenFileMappingW, FILE_MAP_READ, FILE_MAP_WRITE,
        };

        let handle = unsafe { OpenFileMappingW((FILE_MAP_READ | FILE_MAP_WRITE).0, false, name) }
            .map_err(|e| MemoryMapError::OpenMapping { source: e })?;

        let handle_usize = handle.0 as usize;
        let handle_nonzero =
            NonZeroUsize::new(handle_usize).ok_or_else(|| MemoryMapError::OpenMapping {
                source: windows::core::Error::from_win32(),
            })?;

        // MapViewOfFile: https://learn.microsoft.com/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile
        let view_address =
            unsafe { MapViewOfFile(handle, FILE_MAP_READ | FILE_MAP_WRITE, 0, 0, size) };
        if view_address.Value.is_null() {
            let _ = unsafe { CloseHandle(handle) };
            return Err(MemoryMapError::MapView);
        }

        Ok(Self {
            mapping: handle_nonzero,
            view: NonNull::new(view_address.Value.cast()).ok_or(MemoryMapError::MapView)?,
            size,
        })
    }

    /// Creates a new memory-mapped file if one does not exist.
    ///
    /// # Errors
    /// Returns an error if the memory-mapped file cannot be created or mapped.
    ///
    /// # Examples
    /// ```
    /// use commonlibsse_ng::rel::id::memory_map::MemoryMap;
    /// use windows::core::h; // utf-16 macro
    ///
    /// let memory_map = MemoryMap::create(h!("new_mapping"), 2048).expect("Failed to create memory map");
    /// ```
    pub fn create(name: &windows::core::HSTRING, size: usize) -> Result<Self, MemoryMapError> {
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

        let handle_nonzero = {
            let handle_usize = handle.0 as usize;
            NonZeroUsize::new(handle_usize).ok_or_else(|| MemoryMapError::CreateMapping {
                source: windows::core::Error::from_win32(),
            })?
        };

        let view = {
            let view_address =
                unsafe { MapViewOfFile(handle, FILE_MAP_READ | FILE_MAP_WRITE, 0, 0, size) };

            if view_address.Value.is_null() {
                // CloseHandle: https://learn.microsoft.com/windows/win32/api/handleapi/nf-handleapi-closehandle
                let _ = unsafe { CloseHandle(handle) };
                return Err(MemoryMapError::MapView);
            }

            NonNull::new(view_address.Value.cast()).ok_or(MemoryMapError::MapView)?
        };

        Ok(Self {
            mapping: handle_nonzero,
            view,
            size,
        })
    }

    /// Unmaps the file view and closes the file mapping handle.
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    ///
    /// - `UnmapViewOfFile` fails (`UnmapView` error).
    /// - `CloseHandle` fails (`CloseHandle` error).
    ///
    /// When call [`Drop::drop`]` with `Self`, then called close.
    fn close(&self) -> Result<(), MemoryMapError> {
        use windows::Win32::Foundation::{CloseHandle, HANDLE};
        use windows::Win32::System::Memory::{UnmapViewOfFile, MEMORY_MAPPED_VIEW_ADDRESS};

        let view = MEMORY_MAPPED_VIEW_ADDRESS {
            Value: self.view.as_ptr().cast(),
        };

        // view is NonNull
        unsafe { UnmapViewOfFile(view) }.map_err(|e| MemoryMapError::UnmapView { source: e })?;

        if self.mapping.get() != 0 {
            let handle = HANDLE(self.mapping.get() as *mut core::ffi::c_void);
            unsafe { CloseHandle(handle) }
                .map_err(|e| MemoryMapError::CloseHandle { source: e })?;
        }

        Ok(())
    }

    /// Returns the unique ID of the memory-mapped file.
    ///
    /// The ID is derived from the file mapping handle (`HANDLE`).
    ///
    /// # Examples
    ///
    /// ```
    /// use commonlibsse_ng::rel::id::memory_map::MemoryMap;
    /// use windows::core::h; // utf-16 macro
    ///
    /// let memory_map = MemoryMap::create(h!("example_mapping"), 1024).expect("Failed to open");
    /// println!("Memory Map ID: {}", memory_map.id());
    /// ```
    pub const fn id(&self) -> u64 {
        self.mapping.get() as u64
    }

    /// Returns a reference to the underlying file view as a slice.
    ///
    /// # Examples
    /// ```
    /// use commonlibsse_ng::rel::id::memory_map::MemoryMap;
    /// use windows::core::h; // utf-16 macro
    ///
    /// let memory_map = MemoryMap::create(h!("example_mapping"), 1024).expect("Failed to create");
    /// assert_eq!(memory_map.as_slice(), [0_u8; 1024]);
    /// ```
    pub fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts_mut(self.view.as_ptr(), self.size) }
    }

    /// Returns a mutable reference to the underlying file view as a slice.
    ///
    /// # Examples
    /// ```
    /// use commonlibsse_ng::rel::id::memory_map::MemoryMap;
    /// use windows::core::h; // utf-16 macro
    ///
    /// let mut memory_map = MemoryMap::create(h!("example_mapping"), 1024).expect("Failed to create");
    /// let mem_mut = memory_map.as_slice_mut();
    /// mem_mut[0] = 1;
    /// assert_eq!(mem_mut[0], 1);
    /// ```
    ///
    /// # Thread Safety
    /// There were concerns about locks for inter-process shared references with `MapViewOfFile`, but it seems that kernel-level locks are in place.
    // In other words, there will be no conflicts when concurrently writing to the allocated memory.
    // source: https://devblogs.microsoft.com/oldnewthing/20210702-00/?p=105392
    #[allow(clippy::mut_from_ref)]
    pub fn as_slice_mut(&self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.view.as_ptr(), self.size) }
    }

    /// Attempts to cast the memory region to a slice of `Mapping` structs.
    ///
    /// # Errors
    /// Returns an error if:
    /// - `self.size` is zero (`ZeroSize` error).
    /// - `self.size` is smaller than the size of a single `Mapping` struct (`InsufficientSize` error).
    /// - `self.size` is not a multiple of the size of a `Mapping` struct (`NonMultipleSize` error).
    ///
    /// # Note
    /// We can't get correct data just by calling this.
    /// We need to read the `AddressLibrary` for the mappings data and plug in the bit-operated data as per the specifications.(like `IdDatabase::unpack`)
    ///
    /// # Examples
    /// ```
    /// use commonlibsse_ng::rel::id::{MemoryMap, Mapping};
    /// use windows::core::h; // utf-16 macro
    ///
    /// // Create a dummy MemoryMap with a size that is a multiple of Mapping's size
    /// let memory_map = {
    ///     let memory_map = MemoryMap::create(h!("test"), 2 * size_of::<Mapping>())
    ///         .expect("Failed to create memory map");
    ///
    ///     // Step 1: Write to memory using `as_slice_mut` to inject dummy data
    ///     let slice_mut = memory_map.as_slice_mut();
    ///
    ///     // Write the first Mapping (id: 42, offset: 100)
    ///     let mapping1_id = 42_u64.to_le_bytes();
    ///     let mapping1_offset = 100_u64.to_le_bytes();
    ///
    ///     slice_mut[0..8].copy_from_slice(&mapping1_id);
    ///     slice_mut[8..16].copy_from_slice(&mapping1_offset);
    ///
    ///     // Write the second Mapping (id: 84, offset: 200)
    ///     let mapping2_id = 84_u64.to_le_bytes();
    ///     let mapping2_offset = 200_u64.to_le_bytes();
    ///
    ///     slice_mut[16..24].copy_from_slice(&mapping2_id);
    ///     slice_mut[24..32].copy_from_slice(&mapping2_offset);
    ///     memory_map
    /// };
    ///
    /// // Step 2: Cast the written data into a slice of `Mapping` structs
    /// let mappings = memory_map
    ///     .as_mapping_slice()
    ///     .expect("Failed to cast to slice");
    ///
    /// // Step 3: Assert the values are set correctly
    /// assert_eq!(mappings[0].id, 42);
    /// assert_eq!(mappings[0].offset, 100);
    /// assert_eq!(mappings[1].id, 84);
    /// assert_eq!(mappings[1].offset, 200);
    /// ```
    pub const fn as_mapping_slice<'a>(&'a self) -> Result<&'a [Mapping], MemoryMapCastError> {
        // Check if the memory size is zero
        if self.size == 0 {
            return Err(MemoryMapCastError::ZeroSize);
        }

        // Check if the memory size is smaller than the size of one Mapping struct
        if self.size < SIZE_OF_MAPPING {
            return Err(MemoryMapCastError::InsufficientSize { actual: self.size });
        }

        // Ensure the memory map size is a multiple of the size of Mapping
        if self.size % SIZE_OF_MAPPING != 0 {
            return Err(MemoryMapCastError::NonMultipleSize {
                allocated_size: self.size,
            });
        }

        // Convert the raw pointer into a slice of Mappings
        let num_mappings = self.size / SIZE_OF_MAPPING;
        let mappings_slice: &'a [Mapping] = unsafe {
            core::slice::from_raw_parts(self.view.as_ptr() as *const Mapping, num_mappings)
        };

        Ok(mappings_slice)
    }

    /// Attempts to cast the memory region to a mutable slice of `Mapping` structs.
    ///
    /// # Errors
    /// Returns an error if:
    /// - `self.size` is zero (`ZeroSize` error).
    /// - `self.size` is smaller than the size of a single `Mapping` struct (`InsufficientSize` error).
    /// - `self.size` is not a multiple of the size of a `Mapping` struct (`NonMultipleSize` error).
    ///
    /// # Examples
    /// ```
    /// use commonlibsse_ng::rel::id::{MemoryMap, Mapping};
    /// use windows::core::h; // utf-16 macro
    ///
    /// // Step 1: Create a dummy MemoryMap with a size that is a multiple of Mapping's size
    /// let memory_map = MemoryMap::create(h!("test"), 2 * size_of::<Mapping>())
    ///     .expect("Failed to create memory map");
    ///
    /// // Step 2: Cast the written data into a slice of `Mapping` structs
    /// let mappings = memory_map
    ///     .as_mapping_slice_mut()
    ///     .expect("Failed to cast to slice");
    ///
    /// let mappings_data = [
    ///     Mapping {
    ///         id: 42,
    ///         offset: 100,
    ///     },
    ///     Mapping {
    ///         id: 84,
    ///         offset: 200,
    ///     },
    /// ];
    ///
    /// for (target, mapping) in mappings.iter_mut().zip(mappings_data) {
    ///     *target = mapping;
    /// }
    ///
    /// // Step 3: Assert the values are set correctly
    /// assert_eq!(mappings[0].id, 42);
    /// assert_eq!(mappings[0].offset, 100);
    /// assert_eq!(mappings[1].id, 84);
    /// assert_eq!(mappings[1].offset, 200);
    /// ```
    pub fn as_mapping_slice_mut<'a>(&'a self) -> Result<&'a mut [Mapping], MemoryMapCastError> {
        // Check if the memory size is zero
        if self.size == 0 {
            return Err(MemoryMapCastError::ZeroSize);
        }

        // Check if the memory size is smaller than the size of one Mapping struct
        if self.size < SIZE_OF_MAPPING {
            return Err(MemoryMapCastError::InsufficientSize { actual: self.size });
        }

        // Ensure the memory map size is a multiple of the size of Mapping
        if self.size % SIZE_OF_MAPPING != 0 {
            return Err(MemoryMapCastError::NonMultipleSize {
                allocated_size: self.size,
            });
        }

        // Convert the raw pointer into a slice of Mappings
        let num_mappings = self.size / SIZE_OF_MAPPING;
        let mappings_slice: &'a mut [Mapping] = unsafe {
            core::slice::from_raw_parts_mut(self.view.as_ptr().cast::<Mapping>(), num_mappings)
        };

        Ok(mappings_slice)
    }
}

const SIZE_OF_MAPPING: usize = size_of::<Mapping>();

impl Drop for MemoryMap {
    /// Ensures that the memory-mapped file is properly closed on drop.
    fn drop(&mut self) {
        let _ = self.close();
    }
}

// Thread safe sharing memory
// There were concerns about locks for inter-process shared references with `MapViewOfFile`, but it seems that kernel-level locks are in place.
// In other words, there will be no conflicts when concurrently writing to the allocated memory.
// source: https://devblogs.microsoft.com/oldnewthing/20210702-00/?p=105392
unsafe impl Send for MemoryMap {}
unsafe impl Sync for MemoryMap {}

/// Implements conversion from `MemoryMap` to `Mapping`.
///
/// This allows `MemoryMap` to be converted into a lightweight `Mapping` representation.
impl From<&MemoryMap> for Mapping {
    fn from(map: &MemoryMap) -> Self {
        Self {
            id: map.id(),
            offset: map.view.as_ptr() as usize as u64, // Store the pointer address as the offset
        }
    }
}

/// Defines errors that may occur when working with `MemoryMap`.
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

/// Define errors that may occur when casting memory to `Mapping` structs.
#[derive(Debug, snafu::Snafu)]
pub enum MemoryMapCastError {
    /// Memory size is zero.
    ZeroSize,

    /// Memory size({actual} bytes) is smaller than the size of Mapping struct(8 + 8 bytes)
    InsufficientSize { actual: usize },

    /// Memory region size({allocated_size}) is not a multiple of Mapping struct size(16bytes)
    NonMultipleSize { allocated_size: usize },
}

#[cfg(test)]
#[cfg(target_os = "windows")]
mod tests {
    use super::*;
    use windows::core::{h, HSTRING}; // UTF-16 string macro
    use windows::Win32::Foundation::INVALID_HANDLE_VALUE;
    use windows::Win32::System::Memory::{
        CreateFileMappingW, MapViewOfFile, FILE_MAP_READ, FILE_MAP_WRITE, PAGE_READWRITE,
    };

    /// Helper function to create a test memory-mapped file.
    fn create_test_memory_map(hname: &HSTRING, size: usize) -> MemoryMap {
        // Create a memory-mapped file
        let handle = unsafe {
            CreateFileMappingW(
                INVALID_HANDLE_VALUE,
                None,
                PAGE_READWRITE,
                0,
                size as u32,
                hname,
            )
        }
        .expect("Failed to create file mapping");

        let view_address =
            unsafe { MapViewOfFile(handle, FILE_MAP_READ | FILE_MAP_WRITE, 0, 0, size) };
        assert!(!view_address.Value.is_null(), "Failed to map view of file");

        MemoryMap {
            mapping: NonZeroUsize::new(handle.0 as usize).unwrap(),
            view: NonNull::new(view_address.Value.cast()).unwrap(),
            size,
        }
    }

    /// Test: Open a memory-mapped file and ensure it's valid.
    #[test]
    fn test_memory_map_open() {
        let map = create_test_memory_map(h!("test_map_open"), 1024);
        assert_ne!(map.id(), 0, "Memory map ID should not be zero");
        assert_ne!(
            map.view.as_ptr(),
            std::ptr::null_mut(),
            "View pointer should not be null"
        );
    }

    /// Test: Read and write to the mapped memory region.
    #[test]
    fn test_memory_map_read_write() {
        let map = create_test_memory_map(h!("test_map_rw"), 512);

        // Write test data
        let slice = map.as_slice_mut();
        slice[0] = 42;
        slice[1] = 99;

        // Read back the data
        assert_eq!(map.as_slice()[0], 42, "First byte should be 42");
        assert_eq!(map.as_slice()[1], 99, "Second byte should be 99");
    }

    /// Test: Convert `MemoryMap` to `Mapping`
    #[test]
    fn test_memory_map_to_mapping() {
        let map = create_test_memory_map(h!("test_map_mapping"), 256);
        let mapping: Mapping = (&map).into();

        assert_eq!(mapping.id, map.id(), "Mapping ID should match MemoryMap ID");
        assert_eq!(
            mapping.offset,
            map.view.as_ptr() as usize as u64,
            "Offset should match view pointer"
        );
    }

    /// Test: Close a memory-mapped file.
    #[test]
    fn test_memory_map_close() {
        let map = create_test_memory_map(h!("test_map_close"), 128);
        assert!(map.close().is_ok(), "Closing memory map should succeed");
    }

    /// Test: Create a new memory-mapped file.
    #[test]
    fn test_memory_map_create() {
        let map =
            MemoryMap::create(h!("new_test_mapping"), 2048).expect("Failed to create memory map");

        // Check if the memory map was created successfully
        assert_ne!(map.id(), 0, "Memory map ID should not be zero");
        assert_ne!(
            map.view.as_ptr(),
            std::ptr::null_mut(),
            "View pointer should not be null"
        );

        // Optionally check if the size matches
        assert_eq!(map.size, 2048, "The size should match the requested size");
    }

    /// Test: Create and read from the memory-mapped file.
    #[test]
    fn test_memory_map_create_read() {
        let map = MemoryMap::create(h!("new_test_read_mapping"), 1024)
            .expect("Failed to create memory map");

        // Write test data
        let slice = map.as_slice_mut();
        slice[0] = 42;
        slice[1] = 99;

        // Read back the data
        assert_eq!(map.as_slice()[0], 42, "First byte should be 42");
        assert_eq!(map.as_slice()[1], 99, "Second byte should be 99");
    }

    /// Test: Create a memory-mapped file and close it.
    #[test]
    fn test_memory_map_create_close() {
        let map = MemoryMap::create(h!("new_test_close_mapping"), 512)
            .expect("Failed to create memory map");

        // Check the memory map before closing
        assert!(map.id() != 0, "Memory map ID should not be zero");

        // Now close the memory map
        assert!(map.close().is_ok(), "Closing memory map should succeed");
    }

    #[test]
    fn test_memory_map_thread_safe() {
        use std::sync::Arc;
        // 50_000 -> test time: 3.91s
        // 400_000 -> test time: 122.70s
        const TEST_MEMORY_LEN: usize = 50_000;

        let map = MemoryMap::create(h!("test_thread_safe_mapping"), TEST_MEMORY_LEN)
            .expect("Failed to create memory map");

        // Arc<MemoryMap> to allow sharing across threads
        let map = Arc::new(map);

        let mut handles = vec![];

        // Spawn multiple threads to read/write the memory map
        for i in 0..TEST_MEMORY_LEN {
            let map_clone = Arc::clone(&map);
            let handle = std::thread::spawn(move || {
                // Access memory map in each thread
                let index = i;

                let slice = map_clone.as_slice_mut(); // mut from ref;
                slice[index] = index as u8;
                // Read back the value
                let result = slice[index];
                assert_eq!(
                    result, index as u8,
                    "Thread {i} failed to write and read correct value",
                );
            });
            handles.push(handle);
        }

        // Wait for all threads to finish
        for handle in handles {
            handle.join().unwrap();
        }

        // Check that the memory map has not been corrupted
        const fn create_expected_array<const N: usize>() -> [u8; N] {
            let mut expected_array = [0; N];
            let mut index = 0;
            while index < N {
                expected_array[index] = index as u8;
                index += 1;
            }
            expected_array
        }
        assert_eq!(
            map.as_slice(),
            create_expected_array::<TEST_MEMORY_LEN>(),
            "First byte should be 0 after multi-threaded writes"
        );
    }
}

// C++ Original code
// - https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/REL/ID.h
// - https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/REL/ID.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT

//! This module provides functionality for loading and managing an ID-to-offset mapping
//! from a binary address library. It is primarily used to resolve function or data
//! addresses based on their ID values.
//!
//! The ID database is loaded from a precompiled binary file that is versioned based
//! on the runtime environment and module state. This ensures compatibility between
//! different versions of the script extender and the game runtime.

use super::byte_reader::{read_le_u16, read_le_u32, read_le_u64, read_u8};
use super::header::Header;
use super::memory_map::MemoryMap;
use super::Mapping;
use crate::rel::version::Version;
use snafu::ResultExt as _;
use std::io::Read;
use std::sync::LazyLock;

/// Global static instance of `IdDatabase` initialized lazily.
/// This ensures the database is only loaded when needed.
pub(super) static ID_DATABASE: LazyLock<IdDatabase> = LazyLock::new(|| IdDatabase::load().unwrap());

/// Represents a database of ID-to-offset mappings loaded from an address library binary file.
pub struct IdDatabase {
    /// Memory-mapped storage of the ID database.
    pub(super) mem_map: MemoryMap,
}

impl IdDatabase {
    /// Retrieves the offset corresponding to the given ID.
    ///
    /// # Errors
    /// Returns an error if the ID is not found in the database.
    pub(super) fn id_to_offset(&self, id: u64) -> Result<usize, DataBaseLoaderError> {
        let slice = self.mem_map.as_mapping_slice()?;
        slice.binary_search_by(|m| m.id.cmp(&id)).map_or_else(
            |_| Err(DataBaseLoaderError::NotFoundId { id }),
            |index| Ok(slice[index].offset as usize),
        )
    }

    /// Loads the ID database from the appropriate binary file based on the module state.
    ///
    /// # Errors
    /// Returns an error if the module state is invalid, the file cannot be read,
    /// or if the data is not properly formatted.
    fn load() -> Result<Self, DataBaseLoaderError> {
        use crate::rel::module::{ModuleState, Runtime};

        let (version, runtime) = ModuleState::map_or_init(|module| {
            let version = module.version.clone();
            let runtime = module.runtime;
            (version, runtime)
        })?;

        let is_ae = runtime == Runtime::Ae;
        let path = {
            let ver_suffix = if is_ae { "lib" } else { "" };
            format!("Data/SKSE/Plugins/version{ver_suffix}-{version}.bin")
        };
        let expected_fmt_ver = if is_ae { 2 } else { 1 }; // Expected AddressLibrary format version. SE/VR: 1, AE: 1

        Self::load_bin_file(&path, version, expected_fmt_ver)
    }

    /// Reads and parses the ID database binary file.
    ///
    /// - `expected_fmt_ver`: Expected AddressLibrary format version. SE/VR: 1, AE: 2
    ///
    /// # Errors
    /// - If the specified path does not exist.
    /// - If the version without bin file mismatches with the runtime
    /// - If parsing of the data in the bin file fails.
    /// - Failure to allocate memory for bin file storage.
    fn load_bin_file(
        path: &str,
        version: Version,
        expected_fmt_ver: u8,
    ) -> Result<Self, DataBaseLoaderError> {
        use std::fs::File;
        use std::io;

        let mut reader = {
            let file = File::open(path).with_context(|_| AddressLibraryNotFoundSnafu {
                path: path.to_string(),
            })?;
            io::BufReader::new(file)
        };

        // Simulate reading header
        let header = Header::from_reader(&mut reader, expected_fmt_ver)?;

        if header.version != version {
            return Err(DataBaseLoaderError::VersionMismatch {
                expected: version,
                actual: header.version,
            });
        }

        let map_name = windows::core::HSTRING::from(format!("CommonLibSSEOffsets-v2-{version}"));
        let byte_size = header.address_count() * size_of::<Mapping>();

        let mem_map = if let Ok(mem_map) = MemoryMap::open(&map_name, byte_size) {
            mem_map
        } else if let Ok(mem_map) = MemoryMap::create(&map_name, byte_size) {
            Self::unpack_file(&mem_map, &mut reader, header.pointer_size())
                .context(FailedUnpackFileSnafu)?;
            mem_map
            // id2offset.sort_by(|a, b| a.id.cmp(&b.id));
        } else {
            return Err(DataBaseLoaderError::MappingCreationFailed);
        };

        Ok(Self { mem_map })
    }

    /// Unpacks the ID database from the binary file and writes it into the memory map.
    ///
    /// # Errors
    /// - If the memory allocated as `MemoryMap` is not consistent as the length of the mapping data array.
    /// - Returns an error if the binary data cannot be properly parsed.
    fn unpack_file<R>(mem_map: &MemoryMap, reader: &mut R, ptr_size: u64) -> Result<(), UnpackError>
    where
        R: Read,
    {
        // TODO: Parse With `winnow` crate, we can know the exact binary position at the time of the error.
        let mut offset: u64;
        let mut prev_id: u64 = 0;
        let mut prev_offset: u64 = 0;

        for mapping in mem_map.as_mapping_slice_mut()? {
            let type_byte = read_u8(reader)?;

            let low = type_byte & 0xF;
            let high = type_byte >> 4;

            let id = match low {
                0 => read_le_u64(reader)?,
                1 => prev_id + 1,
                2 => prev_id + read_u8(reader)? as u64,
                3 => prev_id - read_u8(reader)? as u64,
                4 => prev_id + read_le_u16(reader)? as u64,
                5 => prev_id - read_le_u16(reader)? as u64,
                6 => read_le_u16(reader)? as u64,
                7 => read_le_u32(reader)? as u64,
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Invalid ID",
                    ))?
                }
            };

            let tmp = if (high & 8) != 0 {
                prev_offset / ptr_size
            } else {
                prev_offset
            };

            offset = match high & 7 {
                0 => read_le_u64(reader)?,
                1 => tmp + 1,
                2 => tmp + read_u8(reader)? as u64,
                3 => tmp - read_u8(reader)? as u64,
                4 => tmp + read_le_u16(reader)? as u64,
                5 => tmp - read_le_u16(reader)? as u64,
                6 => read_le_u16(reader)? as u64,
                7 => read_le_u32(reader)? as u64,
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Invalid offset",
                    ))?
                }
            };

            if (high & 8) != 0 {
                offset *= ptr_size;
            }

            *mapping = Mapping { id, offset };
            prev_id = id;
            prev_offset = offset;
        }

        Ok(())
    }
}

/// Errors that can occur during the file loading process.
#[derive(Debug, snafu::Snafu)]
pub enum DataBaseLoaderError {
    /// Failed to find the id within the address library: {id}. This means this script extender plugin is incompatible.,
    #[snafu(display("Failed to find the id within the address library: {id}\nThis means this script extender plugin is incompatible."))]
    NotFoundId { id: u64 },

    /// Version mismatch
    #[snafu(display("Version mismatch: expected {}, got {}", expected, actual))]
    VersionMismatch { expected: Version, actual: Version },

    /// Failed to create shared mapping
    MappingCreationFailed,

    /// Failed to locate an appropriate address library at: {path}, {source}
    AddressLibraryNotFound {
        path: String,
        source: std::io::Error,
    },

    /// Inherited module state(manager) get error.
    #[snafu(transparent)]
    ModuleStateError {
        source: crate::rel::module::ModuleStateError,
    },

    /// Inherited header parsing error.
    #[snafu(transparent)]
    HeaderParseError { source: super::HeaderError },

    /// Inherited memory mapping error.
    #[snafu(transparent)]
    MemoryMapError {
        source: super::memory_map::MemoryMapError,
    },

    /// Inherited memory mapping error.
    #[snafu(transparent)]
    MemoryMapCastError {
        source: super::memory_map::MemoryMapCastSizeError,
    },

    /// Failed to unpack file at: {source}
    FailedUnpackFile { source: UnpackError },
}

#[derive(Debug, snafu::Snafu)]
pub enum UnpackError {
    /// Inherited memory mapping error.
    #[snafu(transparent)]
    MemoryMapCastError {
        source: super::memory_map::MemoryMapCastSizeError,
    },

    #[snafu(transparent)]
    IoError { source: std::io::Error },
}

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

mod bin_loader;
mod byte_reader;
mod header;
mod unpack;

use super::{shared_rwlock::SharedRwLock, Mapping};
use crate::rel::version::Version;
use std::sync::LazyLock;

/// Global static instance of `IdDatabase` initialized lazily.
/// This ensures the database is only loaded when needed.
pub(crate) static ID_DATABASE: LazyLock<IdDatabase> =
    LazyLock::new(|| IdDatabase::from_bin().unwrap()); // TODO: remove unwrap

/// Represents a database of ID-to-offset mappings loaded from an address library binary file.
pub struct IdDatabase {
    /// Memory-mapped storage of the ID database.
    pub(super) mem_map: SharedRwLock<Mapping>,
}

impl IdDatabase {
    /// Loads the ID database from the appropriate binary file based on the module state.
    ///
    /// # Errors
    /// Returns an error if the module state is invalid, the file cannot be read,
    /// or if the data is not properly formatted.
    fn from_bin() -> Result<Self, DataBaseError> {
        use self::bin_loader::load_bin_file;
        use crate::rel::module::ModuleState;

        let (version, runtime) = ModuleState::map_or_init(|module| {
            let version = module.version.clone();
            (version, module.runtime)
        })?;

        let is_ae = runtime.is_ae();
        let path = {
            let ver_suffix = if is_ae { "lib" } else { "" };
            format!("Data/SKSE/Plugins/version{ver_suffix}-{version}.bin")
        };
        let expected_fmt_ver = if is_ae { 2 } else { 1 }; // Expected AddressLibrary format version. SE/VR: 1, AE: 2

        Ok(Self {
            mem_map: load_bin_file(&path, version, expected_fmt_ver)?,
        })
    }

    /// Retrieves the offset corresponding to the given ID.
    ///
    /// # Errors
    /// Returns an error if the ID is not found in the database.
    pub(crate) fn id_to_offset(&self, id: u64) -> Result<usize, DataBaseError> {
        let slice = self
            .mem_map
            .read()
            .map_err(|_| DataBaseError::MappingCreationFailed)?;

        slice.binary_search_by(|m| m.id.cmp(&id)).map_or_else(
            |_| Err(DataBaseError::NotFoundId { id }),
            |index| Ok(slice[index].offset as usize),
        )
    }
}

/// Errors that can occur during the file loading process.
#[derive(Debug, Clone, snafu::Snafu)]
pub enum DataBaseError {
    /// Failed to find the id within the address library: {id}. This means this script extender plugin is incompatible.,
    #[snafu(display("Failed to find the id within the address library: {id}\nThis means this script extender plugin is incompatible."))]
    NotFoundId { id: u64 },

    /// Version mismatch
    #[snafu(display("Version mismatch: expected {}, got {}", expected, actual))]
    VersionMismatch { expected: Version, actual: Version },

    /// Failed to create shared mapping
    MappingCreationFailed,

    /// Failed to locate an appropriate address library at: {path}
    AddressLibraryNotFound { path: String },

    /// Failed to unpack file at: {source}
    FailedUnpackFile { source: self::unpack::UnpackError },

    /// Inherited module state(manager) get error.
    #[snafu(transparent)]
    ModuleStateError {
        source: crate::rel::module::ModuleStateError,
    },

    /// Inherited header parsing error.
    #[snafu(transparent)]
    HeaderParseError { source: self::header::HeaderError },

    /// A thread that was taking database locks panicked.
    Poisoned,

    /// Inherited memory mapping error.
    #[snafu(transparent)]
    MemoryMapError {
        source: super::shared_rwlock::MemoryMapError,
    },
}

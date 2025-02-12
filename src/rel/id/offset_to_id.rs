// C++ Original code
// - https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/REL/ID.h
// - https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/REL/ID.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT

//! Provides functionality to map offsets to unique IDs using a sorted mapping.
//!
//! This module allows efficient lookup of IDs corresponding to memory offsets.
//! The mapping is backed by a sorted vector for quick binary search.

use super::id_database::ID_DATABASE;
use super::shared_rwlock::{PoisonError, RwLockReadGuard};
use super::Mapping;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Maps memory offsets to unique IDs using a sorted list for quick lookup.
pub struct OffsetToID {
    /// Parses bin data from `AddressLibrary` and arranges offset/id pair structures in order of offset.
    offset_to_id: Vec<Mapping>,
}

// FIXME: This side effect is not testable.
impl OffsetToID {
    /// Creates a new `Offset2ID` instance by loading the offset-to-ID mapping(Global instance).
    ///
    /// # Errors
    /// Returns [`MemoryMapCastError`] if the mapping slice cannot be retrieved.
    ///
    /// # Note
    /// Parse the binary table of bin data in `AddressLibrary` and arrange the offset/id pair structures in order of offset,
    /// noting that a call to [`Clone::clone`] is made to prevent sort from destroying the existing table.
    pub fn new() -> Result<Self, PoisonError<RwLockReadGuard<'static, Mapping>>> {
        let mut offset_to_id = ID_DATABASE.mem_map.read()?.to_vec();
        offset_to_id.sort_by(|a, b| a.offset.cmp(&b.offset));
        Ok(Self { offset_to_id })
    }

    /// Gets the ID corresponding to the given `offset`, if available.
    ///
    /// Performs a binary search on the sorted mapping. O(log n)
    pub fn get_id(&self, offset: u64) -> Option<u64> {
        let elem = Mapping { id: 0, offset };
        self.offset_to_id
            .binary_search_by(|m| m.offset.cmp(&elem.offset))
            .map_or_else(|_| None, |index| Some(self.offset_to_id[index].id))
    }
}

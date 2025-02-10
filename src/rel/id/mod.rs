mod byte_reader;
mod header;
mod id_database;
mod memory_map;
mod offset_to_id;
mod relocation_id;
mod variant_id;

pub use self::header::{Header, HeaderError};
pub use self::memory_map::MemoryMap;
pub use self::offset_to_id::OffsetToID;
pub use self::relocation_id::RelocationID;
pub use self::variant_id::VariantID;

/// Represents a memory mapping ID and offset.
///
/// This struct is used to uniquely identify a mapped memory region.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mapping {
    /// The unique ID of the memory-mapped file.
    pub id: u64,
    /// The memory address offset within the mapped region.
    pub offset: u64,
}

/// Represents different formats of the address library.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Format {
    SSEv1,
    SSEv2,
    VR,
}

/// Represents an ID that can be used to look up an address in the ID database.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ID {
    id: u64,
}

impl ID {
    /// Creates a new ID instance.
    #[inline]
    pub const fn new(id: u64) -> Self {
        Self { id }
    }

    /// Retrieves the absolute address corresponding to the ID.
    ///
    /// # Errors
    /// Returns an error if the ID cannot be resolved.
    #[inline]
    pub fn address(&self) -> Result<usize, id_database::DataBaseLoaderError> {
        Ok(Self::base()? + self.offset()?)
    }

    /// Retrieves the offset corresponding to the ID.
    ///
    /// # Errors
    /// Returns an error if the ID is not found.
    #[inline]
    pub fn offset(&self) -> Result<usize, id_database::DataBaseLoaderError> {
        id_database::ID_DATABASE.id_to_offset(self.id)
    }

    /// Retrieves the base address of the module.
    ///
    /// # Errors
    /// Returns an error if the module is in an invalid state.
    #[inline]
    fn base() -> Result<usize, crate::rel::module::ModuleStateError> {
        crate::rel::module::ModuleState::map_or_init(|module| module.base.as_raw())
    }
}

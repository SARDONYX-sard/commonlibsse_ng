mod id_database;
mod offset_to_id;
mod relocation_id;
pub mod shared_rwlock;
mod variant_id;

pub use self::id_database::DataBaseError;
pub use self::offset_to_id::OffsetToID;
pub use self::relocation_id::RelocationID;
pub use self::variant_id::VariantID;

use self::id_database::ID_DATABASE;
use super::ResolvableAddress;

/// Represents a memory mapping ID and offset.
///
/// This struct is used to uniquely identify a mapped memory region.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
///
/// This struct wraps a `u64` value and allows resolution of an absolute address
/// based on the ID's corresponding offset.
///
/// # Example
/// ```rust
/// use commonlibsse_ng::rel::id::ID;
/// use commonlibsse_ng::rel::ResolvableAddress;
///
/// let id = ID::new(42);
/// let address = id.address();
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ID(u64);

impl ID {
    /// Creates a new `ID` instance with the given value.
    #[inline]
    pub const fn new(id: u64) -> Self {
        Self(id)
    }
}

impl ResolvableAddress for ID {
    /// Retrieves the offset corresponding to the ID.
    ///
    /// # Errors
    /// Returns an error if the ID is not found in the database.
    #[inline]
    fn offset(&self) -> Result<usize, DataBaseError> {
        ID_DATABASE.id_to_offset(self.0)
    }
}

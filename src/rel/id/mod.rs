// mod id_database;
#[cfg(feature = "win_api")]
pub mod memory_map;

mod byte_reader;
mod header;
mod id_database;
mod offset_to_id;

pub use self::header::{Header, HeaderError};
pub use self::id_database::IdDatabase;
#[cfg(feature = "win_api")]
pub use self::memory_map::MemoryMap;
pub use self::offset_to_id::OffsetToID;

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

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum Format {
//     SSEv1,
//     SSEv2,
//     VR,
// }

// pub struct ID {
//     id: u64,
// }

// impl ID {
//     pub const fn new(id: u64) -> Self {
//         Self { id }
//     }
//     pub fn address(&self) -> usize {
//         Self::base() + self.offset()
//     }
//     pub fn offset(&self) -> usize {
//         // IDDatabase::get().id_to_offset(self.id)
//         todo!()
//     }
//     fn base() -> usize {
//         todo!()
//     }
// }

// pub struct RelocationID {
//     se_id: Option<u64>,
//     ae_id: Option<u64>,
//     vr_id: Option<u64>,
// }

// impl RelocationID {
//     pub const fn new(se_id: Option<u64>, ae_id: Option<u64>, vr_id: Option<u64>) -> Self {
//         Self {
//             se_id,
//             ae_id,
//             vr_id,
//         }
//     }
//     pub fn address(&self) -> usize {
//         let offset = self.offset();
//         if offset == 0 {
//             0
//         } else {
//             Self::base() + offset
//         }
//     }
//     pub fn offset(&self) -> usize {
//         self.id().map_or(0, |id| {
//             // IDDatabase::get().id_to_offset(id)
//             todo!()
//         })
//     }
//     pub fn id(&self) -> Option<u64> {
//         todo!()
//     }
//     fn base() -> usize {
//         todo!()
//     }
// }

// pub struct VariantID {
//     se_id: Option<u64>,
//     ae_id: Option<u64>,
//     vr_offset: Option<usize>,
// }

// impl VariantID {
//     pub const fn new(se_id: Option<u64>, ae_id: Option<u64>, vr_offset: Option<usize>) -> Self {
//         Self {
//             se_id,
//             ae_id,
//             vr_offset,
//         }
//     }
//     pub fn address(&self) -> usize {
//         let offset = self.offset();
//         if offset == 0 {
//             0
//         } else {
//             Self::base() + offset
//         }
//     }
//     pub fn offset(&self) -> usize {
//         todo!()
//     }
//     fn base() -> usize {
//         todo!()
//     }
// }

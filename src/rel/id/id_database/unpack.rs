use crate::rel::id::byte_reader::{read_le_u16, read_le_u32, read_le_u64, read_u8};
use crate::rel::id::memory_map::{MemoryMap, MemoryMapCastSizeError};
use crate::rel::id::Mapping;
use std::io::Read;

/// Unpacks the ID database from the binary file and writes it into the memory map(sorted by ID).
///
/// # Errors
/// - If the memory allocated as `MemoryMap` is not consistent as the length of the mapping data array.
/// - Returns an error if the binary data cannot be properly parsed.
pub(crate) fn unpack_file<R>(
    mem_map: &MemoryMap,
    reader: &mut R,
    ptr_size: u64,
) -> Result<(), UnpackError>
where
    R: Read,
{
    // TODO: Parse With `winnow` crate, we can know the exact binary position at the time of the error.
    let mut offset: u64;
    let mut prev_id: u64 = 0;
    let mut prev_offset: u64 = 0;

    let mappings = mem_map.as_mapping_slice_mut()?;
    for mapping in &mut *mappings {
        let type_byte = read_u8(reader)?;

        let low = type_byte & 0xF;
        let high = type_byte >> 4;

        let id = parse_id(low, reader, prev_id)?;

        let tmp = if (high & 8) != 0 {
            prev_offset / ptr_size
        } else {
            prev_offset
        };

        offset = parse_offset(high, reader, tmp)?;

        if (high & 8) != 0 {
            offset *= ptr_size;
        }

        *mapping = Mapping { id, offset };
        prev_id = id;
        prev_offset = offset;
    }

    mappings.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(())
}

fn parse_id<R>(low: u8, reader: &mut R, prev_id: u64) -> Result<u64, UnpackError>
where
    R: Read,
{
    Ok(match low {
        0 => read_le_u64(reader)?,
        1 => prev_id + 1,
        2 => prev_id + read_u8(reader)? as u64,
        3 => prev_id - read_u8(reader)? as u64,
        4 => prev_id + read_le_u16(reader)? as u64,
        5 => prev_id - read_le_u16(reader)? as u64,
        6 => read_le_u16(reader)? as u64,
        7 => read_le_u32(reader)? as u64,
        _ => return Err(UnpackError::InvalidId { id: low }),
    })
}

fn parse_offset<R>(high: u8, reader: &mut R, prev_offset: u64) -> Result<u64, UnpackError>
where
    R: Read,
{
    Ok(match high & 7 {
        0 => read_le_u64(reader)?,
        1 => prev_offset + 1,
        2 => prev_offset + read_u8(reader)? as u64,
        3 => prev_offset - read_u8(reader)? as u64,
        4 => prev_offset + read_le_u16(reader)? as u64,
        5 => prev_offset - read_le_u16(reader)? as u64,
        6 => read_le_u16(reader)? as u64,
        7 => read_le_u32(reader)? as u64,
        _ => {
            return Err(UnpackError::InvalidOffset {
                offset: prev_offset,
            })
        }
    })
}

#[derive(Debug, snafu::Snafu)]
pub enum UnpackError {
    /// Invalid ID encountered
    #[snafu(display("Invalid ID encountered: {}", id))]
    InvalidId { id: u8 },

    /// Invalid offset encountered
    #[snafu(display("Invalid offset encountered: {}", offset))]
    InvalidOffset { offset: u64 },

    /// Inherited memory mapping error.
    #[snafu(transparent)]
    MemoryMapCastError { source: MemoryMapCastSizeError },

    /// Inherited IO Error
    #[snafu(transparent)]
    IoError { source: std::io::Error },
}

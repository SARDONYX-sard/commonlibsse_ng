use crate::rel::id::header::Header;
use crate::rel::id::id_database::unpack::unpack_file;
use crate::rel::id::id_database::{
    AddressLibraryNotFoundSnafu, DataBaseError, FailedUnpackFileSnafu,
};
use crate::rel::id::shared_rwlock::SharedRwLock;
use crate::rel::id::Mapping;
use crate::rel::version::Version;
use snafu::ResultExt as _;

/// Reads, parses, and writes binary database files into memory.
/// Then returns the written memory.
///
/// - `expected_fmt_ver`: Expected AddressLibrary format version. SE/VR: 1, AE: 2
///
/// # Errors
/// - If the specified path does not exist.
/// - If the version without bin file mismatches with the runtime
/// - If parsing of the data in the bin file fails.
/// - Failure to allocate memory for bin file storage.
pub(super) fn load_bin_file(
    path: &str,
    version: Version,
    expected_fmt_ver: u8,
) -> Result<SharedRwLock<Mapping>, DataBaseError> {
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
        return Err(DataBaseError::VersionMismatch {
            expected: version,
            actual: header.version,
        });
    }

    let map_name = windows::core::HSTRING::from(format!("CommonLibSSEOffsets-v2-{version}"));

    let (mem_map, is_created) = SharedRwLock::new(&map_name, header.address_count())
        .map_err(|err| DataBaseError::MemoryMapError { source: err })?;

    if is_created {
        let mut mem_map = mem_map.write().map_err(|_| DataBaseError::Poisoned)?;
        unpack_file(&mut mem_map, &mut reader, header.pointer_size())
            .context(FailedUnpackFileSnafu)?;
    }

    Ok(mem_map)
}

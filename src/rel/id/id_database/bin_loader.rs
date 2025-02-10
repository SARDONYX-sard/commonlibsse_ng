use crate::rel::id::header::Header;
use crate::rel::id::id_database::unpack::unpack_file;
use crate::rel::id::id_database::{
    AddressLibraryNotFoundSnafu, DataBaseLoaderError, FailedUnpackFileSnafu,
};
use crate::rel::id::memory_map::MemoryMap;
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
) -> Result<MemoryMap, DataBaseLoaderError> {
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
        unpack_file(&mem_map, &mut reader, header.pointer_size()).context(FailedUnpackFileSnafu)?;
        mem_map
    } else {
        return Err(DataBaseLoaderError::MappingCreationFailed);
    };

    Ok(mem_map)
}

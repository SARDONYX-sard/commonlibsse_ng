use std::path::Path;

use crate::rel::id::Mapping;
use crate::rel::id::id_database::header::Header;
use crate::rel::id::id_database::unpack::unpack_file;
use crate::rel::id::id_database::{DataBaseError, FailedUnpackFileSnafu};
use crate::rel::version::Version;
use shared_rwlock::SharedRwLock;
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
pub(super) fn load_bin_file<P>(
    path: P,
    version: Version,
    expected_fmt_ver: u8,
) -> Result<SharedRwLock<Mapping>, DataBaseError>
where
    P: AsRef<Path>,
{
    load_bin_file_inner(path.as_ref(), version, expected_fmt_ver)
}

fn load_bin_file_inner(
    path: &Path,
    version: Version,
    expected_fmt_ver: u8,
) -> Result<SharedRwLock<Mapping>, DataBaseError> {
    use std::fs::File;
    use std::io;

    let mut reader = {
        let file = File::open(path)
            .map_err(|_| DataBaseError::AddressLibraryNotFound { path: path.to_path_buf() })?;
        io::BufReader::new(file)
    };

    let header = Header::from_reader(&mut reader, expected_fmt_ver)?;
    if header.version != version {
        return Err(DataBaseError::VersionMismatch { expected: version, actual: header.version });
    }

    let (mem_map, is_created) = {
        let shared_id =
            windows::core::HSTRING::from(format!("CommonLibSSEOffsets-rs-v2-{version}"));
        SharedRwLock::new(&shared_id, header.address_count())
    }
    .map_err(|err| DataBaseError::MemoryMapError { source: err })?;

    if is_created {
        let mut mem_map = mem_map.write().map_err(|_| DataBaseError::Poisoned)?;
        unpack_file(&mut mem_map, &mut reader, header.pointer_size())
            .context(FailedUnpackFileSnafu)?;
    }

    Ok(mem_map)
}

#[cfg(feature = "test_on_local")]
#[cfg(test)]
mod local_tests {
    use core::ffi::c_void;
    use core::num::NonZeroUsize;
    use core::ptr::NonNull;

    use super::*;
    use crate::rel::ResolvableAddress;
    use crate::rel::id::IDDatabase;
    use crate::rel::module::{ModuleStateError, Runtime, get_skyrim_dir};
    use crate::rel::version::Version;

    // ---- Config ---------------------------------------------------------------
    // const VERSION: Version = Version::new(1, 6, 1170, 0);
    const VERSION: Version = Version::new(1, 6, 353, 0);
    const MODULE_BASE: usize = 0x1000;
    // ---------------------------------------------------------------------------

    #[derive(Debug)]
    struct TestRelocation {
        db: IDDatabase,
        current_id: u64,
    }

    impl TestRelocation {
        const fn new(mem_map: SharedRwLock<Mapping>) -> Self {
            Self { db: IDDatabase { mem_map }, current_id: 0 }
        }

        const fn set_id(&mut self, id: u64) {
            self.current_id = id;
        }
    }

    impl ResolvableAddress for TestRelocation {
        fn offset(&self) -> Result<NonZeroUsize, DataBaseError> {
            self.db.id_to_offset(self.current_id)
        }

        fn address(&self) -> Result<NonNull<c_void>, DataBaseError> {
            let offset = self.offset()?;
            Ok(unsafe { Self::base()?.byte_add(offset.get()) })
        }

        // Set dummy module base(SkyrimSE.exe address)
        fn base() -> Result<NonNull<c_void>, ModuleStateError> {
            NonNull::new(core::ptr::without_provenance_mut(MODULE_BASE))
                .ok_or(ModuleStateError::ModuleLockIsPoisoned)
        }
    }

    #[allow(unused)]
    fn write_debug_value(value: impl core::fmt::Debug) -> std::io::Result<()> {
        const TARGET: &str = env!("CARGO_MANIFEST_DIR");
        std::fs::write(format!("{TARGET}/address_dump.log"), format!("{value:#?}"))
    }

    // REQUIREMENT: We need the version of AddressLibrary specified in Skyrim's Data on Steam.
    #[test]
    fn test_load_bin() {
        let mut test_rel = {
            let runtime = Runtime::from_version(&VERSION);
            let ver_suffix = if runtime.is_ae() { "lib" } else { "" };
            let path = get_skyrim_dir(runtime).unwrap().join(format!(
                "Data/SKSE/Plugins/version{ver_suffix}-{}.bin",
                VERSION.to_address_library_string(),
            ));
            TestRelocation::new(load_bin_file(&path, VERSION, 2).unwrap())
        };

        // write_debug_value(&test_rel).unwrap();
        test_rel.set_id(11483);
        assert_eq!(test_rel.offset().unwrap().get(), 0x10f7a0);
        assert_eq!(test_rel.address().unwrap().addr().get(), MODULE_BASE + 0x10f7a0);
    }
}

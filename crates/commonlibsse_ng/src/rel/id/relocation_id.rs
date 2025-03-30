use core::num::NonZeroUsize;

use crate::rel::{ResolvableAddress, id::id_database::DataBaseError};

/// Represents an ID that varies based on runtime format.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RelocationID {
    se_id: u64,
    ae_id: u64,
    vr_id: u64,
}

impl RelocationID {
    /// Creates a new RelocationID instance.
    #[inline]
    pub const fn new(se_id: u64, ae_id: u64, vr_id: u64) -> Self {
        Self { se_id, ae_id, vr_id }
    }

    /// Creates a new RelocationID instance.
    ///
    /// Used when vr and se take the same ID.
    #[inline]
    pub const fn from_se_ae_id(se_vr_id: u64, ae_id: u64) -> Self {
        Self { se_id: se_vr_id, ae_id, vr_id: se_vr_id }
    }

    /// Retrieves the appropriate ID based on the runtime format.
    ///
    /// # Errors
    /// Returns an error if the module is in an invalid state.
    #[inline]
    pub fn id(&self) -> Result<u64, crate::rel::module::ModuleStateError> {
        use crate::rel::module::{ModuleState, Runtime};

        let runtime = ModuleState::map_or_init(|module| module.runtime)?; // derived Copy

        Ok(match runtime {
            Runtime::Ae => self.ae_id,
            Runtime::Se => self.se_id,
            Runtime::Vr => self.vr_id,
        })
    }
}

impl ResolvableAddress for RelocationID {
    #[inline]
    fn offset(&self) -> Result<NonZeroUsize, DataBaseError> {
        use crate::rel::id::id_database::ID_DATABASE;
        ID_DATABASE.id_to_offset(self.id()?)
    }
}

#[cfg(feature = "test_on_local")]
#[cfg(test)]
mod local_tests {
    use super::*;

    // REQUIREMENT: We need the version of AddressLibrary specified in Skyrim's Data on Steam.
    #[test]
    fn test_relocation() {
        let relocation = RelocationID::from_se_ae_id(514287, 400447); // Calender::get_singleton() -> *mut Calender
        if let (Ok(offset), Ok(address)) = (relocation.offset(), relocation.address()) {
            dbg!(offset, &address); // AE offset(ver. 1.6.1170.0): 34572640
        }
    }
}

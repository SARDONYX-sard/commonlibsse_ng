use crate::rel::id::id_database::DataBaseError;

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
        Self {
            se_id,
            ae_id,
            vr_id,
        }
    }

    /// Retrieves the absolute address corresponding to the ID.
    ///
    /// # Errors
    /// Returns an error if the ID cannot be resolved.
    #[inline]
    pub fn address(&self) -> Result<usize, DataBaseError> {
        let offset = self.offset()?;
        Ok(if offset == 0 {
            0
        } else {
            Self::base()? + offset
        })
    }

    /// Retrieves the offset corresponding to the ID.
    ///
    /// # Errors
    /// Returns an error if the ID is not found.
    #[inline]
    pub fn offset(&self) -> Result<usize, DataBaseError> {
        crate::rel::id::id_database::ID_DATABASE.id_to_offset(self.id()?)
    }

    /// Retrieves the appropriate ID based on the runtime format.
    ///
    /// # Errors
    /// Returns an error if the module is in an invalid state.
    pub fn id(&self) -> Result<u64, crate::rel::module::ModuleStateError> {
        use crate::rel::module::{ModuleState, Runtime};

        let runtime = ModuleState::map_or_init(|module| module.runtime)?; // derived Copy

        Ok(match runtime {
            Runtime::Ae => self.ae_id,
            Runtime::Se => self.se_id,
            Runtime::Vr => self.vr_id,
        })
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

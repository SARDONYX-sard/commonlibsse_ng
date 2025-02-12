use crate::rel::id::id_database::DataBaseError;

/// Represents an ID with a possible VR-specific offset.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VariantID {
    se_id: u64,
    ae_id: u64,
    vr_offset: u64,
}

impl VariantID {
    /// Creates a new VariantID instance.
    #[inline]
    pub const fn new(se_id: u64, ae_id: u64, vr_offset: u64) -> Self {
        Self {
            se_id,
            ae_id,
            vr_offset,
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
    pub fn offset(&self) -> Result<usize, DataBaseError> {
        use crate::rel::module::{ModuleState, Runtime};

        let runtime = ModuleState::map_or_init(|module| module.runtime)?; // derived Copy

        let id = match runtime {
            Runtime::Unknown => 0,
            Runtime::Ae => self.ae_id,
            Runtime::Se => self.se_id,
            Runtime::Vr => self.vr_offset,
        };

        crate::rel::id::id_database::ID_DATABASE.id_to_offset(id)
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

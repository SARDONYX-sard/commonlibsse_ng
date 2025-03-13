use core::num::NonZeroUsize;

use crate::rel::{ResolvableAddress, id::id_database::DataBaseError};

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
        Self { se_id, ae_id, vr_offset }
    }
}

impl ResolvableAddress for VariantID {
    fn offset(&self) -> Result<NonZeroUsize, DataBaseError> {
        use crate::rel::id::id_database::ID_DATABASE;
        use crate::rel::module::{ModuleState, Runtime};

        let runtime = ModuleState::map_or_init(|module| module.runtime)?;
        let id = match runtime {
            Runtime::Ae => self.ae_id,
            Runtime::Se => self.se_id,
            Runtime::Vr => self.vr_offset,
        };

        ID_DATABASE.id_to_offset(id)
    }
}

use std::ffi::c_void;

use crate::skse::{
    api::{ApiStorageError, get_plugin_handle},
    impls::stab::SKSETrampolineInterface,
};

#[derive(Debug, Clone)]
pub struct TrampolineInterface(&'static SKSETrampolineInterface);

impl TrampolineInterface {
    /// The version number of the trampoline interface.
    pub const VERSION: u32 = 1;

    #[inline]
    pub(crate) const fn new(interface: &'static SKSETrampolineInterface) -> Self {
        Self(interface)
    }

    /// Returns the version number of the trampoline interface.
    #[inline]
    pub const fn version(&self) -> u32 {
        self.0.interfaceVersion
    }

    /// Allocates memory from the branch pool.
    ///
    /// # Errors
    /// If the internal global API storage is uninitialized because forgot to call `skse::init`
    #[inline]
    pub fn allocate_from_branch_pool(&self, size: usize) -> Result<*mut c_void, ApiStorageError> {
        Ok(unsafe { (self.0.AllocateFromBranchPool)(get_plugin_handle()?, size) })
    }

    /// Allocates memory from the local pool.
    ///
    /// # Errors
    /// If the internal global API storage is uninitialized because forgot to call `skse::init`
    #[inline]
    pub fn allocate_from_local_pool(&self, size: usize) -> Result<*mut c_void, ApiStorageError> {
        Ok(unsafe { (self.0.AllocateFromLocalPool)(get_plugin_handle()?, size) })
    }
}

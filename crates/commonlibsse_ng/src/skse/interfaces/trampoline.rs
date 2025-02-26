use std::ffi::c_void;

use crate::skse::{api::get_plugin_handle, impls::stab::SKSETrampolineInterface};

#[derive(Debug, Clone)]
pub struct TrampolineInterface(&'static SKSETrampolineInterface);

impl TrampolineInterface {
    pub const VERSION: u32 = 1;

    #[inline]
    pub(crate) const fn new(interface: &'static SKSETrampolineInterface) -> Self {
        Self(interface)
    }

    #[inline]
    pub const fn version(&self) -> u32 {
        self.0.interfaceVersion
    }

    #[inline]
    pub fn allocate_from_branch_pool(&self, size: usize) -> *mut c_void {
        unsafe { (self.0.AllocateFromBranchPool)(get_plugin_handle(), size) }
    }

    #[inline]
    pub fn allocate_from_local_pool(&self, size: usize) -> *mut c_void {
        unsafe { (self.0.AllocateFromLocalPool)(get_plugin_handle(), size) }
    }
}

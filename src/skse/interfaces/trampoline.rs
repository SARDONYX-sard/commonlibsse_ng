use std::ffi::c_void;

use crate::skse::{api::get_plugin_handle, impls::stab::SKSETrampolineInterface};

#[derive(Debug)]
pub struct TrampolineInterface {
    address: *const u8,
}

impl TrampolineInterface {
    pub const VERSION: u32 = 1;

    pub fn version(&self) -> u32 {
        unsafe { (*self.get_proxy()).interface_version }
    }

    pub fn allocate_from_branch_pool(&self, size: usize) -> *mut c_void {
        unsafe { ((*self.get_proxy()).allocate_from_branch_pool)(get_plugin_handle(), size) }
    }

    pub fn allocate_from_local_pool(&self, size: usize) -> *mut c_void {
        unsafe { ((*self.get_proxy()).allocate_from_local_pool)(get_plugin_handle(), size) }
    }

    fn get_proxy(&self) -> *const SKSETrampolineInterface {
        assert!(!self.address.is_null());
        self.address.cast()
    }
}

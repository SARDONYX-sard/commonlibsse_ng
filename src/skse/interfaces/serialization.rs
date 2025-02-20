// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/SKSE/Interfaces.h
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/SKSE/Interfaces.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

use core::ffi::c_void;

use crate::re::{FormID, VMHandle};
use crate::skse::api::get_plugin_handle;
use crate::skse::impls::stab::SKSESerializationInterface;

type EventCallback = fn(a_intfc: &SerializationInterface);
type FormDeleteCallback = fn(a_handle: VMHandle);

#[derive(Debug)]
pub struct SerializationInterface {
    address: *const u8,
}
impl SerializationInterface {
    pub const VERSION: u32 = 4;

    pub fn version(&self) -> u32 {
        unsafe { (*self.get_proxy()).version }
    }

    pub fn set_unique_id(&self, uid: u32) {
        unsafe { ((*self.get_proxy()).set_unique_id)(get_plugin_handle(), uid) }
    }

    pub fn set_form_delete_callback(&self, callback: FormDeleteCallback) {
        #[allow(clippy::fn_to_numeric_cast_any)]
        let callback = (callback as *mut FormDeleteCallback).cast();
        unsafe { ((*self.get_proxy()).set_form_delete_callback)(get_plugin_handle(), callback) }
    }

    pub fn set_load_callback(&self, callback: EventCallback) {
        #[allow(clippy::fn_to_numeric_cast_any)]
        let callback = (callback as *mut EventCallback).cast();
        unsafe { ((*self.get_proxy()).set_load_callback)(get_plugin_handle(), callback) }
    }
    pub fn set_revert_callback(&self, callback: EventCallback) {
        #[allow(clippy::fn_to_numeric_cast_any)]
        let callback = (callback as *mut EventCallback).cast();
        unsafe { ((*self.get_proxy()).set_revert_callback)(get_plugin_handle(), callback) }
    }
    pub fn set_save_callback(&self, callback: EventCallback) {
        #[allow(clippy::fn_to_numeric_cast_any)]
        let callback = (callback as *mut EventCallback).cast();
        unsafe { ((*self.get_proxy()).set_save_callback)(get_plugin_handle(), callback) }
    }

    pub fn write_record<T: Sized>(&self, record_type: u32, version: u32, buf: &T) -> bool {
        self.write_record_raw(
            record_type,
            version,
            (buf as *const T).cast::<c_void>(),
            core::mem::size_of::<T>(),
        )
    }

    fn write_record_raw(
        &self,
        record_type: u32,
        version: u32,
        buf: *const c_void,
        length: usize,
    ) -> bool {
        unsafe { ((*self.get_proxy()).write_record)(record_type, version, buf, length as u32) }
    }

    pub fn open_recode(&self, record_type: u32, version: u32) -> bool {
        unsafe { ((*self.get_proxy()).open_record)(record_type, version) }
    }

    pub fn write_record_data<T>(&self, buf: &[T]) -> bool {
        const U32_MAX: usize = u32::MAX as usize;
        let buf_len = buf.len();
        match buf_len {
            0 => true,
            1..=U32_MAX => unsafe {
                ((*self.get_proxy()).write_record_data)(buf.as_ptr().cast(), buf_len as u32)
            },
            _ => false,
        }
    }

    pub fn get_next_record_info(
        &self,
        record_type: &mut u32,
        version: &mut u32,
        length: &mut u32,
    ) -> bool {
        unsafe { ((*self.get_proxy()).get_next_record_info)(record_type, version, length) }
    }

    pub fn read_record_data<T>(&self, buf: &mut [T]) -> u32 {
        unsafe { ((*self.get_proxy()).read_record_data)(buf.as_mut_ptr().cast(), buf.len() as u32) }
    }

    pub fn resolve_form_id(&self, old: FormID, new: &mut FormID) -> bool {
        unsafe { ((*self.get_proxy()).resolve_form_id)(old.0, &mut new.0) }
    }

    pub fn resolve_handle(&self, old: VMHandle, new: &mut VMHandle) -> bool {
        unsafe { ((*self.get_proxy()).resolve_handle)(old.0, &mut new.0) }
    }

    fn get_proxy(&self) -> *const SKSESerializationInterface {
        assert!(!self.address.is_null());
        self.address.cast()
    }
}

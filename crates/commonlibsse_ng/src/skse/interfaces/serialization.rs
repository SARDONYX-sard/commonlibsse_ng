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

const U32_MAX: usize = u32::MAX as usize;
type EventCallback = fn(a_intfc: &SerializationInterface);
type FormDeleteCallback = fn(a_handle: VMHandle);

#[derive(Debug, Clone)]
pub struct SerializationInterface(&'static SKSESerializationInterface);

impl SerializationInterface {
    pub const VERSION: u32 = 4;

    #[inline]
    pub(crate) const fn new(interface: &'static SKSESerializationInterface) -> Self {
        Self(interface)
    }

    #[inline]
    pub const fn version(&self) -> u32 {
        self.0.version
    }

    #[inline]
    pub fn set_unique_id(&self, uid: u32) {
        unsafe { (self.0.SetUniqueId)(get_plugin_handle(), uid) }
    }

    #[inline]
    pub fn set_form_delete_callback(&self, callback: FormDeleteCallback) {
        #[allow(clippy::fn_to_numeric_cast_any)]
        let callback = (callback as *mut FormDeleteCallback).cast();
        unsafe { ((self.0).SetFormDeleteCallback)(get_plugin_handle(), callback) }
    }

    #[inline]
    pub fn set_load_callback(&self, callback: EventCallback) {
        #[allow(clippy::fn_to_numeric_cast_any)]
        let callback = (callback as *mut EventCallback).cast();
        unsafe { ((self.0).SetLoadCallback)(get_plugin_handle(), callback) }
    }

    #[inline]
    pub fn set_revert_callback(&self, callback: EventCallback) {
        #[allow(clippy::fn_to_numeric_cast_any)]
        let callback = (callback as *mut EventCallback).cast();
        unsafe { ((self.0).SetRevertCallback)(get_plugin_handle(), callback) }
    }

    #[inline]
    pub fn set_save_callback(&self, callback: EventCallback) {
        #[allow(clippy::fn_to_numeric_cast_any)]
        let callback = (callback as *mut EventCallback).cast();
        unsafe { ((self.0).SetSaveCallback)(get_plugin_handle(), callback) }
    }

    /// # Note
    /// buf len <= [`u32::MAX`]
    ///
    /// [`u32::MAX`]: https://doc.rust-lang.org/nightly/core/primitive.u32.html#associatedconstant.MAX
    ///
    /// # Errors
    /// If failed to write
    #[inline]
    pub fn write_record<T>(&self, record_type: u32, version: u32, buf: &T) -> Result<(), Error> {
        let data_size: usize = core::mem::size_of::<T>();
        debug_assert!(data_size <= U32_MAX, "Should be T size <= u32::MAX");

        let void_buf = (buf as *const T).cast::<c_void>();
        let result =
            unsafe { ((self.0).WriteRecord)(record_type, version, void_buf, data_size as u32) };

        if result {
            Ok(())
        } else {
            Err(Error::WriteRecordError)
        }
    }

    /// # Errors
    /// If failed to open
    #[inline]
    pub fn open_recode(&self, record_type: u32, version: u32) -> Result<(), Error> {
        if unsafe { ((self.0).OpenRecord)(record_type, version) } {
            Ok(())
        } else {
            Err(Error::OpenRecordError)
        }
    }

    /// # Errors
    /// If `buf.len()` >= `u32::MAX`
    #[inline]
    pub fn write_record_data<T>(&self, buf: &[T]) -> Result<(), Error> {
        let buf_len = buf.len();

        match buf_len {
            0 => Ok(()),
            1..=U32_MAX => {
                let result =
                    unsafe { ((self.0).WriteRecordData)(buf.as_ptr().cast(), buf_len as u32) };
                if result {
                    Ok(())
                } else {
                    Err(Error::WriteRecordDataError)
                }
            }
            too_large_size => Err(Error::TooLargeWriteRecordData {
                actual: too_large_size,
            }),
        }
    }

    #[inline]
    pub fn read_record_data<T>(&self, buf: &mut [T]) -> u32 {
        unsafe { (self.0.ReadRecordData)(buf.as_mut_ptr().cast(), buf.len() as u32) }
    }

    /// # Errors
    #[inline]
    pub fn get_next_record_info(
        &self,
        record_type: &mut u32,
        version: &mut u32,
        length: &mut u32,
    ) -> Result<(), Error> {
        unsafe {
            let result = (self.0.GetNextRecordInfo)(record_type, version, length);
            if result {
                Ok(())
            } else {
                Err(Error::GetNextRecordInfoError)
            }
        }
    }

    /// # Errors
    #[inline]
    pub fn resolve_form_id(&self, old: FormID, new: &mut FormID) -> Result<(), Error> {
        unsafe {
            let result = (self.0.ResolveFormId)(old.0, &mut new.0);
            if result {
                Ok(())
            } else {
                Err(Error::ResolveFormIdError)
            }
        }
    }

    /// # Errors
    #[inline]
    pub fn resolve_handle(&self, old: VMHandle, new: &mut VMHandle) -> Result<(), Error> {
        let result = unsafe { (self.0.ResolveHandle)(old.0, &mut new.0) };
        if result {
            Ok(())
        } else {
            Err(Error::ResolveHandleError)
        }
    }
}

// Define a custom error type to represent failure cases
#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// Failed to write record
    WriteRecordError,

    /// Failed to write record data
    WriteRecordDataError,

    /// The maximum length that can be written is `u32::MAX`, but what is indicated is the length of {actual}.
    TooLargeWriteRecordData {
        /// The actual length of the data
        actual: usize,
    },

    /// Failed to open record
    OpenRecordError,

    /// Failed to get next record info
    GetNextRecordInfoError,

    /// Failed to resolve form ID
    ResolveFormIdError,

    /// Failed to resolve handle
    ResolveHandleError,
}

// C++ Original code
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/include/SKSE/Interfaces.h
// - ref: https://github.com/SARDONYX-forks/CommonLibVR/blob/ng/src/SKSE/Interfaces.cpp
// SPDX-FileCopyrightText: (C) 2018 Ryan-rsm-McKenzie
// SPDX-License-Identifier: MIT
//
// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

use core::ffi::c_void;

use crate::re::BSCoreTypes::{FormID, VMHandle};
use crate::skse::api::{ApiStorageError, get_plugin_handle};
use crate::skse::impls::stab::SKSESerializationInterface;

const U32_MAX: usize = u32::MAX as usize;

/// Interface for interacting with SKSE's serialization functions.
///
/// This struct provides methods for interacting with serialization operations
/// such as setting unique IDs, handling callbacks for form deletions and loads,
/// and reading/writing record data.
#[derive(Debug, Clone)]
pub struct SerializationInterface(&'static SKSESerializationInterface);

impl SerializationInterface {
    /// The version number of the serialization interface.
    pub const VERSION: u32 = 4;

    /// Creates a new `SerializationInterface` instance.
    #[inline]
    pub(crate) const fn new(interface: &'static SKSESerializationInterface) -> Self {
        Self(interface)
    }

    /// Returns the version of the serialization interface.
    #[inline]
    pub const fn version(&self) -> u32 {
        self.0.version
    }

    /// Sets a unique identifier for the serialization interface.
    ///
    /// # Errors
    /// Returns an error if the internal global API storage is uninitialized
    /// (e.g., if `skse::init` has not been called).
    #[inline]
    pub fn set_unique_id(&self, uid: u32) -> Result<(), ApiStorageError> {
        unsafe { (self.0.SetUniqueId)(get_plugin_handle()?, uid) };
        Ok(())
    }

    /// Sets a callback function that will be called when a form is deleted.
    ///
    /// # Errors
    /// Returns an error if the internal global API storage is uninitialized
    /// (e.g., if `skse::init` has not been called).
    #[inline]
    pub fn set_form_delete_callback(
        &self,
        callback: fn(handle: VMHandle),
    ) -> Result<(), ApiStorageError> {
        #[allow(clippy::fn_to_numeric_cast_any)]
        let void_f = (callback as *mut fn(handle: VMHandle)).cast();
        unsafe { ((self.0).SetFormDeleteCallback)(get_plugin_handle()?, void_f) }

        Ok(())
    }

    /// Sets a callback function that will be called when the plugin is loaded.
    ///
    /// # Errors
    /// Returns an error if the internal global API storage is uninitialized
    /// (e.g., if `skse::init` has not been called).
    #[inline]
    pub fn set_load_callback(&self, callback: fn(&Self)) -> Result<(), ApiStorageError> {
        #[allow(clippy::fn_to_numeric_cast_any)]
        let void_f = (callback as *mut fn(&Self)).cast();
        unsafe { ((self.0).SetLoadCallback)(get_plugin_handle()?, void_f) }
        Ok(())
    }

    /// Sets a callback function that will be called when the plugin is reverted.
    ///
    /// # Errors
    /// Returns an error if the internal global API storage is uninitialized
    /// (e.g., if `skse::init` has not been called).
    #[inline]
    pub fn set_revert_callback(&self, callback: fn(&Self)) -> Result<(), ApiStorageError> {
        #[allow(clippy::fn_to_numeric_cast_any)]
        let void_f = (callback as *mut fn(&Self)).cast();
        unsafe { ((self.0).SetRevertCallback)(get_plugin_handle()?, void_f) }
        Ok(())
    }

    /// Sets a callback function that will be called when the plugin is saved.
    ///
    /// # Errors
    /// Returns an error if the internal global API storage is uninitialized
    /// (e.g., if `skse::init` has not been called).
    #[inline]
    pub fn set_save_callback(&self, callback: fn(&Self)) -> Result<(), ApiStorageError> {
        #[allow(clippy::fn_to_numeric_cast_any)]
        let callback = (callback as *mut fn(&Self)).cast();
        unsafe { ((self.0).SetSaveCallback)(get_plugin_handle()?, callback) }
        Ok(())
    }

    /// Writes a record to the serialization interface.
    ///
    /// # Errors
    /// Returns an error if the write operation fails or if the buffer length exceeds `u32::MAX`.
    #[inline]
    pub fn write_record<T>(&self, record_type: u32, version: u32, buf: &T) -> Result<(), Error> {
        let data_size: usize = core::mem::size_of::<T>();

        if data_size > U32_MAX {
            return Err(Error::TooLargeWriteRecordData { actual: data_size });
        }

        let void_buf = (buf as *const T).cast::<c_void>();
        let result =
            unsafe { ((self.0).WriteRecord)(record_type, version, void_buf, data_size as u32) };

        if result { Ok(()) } else { Err(Error::WriteRecordError) }
    }

    /// Opens a record for writing.
    ///
    /// # Errors
    /// Returns an error if the open operation fails.
    #[inline]
    pub fn open_recode(&self, record_type: u32, version: u32) -> Result<(), Error> {
        if unsafe { ((self.0).OpenRecord)(record_type, version) } {
            Ok(())
        } else {
            Err(Error::OpenRecordError)
        }
    }

    /// Writes record data to the serialization interface.
    ///
    /// # Errors
    /// Returns an error if the write operation fails or if the buffer length exceeds `u32::MAX`.
    #[inline]
    pub fn write_record_data<T>(&self, buf: &[T]) -> Result<(), Error> {
        let buf_len = buf.len();

        match buf_len {
            0 => Ok(()),
            1..=U32_MAX => {
                let result =
                    unsafe { ((self.0).WriteRecordData)(buf.as_ptr().cast(), buf_len as u32) };
                if result { Ok(()) } else { Err(Error::WriteRecordDataError) }
            }
            too_large_size => Err(Error::TooLargeWriteRecordData { actual: too_large_size }),
        }
    }

    /// Reads record data into the provided buffer.
    ///
    /// # Returns
    /// The number of bytes read.
    #[inline]
    pub fn read_record_data<T>(&self, buf: &mut [T]) -> u32 {
        unsafe { (self.0.ReadRecordData)(buf.as_mut_ptr().cast(), buf.len() as u32) }
    }

    /// Retrieves the next record's type, version, and length.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    #[inline]
    pub fn get_next_record_info(
        &self,
        record_type: &mut u32,
        version: &mut u32,
        length: &mut u32,
    ) -> Result<(), Error> {
        unsafe {
            let result = (self.0.GetNextRecordInfo)(record_type, version, length);
            if result { Ok(()) } else { Err(Error::GetNextRecordInfoError) }
        }
    }

    /// Resolves the new form ID based on an old form ID.
    ///
    /// # Errors
    /// Returns an error if the form ID resolution fails.
    #[inline]
    pub fn resolve_form_id(&self, old: FormID, new: &mut FormID) -> Result<(), Error> {
        unsafe {
            let result = (self.0.ResolveFormId)(old.get(), &mut new.get());
            if result { Ok(()) } else { Err(Error::ResolveFormIdError) }
        }
    }

    /// Resolves the new handle based on an old handle.
    ///
    /// # Errors
    /// Returns an error if the handle resolution fails.
    #[inline]
    pub fn resolve_handle(&self, old: VMHandle, new: &mut VMHandle) -> Result<(), Error> {
        let result = unsafe { (self.0.ResolveHandle)(old.get(), &mut new.get()) };
        if result { Ok(()) } else { Err(Error::ResolveHandleError) }
    }
}

/// Custom error type for serialization-related failures.
#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// Failed to write record.
    WriteRecordError,

    /// Failed to write record data.
    WriteRecordDataError,

    /// The buffer size exceeds the maximum allowed (`u32::MAX`).
    TooLargeWriteRecordData { actual: usize },

    /// Failed to open record.
    OpenRecordError,

    /// Failed to get next record info.
    GetNextRecordInfoError,

    /// Failed to resolve form ID.
    ResolveFormIdError,

    /// Failed to resolve handle.
    ResolveHandleError,
}

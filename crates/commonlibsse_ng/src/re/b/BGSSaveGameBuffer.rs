use core::ffi::c_void;
use core::ptr::NonNull;

use crate::re::InventoryEntryData::Actor;
use crate::re::TESForm::TESForm;
use crate::re::TESObjectREFR::TESObjectREFR;
use crate::re::offsets_rtti::RTTI_BGSSaveGameBuffer;
use crate::re::offsets_vtable::VTABLE_BGSSaveGameBuffer;
use crate::rel::id::VariantID;

#[derive(Debug)]
pub struct BGSSaveGameBuffer {
    pub vtable_: *const BGSSaveGameBufferVtbl,
    pub buffer: NonNull<c_void>,
    pub size: u32,
    pub buffer_position: u32,
}

const _: () = {
    assert!(core::mem::size_of::<BGSSaveGameBuffer>() == 0x18);
};

pub struct BGSSaveGameBufferVtbl {
    /// C++ virtual destructor
    pub delete: fn(this: *mut BGSSaveGameBuffer),

    pub get_form: fn(this: *const BGSSaveGameBuffer) -> *mut TESForm,
    pub get_reference: fn(this: *const BGSSaveGameBuffer) -> *mut TESObjectREFR,
    pub get_actor: fn(this: *const BGSSaveGameBuffer) -> *mut Actor,
}

impl BGSSaveGameBuffer {
    pub const RTTI: VariantID = RTTI_BGSSaveGameBuffer;
    pub const VTABLE: [VariantID; 1] = VTABLE_BGSSaveGameBuffer;

    /// C++ virtual destructor
    #[inline]
    pub fn delete(&mut self) {
        unsafe { ((*self.vtable_).delete)(self) }
    }

    #[inline]
    pub fn get_form(&self) -> *mut TESForm {
        unsafe { ((*self.vtable_).get_form)(self) }
    }

    #[inline]
    pub fn get_reference(&self) -> *mut TESObjectREFR {
        unsafe { ((*self.vtable_).get_reference)(self) }
    }

    #[inline]
    pub fn get_actor(&self) -> *mut Actor {
        unsafe { ((*self.vtable_).get_actor)(self) }
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 35163, ae_id = 36053)]
    pub fn save_data_endian(data: NonNull<c_void>, size: u32) {}
}

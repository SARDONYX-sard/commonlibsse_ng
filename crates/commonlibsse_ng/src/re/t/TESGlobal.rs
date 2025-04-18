//! # TESGlobal
//!
//! This module defines the `TESGlobal` struct, which inherits from `TESForm` and represents
//! global variables in Skyrim's engine. It includes a virtual table for C++ compatibility
//! and maintains the original memory layout.

use core::ffi::{CStr, c_float};

use crate::re::BSString::{BSString, BSStringError};
use crate::re::FormTypes::FormType;
use crate::re::TESForm::{TESForm, TESFormVtbl};
use crate::re::offsets_rtti::RTTI_TESGlobal;
use crate::re::offsets_vtable::VTABLE_TESGlobal;
use crate::rel::id::VariantID;

/// Represents a global variable in Skyrim.
#[repr(C)]
#[derive(Debug)]
pub struct TESGlobal {
    /// Base class `TESForm`.
    pub __base: TESForm,

    /// The form editor ID.
    /// - Offset: `0x20`
    pub form_editor_id: BSString,

    /// The type of the global (float, long, short).
    /// - Offset: `0x30`
    pub type_: Type_CEnum,

    /// Padding for alignment.
    /// - Offset: `0x31`
    pub pad31: u8,

    /// Additional padding.
    /// - Offset: `0x32`
    pub pad32: u16,

    /// The value of the global variable.
    /// - Offset: `0x34`
    pub value: c_float,
}

const _: () = {
    assert!(core::mem::offset_of!(TESGlobal, __base) == 0x0);
    assert!(core::mem::offset_of!(TESGlobal, form_editor_id) == 0x20);
    assert!(core::mem::offset_of!(TESGlobal, type_) == 0x30);
    assert!(core::mem::offset_of!(TESGlobal, pad31) == 0x31);
    assert!(core::mem::offset_of!(TESGlobal, pad32) == 0x32);
    assert!(core::mem::offset_of!(TESGlobal, value) == 0x34);
    assert!(core::mem::size_of::<TESGlobal>() == 0x38);
};

/// The global variable type.
#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Type {
    /// Stored type is [`f32`].
    Float = b'f',
    /// Stored type is [`i32`].
    Long = b'l',
    /// Stored type is [`i16`].
    Short = b's',
}

impl TESGlobal {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_TESGlobal;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_TESGlobal;

    pub const FORM_TYPE: FormType = FormType::Global;

    /// Gets the form editor ID as a `&str`.
    #[inline]
    pub const fn get_form_editor_id(&self) -> &CStr {
        self.form_editor_id.as_c_str()
    }

    /// Sets the form editor ID.
    ///
    /// # Errors
    /// - If the string is too long to fit in a `u16`, or if allocation fails.
    /// - If allocations fail.
    #[inline]
    pub fn set_form_editor_id(&mut self, id: &CStr) -> Result<(), BSStringError> {
        self.form_editor_id = BSString::from_c_str(id)?;
        Ok(())
    }
}

/// The virtual function table for `TESGlobal`.
#[repr(C)]
pub struct TESGlobalVtbl {
    __base: TESFormVtbl,
}

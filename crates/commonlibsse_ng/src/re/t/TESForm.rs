mod flags;
mod vtable;

pub use self::flags::{InGameFormFlag, RecordFlag};
pub use self::vtable::TESFormVtbl;

use crate::re::BSAtomic::BSReadWriteLock;
use crate::re::BSCoreTypes::FormID;
use crate::re::BSFixedString::BSFixedString;
use crate::re::BSTArray::BSStaticArray;
use crate::re::BSTHashMap::BSTHashMap;
use crate::re::BaseFormComponent::BaseFormComponent;
use crate::re::FormTypes::FormType;
use crate::re::TESFile::TESFile;
use crate::re::offsets_rtti::RTTI_TESForm;
use crate::re::offsets_vtable::VTABLE_TESForm;
use crate::rel::ResolvableAddress as _;
use crate::rel::id::RelocationID;
use crate::rel::id::{DataBaseError, VariantID};
use core::ffi::c_char;
use core::ptr::NonNull;
use std::sync::LazyLock;

#[repr(C)]
#[derive(Debug, PartialEq)]
struct TESFileArray {
    _base: BSStaticArray<TESFile>,
}
const_assert_eq!(core::mem::size_of::<TESFileArray>(), 0x10);

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct TESFileContainer {
    array: *mut TESFileArray,
}
const_assert_eq!(core::mem::size_of::<TESFileContainer>(), 0x8);

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct TESForm {
    pub __base: BaseFormComponent,
    pub sourceFiles: TESFileContainer,
    pub formFlags: u32,
    pub formID: FormID,
    pub inGameFormFlags: u16,
    pub formType: u8,
    pub pad1B: u8,
    pub pad1C: u32,
}
const_assert_eq!(core::mem::size_of::<TESForm>(), 0x20);

pub struct FormsLock<K> {
    pub map: NonNull<*mut BSTHashMap<K, *mut TESForm>>,
    pub lock: NonNull<BSReadWriteLock>,
}
unsafe impl<K> Send for FormsLock<K> {}
unsafe impl<K> Sync for FormsLock<K> {}

/// Pointer of `BSTHashMap<FormID, *mut TESForm>` & Pointer of `BSReadWriteLock`
type AllFormsIDLock = FormsLock<FormID>;

/// Pointer of `BSTHashMap<BSFixedString, *mut TESForm>` & Pointer of `BSReadWriteLock`
type AllFormsStringLock = FormsLock<BSFixedString>;

impl TESForm {
    pub const RTTI: VariantID = RTTI_TESForm;
    pub const VTABLE: [VariantID; 1] = VTABLE_TESForm;
    pub const FORM_TYPE: FormType = FormType::None;

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 14509, ae_id = 14667)]
    pub unsafe fn add_compile_index(id: FormID, file: TESFile) {}

    pub fn get_all_forms() -> &'static Result<AllFormsIDLock, DataBaseError> {
        static PTR_LOCK: LazyLock<Result<AllFormsIDLock, DataBaseError>> = LazyLock::new(|| {
            let map = RelocationID::from_se_ae_id(514351, 400507).address()?.cast();
            let lock = RelocationID::from_se_ae_id(514360, 400517).address()?.cast();
            Ok(AllFormsIDLock { map, lock })
        });
        &PTR_LOCK
    }

    pub fn get_all_by_editor_id() -> &'static Result<AllFormsStringLock, DataBaseError> {
        static PTR_LOCK: LazyLock<Result<AllFormsStringLock, DataBaseError>> =
            LazyLock::new(|| {
                let map = RelocationID::from_se_ae_id(514352, 400509).address()?.cast();
                let lock = RelocationID::from_se_ae_id(514361, 400518).address()?.cast();
                Ok(AllFormsStringLock { map, lock })
            });
        &PTR_LOCK
    }

    /// Search for TESForm based on FormID
    pub fn lookup_by_id(&self, form_id: FormID) -> Option<*mut Self> {
        let AllFormsIDLock { map, lock: _lock } = Self::get_all_forms().as_ref().ok()?;

        if let Some(map) = unsafe { map.as_ref().as_ref() } {
            if let Some(entry) = map.get(&form_id) {
                return Some(*entry);
            }
        }
        None
    }

    /// Dummy yet.
    #[allow(clippy::missing_const_for_fn)]
    pub fn get_name(&self) -> *const c_char {
        c"".as_ptr()
    }

    /// Dummy yet.
    #[allow(clippy::missing_const_for_fn)]
    pub fn is_deleted(&self) -> bool {
        false
    }
}

pub trait DerivedTESForm {
    fn get_form(&self) -> &TESForm;
    fn get_form_type() -> FormType;
}

impl DerivedTESForm for TESForm {
    #[inline]
    fn get_form(&self) -> &TESForm {
        self
    }

    #[inline]
    fn get_form_type() -> FormType {
        Self::FORM_TYPE
    }
}

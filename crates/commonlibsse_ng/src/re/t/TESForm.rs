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
use crate::re::FormTypes::{FormType, FormType_CEnum};
use crate::re::TESFile::TESFile;
use crate::re::offsets_rtti::RTTI_TESForm;
use crate::re::offsets_vtable::VTABLE_TESForm;
use crate::rel::ResolvableAddress as _;
use crate::rel::id::RelocationID;
use crate::rel::id::{DataBaseError, VariantID};
use core::ffi::CStr;
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
    pub inGameFormFlags: InGameFormFlag,
    pub formType: FormType_CEnum,
    pub pad1B: u8,
    pub pad1C: u32,
}
const_assert_eq!(core::mem::size_of::<TESForm>(), 0x20);

pub struct FormMapLockPtr<K>
where
    K: core::hash::Hash + Eq,
{
    pub map_ptr: NonNull<*mut BSTHashMap<K, Option<NonNull<TESForm>>>>,
    pub lock_ptr: NonNull<BSReadWriteLock>,
}
unsafe impl<K> Send for FormMapLockPtr<K> where K: core::hash::Hash + Eq {}
unsafe impl<K> Sync for FormMapLockPtr<K> where K: core::hash::Hash + Eq {}

/// Pointer of `BSTHashMap<FormID, *mut TESForm>` & Pointer of `BSReadWriteLock`
pub struct IDAllFormsMapPtr(FormMapLockPtr<FormID>);
impl IDAllFormsMapPtr {
    #[inline]
    pub fn get(&self, form_id: FormID) -> Option<NonNull<TESForm>> {
        let _guard = unsafe { self.0.lock_ptr.as_ref().read() };
        let map = unsafe { self.0.map_ptr.as_ref().as_ref()? };

        if let Some(form) = map.get(&form_id) {
            return *form;
        }
        None
    }
}

/// Pointer of `BSTHashMap<BSFixedString, *mut TESForm>` & Pointer of `BSReadWriteLock`
pub struct StringAllFormsMapPtr(FormMapLockPtr<BSFixedString>);
impl StringAllFormsMapPtr {
    #[inline]
    pub fn get(&self, editor_id: &CStr) -> Option<NonNull<TESForm>> {
        let _guard = unsafe { self.0.lock_ptr.as_ref().read() };
        let map = unsafe { self.0.map_ptr.as_ref().as_ref()? };

        if let Some(form) = map.get(&editor_id.into()) {
            return *form;
        }
        None
    }
}

impl TESForm {
    pub const RTTI: VariantID = RTTI_TESForm;
    pub const VTABLE: [VariantID; 1] = VTABLE_TESForm;
    pub const FORM_TYPE: FormType = FormType::None;

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 14509, ae_id = 14667)]
    pub unsafe fn add_compile_index(id: FormID, file: TESFile) {}

    pub fn get_all_forms() -> &'static Result<IDAllFormsMapPtr, DataBaseError> {
        static PTR_LOCK: LazyLock<Result<IDAllFormsMapPtr, DataBaseError>> = LazyLock::new(|| {
            let map_ptr = RelocationID::from_se_ae_id(514351, 400507).address()?.cast();
            let lock_ptr = RelocationID::from_se_ae_id(514360, 400517).address()?.cast();
            Ok(IDAllFormsMapPtr(FormMapLockPtr { map_ptr, lock_ptr }))
        });
        &PTR_LOCK
    }

    pub fn get_all_forms_by_editor_id() -> &'static Result<StringAllFormsMapPtr, DataBaseError> {
        static PTR_LOCK: LazyLock<Result<StringAllFormsMapPtr, DataBaseError>> =
            LazyLock::new(|| {
                Ok(StringAllFormsMapPtr(FormMapLockPtr {
                    map_ptr: RelocationID::from_se_ae_id(514352, 400509).address()?.cast(),
                    lock_ptr: RelocationID::from_se_ae_id(514361, 400518).address()?.cast(),
                }))
            });
        &PTR_LOCK
    }

    /// Search for TESForm based on FormID.
    #[inline]
    pub fn lookup_by_id(form_id: FormID) -> Option<NonNull<Self>> {
        let all_form_map = Self::get_all_forms().as_ref().ok()?;
        all_form_map.get(form_id)
    }

    /// Search for TESForm based on editor ID.
    #[inline]
    pub fn lookup_by_editor_id(editor_id: &CStr) -> Option<NonNull<Self>> {
        let all_form_map = Self::get_all_forms_by_editor_id().as_ref().ok()?;
        all_form_map.get(editor_id)
    }

    // #[deprecated = "The NG branch implementation of VR typecasts `TESForm` to `TESFullName`, which is invalid because there is no inheritance relationship."]
    // #[inline]
    // #[allow(clippy::missing_const_for_fn)]
    // pub fn get_name(&self) -> Option<&CStr> {
    //     // let fullname = unsafe { (self as *const Self).cast::<TESFullName>().as_ref() }?; // <- Invalid cast
    //     // Some(fullname.fullName.as_c_str())
    // }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 14809, ae_id = 14988)]
    pub fn get_weight(&self) -> f32 {}

    /// Dummy yet.
    #[allow(clippy::missing_const_for_fn)]
    pub fn is_deleted(&self) -> bool {
        false
    }
}

pub trait DerivedTESForm {
    const FORM_TYPE: FormType;

    fn get_form(&self) -> &TESForm;
}

impl DerivedTESForm for TESForm {
    const FORM_TYPE: FormType = FormType::None;

    #[inline]
    fn get_form(&self) -> &TESForm {
        self
    }
}

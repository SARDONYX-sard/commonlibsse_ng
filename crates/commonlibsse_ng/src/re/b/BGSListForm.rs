use core::ptr::NonNull;

use crate::re::BSContainer::ForEachResult;
use crate::re::BSCoreTypes::FormID;
use crate::re::BSTArray::BSTArray;
use crate::re::FormTypes::FormType;
use crate::re::TESForm::{TESForm, TESFormVtbl};
use crate::re::offsets_rtti::RTTI_BGSListForm;
use crate::re::offsets_vtable::VTABLE_BGSListForm;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct BGSListForm {
    pub __base: TESForm,                             // 0x00
    pub forms: BSTArray<*mut TESForm>,               // 0x20 - LNAM
    pub scriptAddedTempForms: *mut BSTArray<FormID>, // 0x38
    pub scriptAddedFormCount: u32,                   // 0x40
    pub pad44: u32,                                  // 0x44
}
const _: () = assert!(core::mem::size_of::<BGSListForm>() == 0x48);

impl BGSListForm {
    pub const RTTI: VariantID = RTTI_BGSListForm;
    pub const VTABLE: [VariantID; 1] = VTABLE_BGSListForm;
    pub const FORM_TYPE: FormType = FormType::FormList;

    pub const fn vtable(&self) -> *const BGSListFormVtbl {
        self.__base.__base.vtable.cast()
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 20470, ae_id = 20913)]
    pub unsafe fn add_form(&mut self, form: *mut TESForm) {}

    pub fn contains_only_type(&self, form_type: FormType) -> bool {
        let mut result = true;
        self.for_each_form(|form| {
            if form.formType.to_enum() != Some(form_type) {
                result = false;
                return ForEachResult::Stop;
            }
            ForEachResult::Continue
        });
        result
    }

    pub fn for_each_form<F>(&self, mut f: F)
    where
        F: FnMut(&TESForm) -> ForEachResult,
    {
        // 1. LNAM forms
        for &form in self.forms.as_slice() {
            if let Some(form) = NonNull::new(form) {
                if f(unsafe { form.as_ref() }) == ForEachResult::Stop {
                    return;
                }
            }
        }

        // 2. Script-added forms
        if !self.scriptAddedTempForms.is_null() {
            let added_forms = unsafe { &*self.scriptAddedTempForms };
            for &form_id in added_forms.as_slice() {
                let form = TESForm::lookup_by_id(form_id);
                if let Some(form) = form {
                    if f(unsafe { form.as_ref() }) == ForEachResult::Stop {
                        return;
                    }
                }
            }
        }
    }

    pub fn has_form_ptr(&self, form: NonNull<TESForm>) -> bool {
        for &f in self.forms.as_slice() {
            if f == form.as_ptr() {
                return true;
            }
        }

        if self.scriptAddedTempForms.is_null() {
            return false;
        }

        self.has_form_id((unsafe { form.as_ref() }).formID)
    }

    pub fn has_form_id(&self, form_id: FormID) -> bool {
        TESForm::lookup_by_id(form_id).is_some_and(|form| self.has_form_ptr(form))
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct BGSListFormVtbl {
    pub base: TESFormVtbl,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum ChangeFlag {
    DAddedForm = 1 << 31,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum RecordFlag {
    Deleted = 1 << 5,
    Ignored = 1 << 12,
}

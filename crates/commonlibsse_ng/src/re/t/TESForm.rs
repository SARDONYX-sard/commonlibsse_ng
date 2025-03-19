use crate::re::BGSLoadFormBuffer::BGSLoadFormBuffer;
use crate::re::BGSSaveFormBuffer::BGSSaveFormBuffer;
use crate::re::BSCoreTypes::FormID;
use crate::re::BSFixedString::BSFixedString;
use crate::re::BSTArray::BSStaticArray;
use crate::re::BaseFormComponent::BaseFormComponent;
use crate::re::FORM::{FORM, FORM_GROUP};
use crate::re::FormTypes::FormType;
use crate::re::TESBoundObject::TESBoundObject;
use crate::re::TESFile::TESFile;
use crate::re::TESObjectREFR::TESObjectREFR;
use crate::re::offsets_rtti::RTTI_TESForm;
use crate::re::offsets_vtable::VTABLE_TESForm;
use crate::rel::id::VariantID;
use core::ffi::c_char;

#[repr(C)]
#[derive(Debug)]
struct TESFileArray {
    _base: BSStaticArray<TESFile>,
}
const_assert_eq!(core::mem::size_of::<TESFileArray>(), 0x10);

#[repr(C)]
#[derive(Debug)]
pub struct TESFileContainer {
    array: *mut TESFileArray,
}
const_assert_eq!(core::mem::size_of::<TESFileContainer>(), 0x8);

#[repr(C)]
#[derive(Debug)]
pub struct TESForm {
    pub _base: BaseFormComponent,
    pub sourceFiles: TESFileContainer,
    pub formFlags: u32,
    pub formID: FormID,
    pub inGameFormFlags: u16,
    pub formType: u8,
    pub pad1B: u8,
    pub pad1C: u32,
}
const_assert_eq!(core::mem::size_of::<TESForm>(), 0x20);

impl TESForm {
    pub const RTTI: VariantID = RTTI_TESForm;
    pub const VTABLE: [VariantID; 1] = VTABLE_TESForm;
}

#[repr(C)]
#[derive(Debug)]
pub struct TESFormVtbl {
    pub destructor: unsafe extern "C" fn(this: *mut TESForm),

    // BaseFormComponent methods
    pub initialize_data_component: unsafe extern "C" fn(this: *mut TESForm),
    pub clear_data_component: unsafe extern "C" fn(this: *mut TESForm),
    pub copy_component: unsafe extern "C" fn(this: *mut TESForm, rhs: *const TESForm),

    // TESForm-specific methods
    pub initialize_data: unsafe extern "C" fn(this: *mut TESForm),
    pub clear_data: unsafe extern "C" fn(this: *mut TESForm),
    pub load: unsafe extern "C" fn(this: *mut TESForm, mod_: *mut TESFile) -> bool,
    pub load_partial: unsafe extern "C" fn(this: *mut TESForm, mod_: *mut TESFile) -> bool,
    pub load_edit: unsafe extern "C" fn(this: *mut TESForm, mod_: *mut TESFile) -> bool,
    pub create_duplicate_form: unsafe extern "C" fn(
        this: *mut TESForm,
        create_editor_id: bool,
        arg2: *mut std::ffi::c_void,
    ) -> *mut TESForm,

    pub add_change: unsafe extern "C" fn(this: *mut TESForm, change_flags: u32) -> bool,
    pub remove_change: unsafe extern "C" fn(this: *mut TESForm, change_flags: u32),

    pub find_in_file_fast: unsafe extern "C" fn(this: *mut TESForm, mod_: *mut TESFile) -> bool,
    pub check_save_game:
        unsafe extern "C" fn(this: *mut TESForm, buf: *mut BGSSaveFormBuffer) -> bool,
    pub save_game: unsafe extern "C" fn(this: *mut TESForm, buf: *mut BGSSaveFormBuffer),
    pub load_game: unsafe extern "C" fn(this: *mut TESForm, buf: *mut BGSLoadFormBuffer),

    pub init_load_game: unsafe extern "C" fn(this: *mut TESForm, buf: *mut BGSLoadFormBuffer),
    pub finish_load_game: unsafe extern "C" fn(this: *mut TESForm, buf: *mut BGSLoadFormBuffer),
    pub revert: unsafe extern "C" fn(this: *mut TESForm, buf: *mut BGSLoadFormBuffer),

    pub init_item_impl: unsafe extern "C" fn(this: *mut TESForm),

    pub get_description_owner_file: unsafe extern "C" fn(this: *const TESForm) -> *mut TESFile,
    pub get_saved_form_type: unsafe extern "C" fn(this: *const TESForm) -> FormType,
    pub get_form_detailed_string:
        unsafe extern "C" fn(this: *const TESForm, buf: *mut c_char, buf_len: u32),

    pub get_known: unsafe extern "C" fn(this: *const TESForm) -> bool,
    pub get_random_anim: unsafe extern "C" fn(this: *const TESForm) -> bool,
    pub get_playable: unsafe extern "C" fn(this: *const TESForm) -> bool,

    pub is_heading_marker: unsafe extern "C" fn(this: *const TESForm) -> bool,
    pub get_dangerous: unsafe extern "C" fn(this: *const TESForm) -> bool,

    pub q_has_currents: unsafe extern "C" fn(this: *const TESForm) -> bool,
    pub get_obstacle: unsafe extern "C" fn(this: *const TESForm) -> bool,

    pub q_is_lod_land_object: unsafe extern "C" fn(this: *const TESForm) -> bool,
    pub get_on_local_map: unsafe extern "C" fn(this: *const TESForm) -> bool,
    pub get_must_update: unsafe extern "C" fn(this: *const TESForm) -> bool,

    pub set_on_local_map: unsafe extern "C" fn(this: *mut TESForm, set: bool),

    pub get_ignored_by_sandbox: unsafe extern "C" fn(this: *const TESForm) -> bool,
    pub set_delete: unsafe extern "C" fn(this: *mut TESForm, set: bool),
    pub set_altered: unsafe extern "C" fn(this: *mut TESForm, set: bool),

    pub save_object_bound: unsafe extern "C" fn(this: *mut TESForm),
    pub load_object_bound: unsafe extern "C" fn(this: *mut TESForm, mod_: *mut TESFile),

    pub is_bound_object: unsafe extern "C" fn(this: *const TESForm) -> bool,
    pub is_object: unsafe extern "C" fn(this: *const TESForm) -> bool,

    pub is_magic_item: unsafe extern "C" fn(this: *const TESForm) -> bool,
    pub is_water: unsafe extern "C" fn(this: *const TESForm) -> bool,

    pub as_reference1: unsafe extern "C" fn(this: *mut TESForm) -> *mut TESObjectREFR,
    pub as_reference2: unsafe extern "C" fn(this: *const TESForm) -> *const TESObjectREFR,

    pub get_ref_count: unsafe extern "C" fn(this: *const TESForm) -> u32,

    pub get_text_for_parsed_sub_tag:
        unsafe extern "C" fn(this: *const TESForm, tag: *const BSFixedString) -> *const c_char,

    pub copy: unsafe extern "C" fn(this: *mut TESForm, src: *const TESForm),

    pub belongs_in_group: unsafe extern "C" fn(
        this: *mut TESForm,
        form: *mut FORM,
        allow_parent_groups: bool,
        current_only: bool,
    ) -> bool,
    pub create_group_data:
        unsafe extern "C" fn(this: *mut TESForm, form: *mut FORM, group: *mut FORM_GROUP),

    pub get_form_editor_id: unsafe extern "C" fn(this: *const TESForm) -> *const c_char,
    pub set_form_editor_id: unsafe extern "C" fn(this: *mut TESForm, str: *const c_char) -> bool,

    pub is_parent_form: unsafe extern "C" fn(this: *const TESForm) -> bool,
    pub is_parent_form_tree: unsafe extern "C" fn(this: *const TESForm) -> bool,
    pub is_form_type_child: unsafe extern "C" fn(this: *const TESForm, form_type: FormType) -> bool,
    pub activate: unsafe extern "C" fn(
        this: *mut TESForm,
        target_ref: *mut TESObjectREFR,
        activator_ref: *mut TESObjectREFR,
        arg3: u8,
        object: *mut TESBoundObject,
        target_count: i32,
    ) -> bool,
}

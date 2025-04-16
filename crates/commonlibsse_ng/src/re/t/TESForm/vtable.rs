use super::TESForm;
use crate::re::BGSLoadFormBuffer::BGSLoadFormBuffer;
use crate::re::BGSSaveFormBuffer::BGSSaveFormBuffer;
use crate::re::BSCoreTypes::FormID;
use crate::re::BSFixedString::BSFixedString;
use crate::re::FORM::{FORM, FORM_GROUP};
use crate::re::FormTypes::FormType;
use crate::re::TESBoundObject::TESBoundObject;
use crate::re::TESFile::TESFile;
use crate::re::TESObjectREFR::TESObjectREFR;
use core::ffi::c_char;

/// Virtual function table for `TESForm`
#[repr(C)]
#[derive(Debug)]
pub struct TESFormVtbl {
    /// # C++
    /// - method nth: 0x00
    /// - Destructor `~TESForm`
    pub CxxDrop: fn(this: &mut TESForm),

    /// # C++
    /// - method nth: 0x01
    /// - { return; }
    pub InitializeDataComponent: fn(this: &mut TESForm),

    /// # C++
    /// - method nth: 0x02
    /// - { SetEditorID(""); }
    pub ClearDataComponent: fn(this: &mut TESForm),

    /// # C++
    /// - method nth: 0x03
    pub CopyComponent: fn(this: &mut TESForm, rhs: *const TESForm),

    /// # C++
    /// - method nth: 0x04
    /// - { return; }
    pub InitializeData: fn(this: &mut TESForm),

    /// # C++
    /// - method nth: 0x05
    /// - { return; }
    pub ClearData: fn(this: &mut TESForm),

    /// # C++
    /// - method nth: 0x06
    /// - { return true; }
    pub Load: fn(this: &mut TESForm, mod_file: *mut TESFile) -> bool,

    /// # C++
    /// - method nth: 0x07
    /// - { return true; }
    pub LoadPartial: fn(this: &mut TESForm, mod_file: *mut TESFile) -> bool,

    /// # C++
    /// - method nth: 0x08
    /// - { return Load(mod); }
    pub LoadEdit: fn(this: &mut TESForm, mod_file: *mut TESFile) -> bool,

    /// # C++
    /// - method nth: 0x09
    pub CreateDuplicateForm:
        fn(this: &mut TESForm, create_editor_id: bool, arg2: *mut ()) -> *mut TESForm,

    /// # C++
    /// - method nth: 0x0A
    pub AddChange: fn(this: &mut TESForm, change_flags: u32) -> bool,

    /// # C++
    /// - method nth: 0x0B
    pub RemoveChange: fn(this: &mut TESForm, change_flags: u32),

    /// # C++
    /// - method nth: 0x0C
    /// - { return false; }
    pub FindInFileFast: fn(this: &mut TESForm, mod_file: *mut TESFile) -> bool,

    /// # C++
    /// - method nth: 0x0D
    /// - { return true; }
    pub CheckSaveGame: fn(this: &mut TESForm, buf: *mut BGSSaveFormBuffer) -> bool,

    /// # C++
    /// - method nth: 0x0E
    pub SaveGame: fn(this: &TESForm, buf: *mut BGSSaveFormBuffer),

    /// # C++
    /// - method nth: 0x0F
    pub LoadGame: fn(this: &mut TESForm, buf: *mut BGSLoadFormBuffer),

    /// # C++
    /// - method nth: 0x10
    /// - { return; }
    pub InitLoadGame: fn(this: &mut TESForm, buf: *mut BGSLoadFormBuffer),

    /// # C++
    /// - method nth: 0x11
    /// - { return; }
    pub FinishLoadGame: fn(this: &mut TESForm, buf: *mut BGSLoadFormBuffer),

    /// # C++
    /// - method nth: 0x12
    /// - { return; }
    pub Revert: fn(this: &mut TESForm, buf: *mut BGSLoadFormBuffer),

    /// # C++
    /// - method nth: 0x13
    /// - { return; }
    pub InitItemImpl: fn(this: &mut TESForm),

    /// # C++
    /// - method nth: 0x14
    pub GetDescriptionOwnerFile: fn(this: &TESForm) -> *mut TESFile,

    /// # C++
    /// - method nth: 0x15
    /// - { return formType; }
    pub GetSavedFormType: fn(this: &TESForm) -> FormType,

    /// # C++
    /// - method nth: 0x16
    pub GetFormDetailedString: fn(this: &TESForm, buf: *mut c_char, buf_len: u32),

    /// # C++
    /// - method nth: 0x17
    /// - { return (flags >> 6) & 1; }
    pub GetKnown: fn(this: &TESForm) -> bool,

    /// # C++
    /// - method nth: 0x18
    /// - { return (flags >> 16) & 1; }
    pub GetRandomAnim: fn(this: &TESForm) -> bool,

    /// # C++
    /// - method nth: 0x19
    /// - { return (flags >> 2) & 1; }
    pub GetPlayable: fn(this: &TESForm) -> bool,

    /// # C++
    /// - method nth: 0x1A
    /// - { return false; }
    pub IsHeadingMarker: fn(this: &TESForm) -> bool,

    /// # C++
    /// - method nth: 0x1B
    /// - { return (flags >> 17) & 1; }
    pub GetDangerous: fn(this: &TESForm) -> bool,

    /// # C++
    /// - method nth: 0x1C
    /// - { return (flags >> 19) & 1; }
    pub QHasCurrents: fn(this: &TESForm) -> bool,

    /// # C++
    /// - method nth: 0x1D
    /// - { return (flags >> 25) & 1; }
    pub GetObstacle: fn(this: &TESForm) -> bool,

    /// # C++
    /// - method nth: 0x1E
    /// - { return false; }
    pub QIsLODLandObject: fn(this: &TESForm) -> bool,

    /// # C++
    /// - method nth: 0x1F
    /// - { return (flags >> 9) & 1; }
    pub GetOnLocalMap: fn(this: &TESForm) -> bool,

    /// # C++
    /// - method nth: 0x20
    /// - { return (flags >> 8) & 1; }
    pub GetMustUpdate: fn(this: &TESForm) -> bool,

    /// # C++
    /// - method nth: 0x21
    /// - { if (set) flags &= 0xFFFFFDFF; else flags |= 0x200; }
    pub SetOnLocalMap: fn(this: &mut TESForm, set: bool),

    /// # C++
    /// - method nth: 0x22
    /// - { return false; }
    pub GetIgnoredBySandbox: fn(this: &TESForm) -> bool,

    /// # C++
    /// - method nth: 0x23
    /// - { bool result = (flags >> 5) & 1; if (result != set) { if (set) flags |= 0x20; else flags &= 0xFFFFFFDF; AddChange(1); return result; }
    pub SetDelete: fn(this: &mut TESForm, set: bool),

    /// # C++
    /// - method nth: 0x24
    pub SetAltered: fn(this: &mut TESForm, set: bool),

    /// # C++
    /// - method nth: 0x25
    /// - { return; }
    pub SaveObjectBound: fn(this: &mut TESForm),

    /// # C++
    /// - method nth: 0x26
    /// - { return; }
    pub LoadObjectBound: fn(this: &mut TESForm, mod_: *mut TESFile),

    /// # C++
    /// - method nth: 0x27
    /// - { return false; }
    pub IsBoundObject: fn(this: &TESForm) -> bool,

    /// # C++
    /// - method nth: 0x28
    /// - { return false; }
    pub IsObject: fn(this: &TESForm) -> bool,

    /// # C++
    /// - method nth: 0x29
    /// - { return false; }
    pub IsMagicItem: fn(this: &TESForm) -> bool,

    /// # C++
    /// - method nth: 0x2A
    /// - { return false; }
    pub IsWater: fn(this: &TESForm) -> bool,

    /// # C++
    /// - method nth: 0x2B
    /// - { return 0; }
    pub AsReference1: fn(this: &mut TESForm) -> *mut TESObjectREFR,

    /// # C++
    /// - method nth: 0x2C
    /// - { return 0; }
    pub AsReference2: fn(this: &TESForm) -> *const TESObjectREFR,

    /// # C++
    /// - method nth: 0x2D
    /// - { return 0; }
    pub GetRefCount: fn(this: &TESForm) -> u32,

    /// # C++
    /// - method nth: 0x2E
    pub GetTextForParsedSubTag: fn(this: &TESForm, tag: &BSFixedString) -> *const c_char,

    /// # C++
    /// - method nth: 0x2F
    /// - { return; }
    pub Copy: fn(this: &mut TESForm, src_form: *const TESForm),

    /// # C++
    /// - method nth: 0x30
    pub BelongsInGroup: fn(
        this: &TESForm,
        form: *const FORM,
        allow_parent_groups: bool,
        current_only: bool,
    ) -> bool,

    /// # C++
    /// - method nth: 0x31
    pub CreateGroupData: fn(this: &TESForm, form: *const FORM, group: *mut FORM_GROUP),

    /// # C++
    /// - method nth: 0x32
    /// - { return ""; }
    pub GetFormEditorID: fn(this: &TESForm) -> *const c_char,

    /// # C++
    /// - method nth: 0x33
    /// - { return true; }
    pub SetFormEditorID: fn(this: &mut TESForm, str: *const c_char) -> bool,

    /// # C++
    /// - method nth: 0x34
    /// - { return false; }
    pub IsParentForm: fn(this: &TESForm) -> bool,

    /// # C++
    /// - method nth: 0x35
    /// - { return false; }
    pub IsParentFormTree: fn(this: &TESForm) -> bool,

    /// # C++
    /// - method nth: 0x36
    /// - { return false; }
    pub IsFormTypeChild: fn(this: &TESForm, type_: FormType) -> bool,

    /// # C++
    /// - method nth: 0x37
    /// - { return false; }
    pub Activate: fn(
        this: &mut TESForm,
        target_ref: *mut TESObjectREFR,
        activator_ref: *mut TESObjectREFR,
        arg3: u8,
        object: *mut TESBoundObject,
        target_count: i32,
    ) -> bool,

    /// # C++
    /// - method nth: 0x38
    pub SetFormID: fn(this: &mut TESForm, id: FormID, update_file: bool),

    /// # C++
    /// - method nth: 0x39
    /// - { return ""; }
    pub GetObjectTypeName: fn(this: &TESForm) -> *const c_char,

    /// # C++
    /// - method nth: 0x3A
    /// - { return true; }
    pub QAvailableInGame: fn(this: &TESForm) -> bool,
}
const _: () = {
    const VFUNC_COUNT: usize = 0x3A + 1;

    const EXPECTED_SIZE: usize = VFUNC_COUNT * core::mem::size_of::<usize>();
    const ACTUAL_SIZE: usize = core::mem::size_of::<TESFormVtbl>();
    assert!(ACTUAL_SIZE == EXPECTED_SIZE);
};

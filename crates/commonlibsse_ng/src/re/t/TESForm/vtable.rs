use crate::re::BGSLoadFormBuffer::BGSLoadFormBuffer;
use crate::re::BGSSaveFormBuffer::BGSSaveFormBuffer;
use crate::re::FormTypes::FormType;
use crate::re::TESFile::TESFile;
use super::TESForm;
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
    /// - { return Load(a_mod); }
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
}

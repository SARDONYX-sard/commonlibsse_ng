use crate::re::BGSLoadFormBuffer::BGSLoadFormBuffer;
use crate::re::BGSSaveFormBuffer::BGSSaveFormBuffer;
use crate::re::FormTypes::FormType;
use crate::re::TESFile::TESFile;
use crate::re::TESForm::{DerivedTESForm, TESForm};
use crate::re::offsets_rtti::RTTI_BGSEncounterZone;
use crate::re::offsets_vtable::VTABLE_BGSEncounterZone;
use crate::re::{BGSLocation, TESFaction};
use crate::rel::id::VariantID;
use std::ptr;

/// Represents the `ENCOUNTER_ZONE_DATA` structure.
#[repr(C)]
pub struct ENCOUNTER_ZONE_DATA {
    /// Pointer to `TESFaction`.
    pub zone_owner: *mut TESFaction, // 00

    /// Pointer to `BGSLocation`.
    pub location: *mut BGSLocation, // 08

    /// Owner rank.
    pub owner_rank: i8, // 10

    /// Minimum level.
    pub min_level: i8, // 11

    /// Encounter zone flags.
    pub flags: Flags, // 12

    /// Maximum level.
    pub max_level: i8, // 13

    /// Padding to align the structure with C++ layout.
    pub pad14: u32, // 14
}

const _: () = {
    assert!(core::mem::size_of::<ENCOUNTER_ZONE_DATA>() == 0x18);
};

impl Default for ENCOUNTER_ZONE_DATA {
    #[inline]
    fn default() -> Self {
        Self {
            zone_owner: ptr::null_mut(),
            location: ptr::null_mut(),
            owner_rank: 0,
            min_level: 0,
            flags: Flags::empty(),
            max_level: 0,
            pad14: 0,
        }
    }
}

bitflags::bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Flags: u8 {
        const None = 0;
        const NEVER_RESETS = 1 << 0;
        const MATCH_PC_BELOW_MINIMUM_LEVEL = 1 << 1;
        const DISABLE_COMBAT_BOUNDARY = 1 << 2;
    }
}

/// Represents the `ENCOUNTER_ZONE_GAME_DATA` structure.
#[repr(C)]
pub struct ENCOUNTER_ZONE_GAME_DATA {
    /// Detach time.
    pub detach_time: u32, // 00

    /// Attach time.
    pub attach_time: u32, // 04

    /// Reset time.
    pub reset_time: u32, // 08

    /// Zone level.
    pub zone_level: u16, // 0C

    /// Padding.
    pub pad0d: u16, // 0E
}

const _: () = {
    assert!(core::mem::size_of::<ENCOUNTER_ZONE_GAME_DATA>() == 0x10);
};

impl Default for ENCOUNTER_ZONE_GAME_DATA {
    #[inline]
    fn default() -> Self {
        Self { detach_time: 0, attach_time: 0, reset_time: 0, zone_level: 0, pad0d: 0 }
    }
}

/// Represents the `BGSEncounterZone` class.
#[repr(C)]
pub struct BGSEncounterZone {
    /// Base class `TESForm`.
    pub __base: TESForm,

    /// Encounter zone data.
    pub data: ENCOUNTER_ZONE_DATA, // 20

    /// Encounter zone game data.
    pub game_data: ENCOUNTER_ZONE_GAME_DATA, // 38
}

const _: () = {
    assert!(core::mem::offset_of!(BGSEncounterZone, __base) == 0x0);
    assert!(core::mem::offset_of!(BGSEncounterZone, data) == 0x20);
    assert!(core::mem::offset_of!(BGSEncounterZone, game_data) == 0x38);
    assert!(core::mem::size_of::<BGSEncounterZone>() == 0x48);
};

bitflags::bitflags! {
    /// - Maybe unused
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ChangeFlags: u32 {
        const ZONE_FLAGS = 1 << 1;
        const GAME_DATA = 1 << 31;
    }

    /// - Maybe unused
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

impl DerivedTESForm for BGSEncounterZone {
    const FORM_TYPE: FormType = Self::FORM_TYPE;

    #[inline]
    fn get_form(&self) -> &TESForm {
        &self.__base
    }
}

impl BGSEncounterZone {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_BGSEncounterZone;

    /// Address & offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_BGSEncounterZone;

    /// The `FormType` value for encounter zones.
    pub const FORM_TYPE: FormType = FormType::EncounterZone;
}

/// The virtual function table for `BGSEncounterZone`.
#[repr(C)]
pub struct BGSEncounterZoneVtbl {
    /// Destructor function pointer.
    pub CxxDrop: fn(this: &mut BGSEncounterZone),

    /// Function pointer for `InitializeData`.
    pub InitializeData: fn(this: &mut BGSEncounterZone),

    /// Function pointer for `Load`.
    pub Load: fn(this: &mut BGSEncounterZone, mod_file: *mut TESFile) -> bool,

    /// Function pointer for `SaveGame`.
    pub SaveGame: fn(this: &BGSEncounterZone, buf: *mut BGSSaveFormBuffer),

    /// Function pointer for `LoadGame`.
    pub LoadGame: fn(this: &mut BGSEncounterZone, buf: *mut BGSLoadFormBuffer),

    /// Function pointer for `Revert`.
    pub Revert: fn(this: &mut BGSEncounterZone, buf: *mut BGSLoadFormBuffer),

    /// Function pointer for `InitItemImpl`.
    pub InitItemImpl: fn(this: &mut BGSEncounterZone),
}

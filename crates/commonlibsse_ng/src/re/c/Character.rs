use core::ffi::c_void;

use crate::re::Actor::{Actor, ActorVtbl};
use crate::re::FormTypes::FormType;
use crate::re::offsets_rtti::RTTI_Character;
use crate::re::offsets_vtable::VTABLE_Character;
use crate::rel::id::VariantID;

#[commonlibsse_ng_derive_internal::to_bitflags]
#[repr(u32)]
enum RecordFlag {
    Deleted = 1 << 5,
    StartsDead = 1 << 9,
    Persistent = 1 << 10,
    InitiallyDisabled = 1 << 11,
    Ignored = 1 << 12,
    NoAIAcquire = 1 << 25,
    DontHavokSettle = 1 << 29,
}

#[repr(C)]
pub struct Character {
    pub __base: Actor,
}

impl Character {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_Character;

    /// Address & offset of the virtual function table.
    ///
    /// The number of tables is the same as the number of classes with inherited virtual functions.
    pub const VTABLE: [VariantID; 10] = VTABLE_Character;

    /// The `FormType` value for Character.
    pub const FORM_TYPE: FormType = FormType::ActorCharacter;
}

pub struct CharacterVtbl {
    pub __base: ActorVtbl,
    pub Unk_128: extern "C" fn(this: *mut Character, c_void) -> c_void,
    pub Unk_129: extern "C" fn(this: *mut Character, c_void) -> c_void,
}

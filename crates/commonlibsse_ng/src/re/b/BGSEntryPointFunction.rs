use core::ffi::{c_char, c_void};
use core::ptr::NonNull;

use crate::re::TESObjectREFR::TESObjectREFR;

#[commonlibsse_ng_derive_internal::ffi_enum]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ENTRY_POINT_FUNCTION {
    NullFunction = 0,
    SetValue = 1,
    AddValue = 2,
    MultiplyValue = 3,
    AddRangeToValue = 4,
    AddActorValueMult = 5,
    AbsoluteValue = 6,
    NegativeAbsoluteValue = 7,
    AddLeveledList = 8,
    AddActivateChoice = 9,
    SelectSpell = 10,
    SelectText = 11,
    SetToActorValueMult = 12,
    MultiplyActorValueMult = 13,
    MultiplyOnePlusActorValueMult = 14,
    SetText = 15,
}

#[commonlibsse_ng_derive_internal::ffi_enum]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ENTRY_POINT_FUNCTION_TYPE {
    Value = 0,
    AddLeveledList = 1,
    AddActivateChoice = 2,
    Null = 3,
    SelectSpell = 4,
    SelectText = 5,
    SetText = 6,
}

type Function =
    fn(repr: TESObjectREFR, type_: ENTRY_POINT_FUNCTION_TYPE, u8, *mut *mut c_void, *mut c_void);

#[repr(C)]
#[derive(Debug)]
pub struct EntryPointFunction {
    name: *const c_char,              // 0x00
    type_: ENTRY_POINT_FUNCTION_TYPE, // 0x08
    pad0C: u32,                       // 0x0C
    function: Function,               // 0x10
}
const _: () = assert!(core::mem::size_of::<EntryPointFunction>() == 0x18);

impl EntryPointFunction {
    /// Gets `Self` from `ENTRY_POINT_FUNCTION`. for SE, VR.
    ///
    /// # Panics
    /// - If Runtime is AE then panic.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut EntryPointFunction",
        default = "None",
        id(se = 369178, ae = 0)
    )]
    #[inline]
    pub fn from_entry_point_function(
        entry_point_function: ENTRY_POINT_FUNCTION,
    ) -> Option<NonNull<EntryPointFunction>> {
        |as_type: AsType| unsafe { NonNull::new(as_type.add(entry_point_function as usize)) }
    }
}

/// Gets argument count from `ENTRY_POINT_FUNCTION_TYPE`. for SE, VR.
///
/// # Panics
/// If Runtime is AE then panic.
#[commonlibsse_ng_derive_internal::relocate(
    cast_as = "*mut u32",
    default = "None",
    id(se = 369210, ae = 0)
)]
#[inline]
pub fn get_argument_count(entry_point_function_type: ENTRY_POINT_FUNCTION_TYPE) -> Option<u32> {
    |as_type: AsType| unsafe { as_type.add(entry_point_function_type as usize).as_ref().copied() }
}

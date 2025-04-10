mod function_id;

pub use function_id::FunctionData;

use crate::re::BSPointerHandle::ObjectRefHandle;
use crate::re::TESGlobal::TESGlobal;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    /// - `==`: EqualTo
    Eq,
    /// - `!=`: NotEqualTo
    Ne,
    /// - `>`: GreaterThan
    Gt,
    /// - `>=`: GreaterThanOrEqualTo
    Ge,
    /// - `<`: LessThan
    Lt,
    /// - `<=`: LessThanOrEqualTo
    Le,
}

#[repr(C)]
pub union GlobalOrFloat {
    pub g: *mut TESGlobal,
    pub f: f32,
}
const _: () = assert!(std::mem::size_of::<GlobalOrFloat>() == 0x8);

bitflags::bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Flags: u8 {
        /// 0 - false == AND, true == OR
        const IS_OR         = 0b00000001;
        const USES_ALIASES  = 0b00000010;
        const GLOBAL        = 0b00000100;
        const USE_PACK_DATA = 0b00001000;
        const SWAP_TARGET   = 0b00010000;
        // OpCode uses remaining 3 bits: 0b11100000
    }
}

impl Flags {
    /// Gets operation code from bits.
    #[inline]
    pub const fn op_code(&self) -> Option<OpCode> {
        // Extract the 3-bit opcode from bits 5-7
        Some(match (self.bits() >> 5) & 0b111 {
            0 => OpCode::Eq,
            1 => OpCode::Ne,
            2 => OpCode::Gt,
            3 => OpCode::Ge,
            4 => OpCode::Lt,
            5 => OpCode::Le,
            _ => return None, // invalid OpCode
        })
    }
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConditionItemObject {
    #[default]
    Self_ = 0,
    Target = 1,
    Ref = 2,
    CombatTarget = 3,
    LinkedRef = 4,
    QuestAlias = 5,
    PackData = 6,
    EventData = 7,
    CommandTarget = 8,
}

// #[derive(Debug)]
#[repr(C)]
pub struct CONDITION_ITEM_DATA {
    pub comparison_value: GlobalOrFloat, // 0x08
    pub run_on_ref: ObjectRefHandle,     // 0x10
    pub data_id: u32,                    // 0x14
    pub function_data: FunctionData,     // 0x18
    pub flags: Flags,                    // 0x30
    pub object: ConditionItemObject,     // 0x31
    pub pad32: u16,                      // 0x32
    pub pad34: u32,                      // 0x34
}
const _: () = assert!(core::mem::size_of::<CONDITION_ITEM_DATA>() == 0x30);

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct TESCondition {
    pub head: *mut TESConditionItem, // 0x00
}
const _: () = assert!(core::mem::size_of::<TESCondition>() == 0x8);

impl TESCondition {}

// #[derive(Debug)]
#[repr(C)]
pub struct TESConditionItem {
    next: *mut TESConditionItem, // 0x0
    data: CONDITION_ITEM_DATA,   // 0x8
}
const _: () = assert!(core::mem::size_of::<TESConditionItem>() == 0x38);

#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum PERK_ENTRY_TYPE {
    Quest = 0,
    Ability = 1,
    EntryPoint = 2,
}

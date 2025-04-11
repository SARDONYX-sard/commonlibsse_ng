mod function_id;
pub use function_id::FunctionData;

use crate::re::BSPointerHandle::ObjectRefHandle;
use crate::re::TESGlobal::TESGlobal;
use crate::re::TESObjectREFR::TESObjectREFR;
use crate::re::TESQuest::TESQuest;
use core::ffi::c_void;
use core::ptr::NonNull;
use core::{fmt, ptr};

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OpCode {
    /// - EqualTo: `==`
    #[default]
    Eq,
    /// - NotEqualTo: `!=`
    Ne,
    /// - GreaterThan: `>`
    Gt,
    /// - GreaterThanOrEqualTo: `>=`
    Ge,
    /// - LessThan: `<`
    Lt,
    /// - LessThanOrEqualTo: `<=`
    Le,
}

/// Either is resolved by [`Flags`]
#[repr(C)]
pub union GlobalOrFloat {
    pub g: *mut TESGlobal,
    pub f: f32,
}
const _: () = assert!(std::mem::size_of::<GlobalOrFloat>() == 0x8);

impl GlobalOrFloat {
    #[inline]
    pub const fn new() -> Self {
        Self { g: ptr::null_mut() }
    }
}

/// `GlobalOrFloat` after either is resolved by Flags
#[derive(Debug)]
pub enum ComparisonValue {
    Global(*mut TESGlobal),
    Float(f32),
}

impl Default for GlobalOrFloat {
    #[inline]
    fn default() -> Self {
        Self { g: ptr::null_mut() }
    }
}

bitflags::bitflags! {
    #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    #[inline]
    pub const fn new() -> Self {
        Self::empty()
    }

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

impl fmt::Debug for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = vec![];

        if self.contains(Self::IS_OR) {
            flags.push("IS_OR");
        }
        if self.contains(Self::USES_ALIASES) {
            flags.push("USES_ALIASES");
        }
        if self.contains(Self::GLOBAL) {
            flags.push("GLOBAL");
        }
        if self.contains(Self::USE_PACK_DATA) {
            flags.push("USE_PACK_DATA");
        }
        if self.contains(Self::SWAP_TARGET) {
            flags.push("SWAP_TARGET");
        }

        if let Some(op) = self.op_code() {
            flags.push(match op {
                OpCode::Eq => "Op::Eq",
                OpCode::Ne => "Op::Ne",
                OpCode::Gt => "Op::Gt",
                OpCode::Ge => "Op::Ge",
                OpCode::Lt => "Op::Lt",
                OpCode::Le => "Op::Le",
            });
        } else {
            flags.push("Op::Invalid");
        }

        f.debug_tuple("Flags").field(&flags.join(" | ")).finish()
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

#[derive(Default)]
#[repr(C)]
pub struct CONDITION_ITEM_DATA {
    pub comparisonValue: GlobalOrFloat, // 0x08
    pub runOnRef: ObjectRefHandle,      // 0x10
    pub dataId: u32,                    // 0x14
    pub functionData: FunctionData,     // 0x18
    pub flags: Flags,                   // 0x30
    pub object: ConditionItemObject,    // 0x31
    pub pad32: u16,                     // 0x32
    pub pad34: u32,                     // 0x34
}
const _: () = assert!(core::mem::size_of::<CONDITION_ITEM_DATA>() == 0x30);

impl CONDITION_ITEM_DATA {
    #[inline]
    pub const fn new() -> Self {
        Self {
            comparisonValue: GlobalOrFloat::new(),
            runOnRef: ObjectRefHandle::new(),
            dataId: 0,
            functionData: FunctionData::new(),
            flags: Flags::new(),
            object: ConditionItemObject::Self_,
            pad32: 0,
            pad34: 0,
        }
    }

    /// Type convert unions that are unsafe to access to safe enums.
    #[inline]
    pub const fn comparison_value(&self) -> ComparisonValue {
        unsafe {
            if self.flags.contains(Flags::GLOBAL) {
                ComparisonValue::Global(self.comparisonValue.g)
            } else {
                ComparisonValue::Float(self.comparisonValue.f)
            }
        }
    }
}

impl fmt::Debug for CONDITION_ITEM_DATA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CONDITION_ITEM_DATA")
            .field("comparisonValue", &self.comparison_value())
            .field("runOnRef", &self.runOnRef)
            .field("dataId", &self.dataId)
            .field("functionData", &self.functionData)
            .field("flags", &self.flags)
            .field("object", &self.object)
            .finish()
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct TESCondition {
    pub head: Option<NonNull<TESConditionItem>>, // 0x00
}
const _: () = assert!(core::mem::size_of::<TESCondition>() == 0x8);

impl TESCondition {
    #[inline]
    pub const fn new() -> Self {
        Self { head: None }
    }

    /// Is the single directional link list empty?
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Perhaps indicating the equivalence of two objects.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 29074, ae_id = 29888)]
    pub fn is_true(&self, action_ref: *mut TESObjectREFR, target_ref: *mut TESObjectREFR) -> bool {}
}

impl Default for TESCondition {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for TESCondition {
    fn drop(&mut self) {
        let mut cur = self.head;
        while let Some(cur_ptr) = cur {
            unsafe {
                let next = cur_ptr.as_ref().next;
                drop(Box::from_raw(cur_ptr.as_ptr()));
                cur = next;
            }
        }
        self.head = None;
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct ConditionCheckParams {
    actionRef: *mut TESObjectREFR,            // 0x00
    targetRef: *mut TESObjectREFR,            // 0x08
    quest: *mut TESQuest,                     // 0x10
    questStartEvent: *mut BGSStoryEvent,      // 0x18
    unk20: *mut c_void,                       // 0x20
    unk28: bool,                              // 0x28
    packageDataList: *mut BGSPackageDataList, // 0x30
}
const _: () = assert!(core::mem::size_of::<ConditionCheckParams>() == 0x38);

impl ConditionCheckParams {
    #[inline]
    pub const fn new(action_ref: *mut TESObjectREFR, target_ref: *mut TESObjectREFR) -> Self {
        Self {
            actionRef: action_ref,
            targetRef: target_ref,
            quest: ptr::null_mut(),
            questStartEvent: ptr::null_mut(),
            unk20: ptr::null_mut(),
            unk28: false,
            packageDataList: ptr::null_mut(),
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct TESConditionItem {
    next: Option<NonNull<TESConditionItem>>, // 0x0
    data: CONDITION_ITEM_DATA,               // 0x8
}
const _: () = assert!(core::mem::size_of::<TESConditionItem>() == 0x38);

impl TESConditionItem {
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 29090, ae_id = 29924)]
    pub fn is_true(&self, solution: &mut ConditionCheckParams) -> bool {}
}

pub struct BGSPackageDataList;
pub struct BGSStoryEvent;

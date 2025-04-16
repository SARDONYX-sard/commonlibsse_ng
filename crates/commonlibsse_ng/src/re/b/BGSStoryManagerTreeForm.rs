use crate::re::BSTArray::BSTArray;
use crate::re::TESForm::{TESForm, TESFormVtbl};
use crate::re::offsets_rtti::{RTTI_BGSStoryManagerTreeForm, RTTI_BGSStoryManagerTreeVisitor};
use crate::re::offsets_vtable::{
    VTABLE_BGSStoryManagerTreeForm, VTABLE_BGSStoryManagerTreeVisitor,
};
use crate::re::{BGSStoryManagerQuestNode, PeriodicUpdateTimer, VisitControl_CEnum};
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct BGSStoryManagerTreeVisitor {
    pub vtable: *const BGSStoryManagerTreeVisitorVtbl, // 0x00
    pub timer: *mut PeriodicUpdateTimer,               // 0x08
    pub currentCursorDepth: i32,                       // 0x10
    pub pad14: u32,                                    // 0x14
    pub lastQuestParent: *mut BGSStoryManagerQuestNode, // 0x18
    pub cursorAncestry: BSTArray<*mut BGSStoryManagerTreeForm>, // 0x20
    pub queryID: u32,                                  // 0x38
    pub pad3C: u32,                                    // 0x3C
}
const _: () = assert!(core::mem::size_of::<BGSStoryManagerTreeVisitor>() == 0x40);

impl BGSStoryManagerTreeVisitor {
    pub const RTTI: VariantID = RTTI_BGSStoryManagerTreeVisitor;
    pub const VTABLE: [VariantID; 1] = VTABLE_BGSStoryManagerTreeVisitor;
}

#[repr(C)]
#[derive(Debug)]
pub struct BGSStoryManagerTreeVisitorVtbl {
    pub __base: TESFormVtbl, // 0x00
}

#[repr(C)]
#[derive(Debug)]
pub struct BGSStoryManagerTreeForm {
    pub __base: TESForm,    // 0x00
    pub lastVisitorID: u32, // 0x20
    pub pad14: u32,         // 0x24
}
const _: () = assert!(core::mem::size_of::<BGSStoryManagerTreeForm>() == 0x28);

impl BGSStoryManagerTreeForm {
    pub const RTTI: VariantID = RTTI_BGSStoryManagerTreeForm;
    pub const VTABLE: [VariantID; 1] = VTABLE_BGSStoryManagerTreeForm;
}

#[repr(C)]
#[derive(Debug)]
pub struct BGSStoryManagerTreeFormVtbl {
    pub __base: TESFormVtbl,                                    // 0x00
    pub QChildCount: fn(this: &BGSStoryManagerTreeForm) -> u32, // 0x38
    pub GetChild: fn(this: &BGSStoryManagerTreeForm, index: u32) -> *mut BGSStoryManagerTreeForm, // 0x3C
    pub QConditions: fn(this: &mut BGSStoryManagerTreeForm) -> u32,
    pub AcceptVisitor: fn(
        this: &mut BGSStoryManagerTreeForm,
        visitor: &BGSStoryManagerTreeVisitor,
    ) -> VisitControl_CEnum,
}
const _: () = {
    const VFUNC_COUNT: usize = 0x3E + 1;

    const EXPECTED_SIZE: usize = VFUNC_COUNT * core::mem::size_of::<usize>();
    assert!(core::mem::size_of::<BGSStoryManagerTreeFormVtbl>() == EXPECTED_SIZE);
};

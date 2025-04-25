use crate::re::BSTArray::BSTArray;
use crate::re::BSTEvent::BSTEventSink;
use crate::re::BSTEvent::BSTEventSinkVtbl;
use crate::re::BSTHashMap::BSTHashMap;
use crate::re::TESQuest::TESQuest;
use crate::re::TESQuestStageItemDoneEvent;
use crate::re::offsets_rtti::RTTI_BGSStoryTeller;
use crate::re::offsets_vtable::VTABLE_BGSStoryTeller;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct BGSStoryTeller {
    // __base: BSTSingletonSDM<BGSStoryTeller>, // Empty base optimization -> 0 size,
    pub __base: BSTEventSink<TESQuestStageItemDoneEvent>, // 0x00: vtable size
    pad09: u8,                                            // 0x09
    pad0A: u16,                                           // 0x0A
    pad0C: u32,                                           // 0x0C
    pub queuedStartQuests: BSTArray<*mut TESQuest>,       // 0x10
    pub runningQuests: BSTArray<*mut TESQuest>,           // 0x28
    pub queuedStopQuests: BSTArray<*mut TESQuest>,        // 0x40
    pub infoClearQuests: BSTArray<*mut TESQuest>,         // 0x58
    pub helloTopicQuests: BSTArray<*mut TESQuest>,        // 0x70
    pub greetingTopicQuests: BSTArray<*mut TESQuest>,     // 0x88
    pub startUpQuestsInitialized: bool,                   // 0xA0
    padA1: u8,                                            // 0xA1
    padA2: u16,                                           // 0xA2
    padA4: u32,                                           // 0xA4
    pub questStageWaitMap: BSTHashMap<u32, *mut BSTArray<(u32, u32)>>, // 0xA8
}
const _: () = assert!(core::mem::size_of::<BGSStoryTeller>() == 0xD8);

impl BGSStoryTeller {
    pub const RTTI: VariantID = RTTI_BGSStoryTeller;
    pub const VTABLE: [VariantID; 1] = VTABLE_BGSStoryTeller;

    /// Get vtable this class.
    ///
    /// # Panics
    /// If vtable is null.
    #[inline]
    pub const fn vtable(&self) -> &BGSStoryTellerVtbl {
        debug_assert!(self.__base.vtable.is_some(), "BGSStoryTellerVtbl ptr must not be null ptr");
        unsafe { self.__base.vtable.unwrap().cast().as_ref() }
    }

    /// Gets the singleton instance of `BGSStoryTeller`.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut *mut BGSStoryTeller",
        default = "None",
        deref_once,
        id(se = 514316, ae = 400476)
    )]
    pub fn get_singleton() -> Option<&'static BGSStoryTeller> {
        |deref_type: DerefType| unsafe { deref_type.as_ref() }
    }

    /// Gets the mutable singleton instance of `BGSStoryTeller`.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut *mut BGSStoryTeller",
        default = "None",
        deref_once,
        id(se = 514316, ae = 400476)
    )]
    pub fn get_singleton_mut() -> Option<&'static mut BGSStoryTeller> {
        |deref_type: DerefType| unsafe { deref_type.as_mut() }
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 31718, ae_id = 32486)]
    pub fn begin_shut_down_quest(&mut self, quest: *mut TESQuest) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 31717, ae_id = 32485)]
    pub fn begin_start_up_quest(&mut self, quest: *mut TESQuest) {}
}

#[repr(C)]
pub struct BGSStoryTellerVtbl {
    pub __base: BSTEventSinkVtbl<TESQuestStageItemDoneEvent>, // 0x00
}
const _: () = {
    const VFUNC_COUNT: usize = 0x2;

    const EXPECTED_SIZE: usize = VFUNC_COUNT * core::mem::size_of::<usize>();
    assert!(core::mem::size_of::<BGSStoryTellerVtbl>() == EXPECTED_SIZE);
};

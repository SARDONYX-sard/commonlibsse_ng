use crate::re::BGSScene;
use crate::re::BGSStoryManagerTreeForm::{BGSStoryManagerTreeForm, BGSStoryManagerTreeFormVtbl};
use crate::re::BGSStoryTeller::BGSStoryTeller;
use crate::re::BSAtomic::BSReadWriteLock;
use crate::re::BSFixedString::BSFixedString;
use crate::re::BSPointerHandle::ObjectRefHandle;
use crate::re::BSString::BSString;
use crate::re::BSTArray::BSTArray;
use crate::re::BSTHashMap::{BSTHashMap, UnkKey, UnkValue};
use crate::re::BSTList::BSSimpleList;
use crate::re::DialogueTypes::{DIALOGUE_TYPE, DIALOGUE_TYPE_CEnum};
use crate::re::FormTypes::FormType;
use crate::re::NiSmartPointer::NiPointer;
use crate::re::QuestEvent::QuestEvent;
use crate::re::QuestObjectiveStates::QUEST_OBJECTIVE_STATE;
use crate::re::TESCondition::TESCondition;
use crate::re::TESForm::{DerivedTESForm, TESForm};
use crate::re::TESFullName::{TESFullName, TESFullNameVtbl};
use crate::re::TESGlobal::TESGlobal;
use crate::re::offsets_rtti::RTTI_TESQuest;
use crate::re::offsets_vtable::VTABLE_TESQuest;
use crate::re::{BGSBaseAlias, BGSDialogueBranch, QueuedPromoteQuestTask, TESTopic};
use crate::rel::id::VariantID;

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum QuestFlag {
    StopStart = 65535, // cast -1
    None = 0,
    Enabled = 1 << 0,
    Completed = 1 << 1,
    AddIdleToHello = 1 << 2,
    AllowRepeatStages = 1 << 3,
    StartsEnabled = 1 << 4,
    DisplayedInHUD = 1 << 5,
    Failed = 1 << 6,
    StageWait = 1 << 7,
    RunOnce = 1 << 8,
    ExcludeFromExport = 1 << 9,
    WarnOnAliasFillFailure = 1 << 10,
    Active = 1 << 11,
    RepeatsConditions = 1 << 12,
    KeepInstance = 1 << 13,
    WantDormant = 1 << 14,
    HasDialogueData = 1 << 15,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum QUEST_OBJECTIVE_FLAGS {
    None = 0,
    ORWithPrevious = 1 << 0,
    NoStatsTracking = 1 << 1,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringData {
    pub aliasId: u32,
    pub fullNameFormID: u32,
}
const _: () = assert!(core::mem::size_of::<StringData>() == 0x8);

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalValueData {
    pub global: *const TESGlobal, // 0x000
    pub value: f32,               // 0x008
    pub pad0C: u32,               // 0x00C
}
const _: () = assert!(core::mem::size_of::<GlobalValueData>() == 0x10);

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BGSQuestInstanceText {
    id: u32,                              // 0x000
    pad04: u32,                           // 0x004
    stringData: BSTArray<StringData>,     // 0x008
    valueData: BSTArray<GlobalValueData>, // 0x020
    journalStage: u16,                    // 0x038
    journalStageItem: u8,                 // 0x03A
    pad3B: u8,                            // 0x03B
    pad3C: u32,                           // 0x03C
}
const _: () = assert!(core::mem::size_of::<BGSQuestInstanceText>() == 0x40);

/// - C++ `Type`
#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum QuestType {
    None = 0,
    MainQuest = 1,
    MagesGuild = 2,
    ThievesGuild = 3,
    DarkBrotherhood = 4,
    CompanionsQuest = 5,
    Miscellaneous = 6,
    Daedric = 7,
    SideQuest = 8,
    CivilWar = 9,
    DLC01Vampire = 10,
    DLC02Dragonborn = 11,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct QUEST_DATA {
    pub questDelayTime: f32,  // 0x000
    pub flags: QuestFlag,     // 0x004
    pub priority: i8,         // 0x006
    pub questType: QuestType, // 0x007
}
const _: () = assert!(core::mem::size_of::<QUEST_DATA>() == 0x8);

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum QuestStageFlag {
    None = 0,
    StartUpStage = 1 << 1,
    ShutDownStage = 1 << 2,
    KeepInstanceDataFromHereOn = 1 << 3,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QUEST_STAGE_DATA {
    pub index: u16,            // 0x000
    pub flags: QuestStageFlag, // 0x002
    pad3: u8,                  // 0x003
    pad4: u32,                 // 0x004
}
const _: () = assert!(core::mem::size_of::<QUEST_STAGE_DATA>() == 0x8);

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TESQuestStage {
    pub data: QUEST_STAGE_DATA,
}
const _: () = assert!(core::mem::size_of::<TESQuestStage>() == 0x8);

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TESQuestTargetFlag {
    None = 0,
    CompassMarkerIgnoresLocks = 1 << 0,
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct TESQuestTarget {
    pub unk00: u64,               // 0x00
    pub conditions: TESCondition, // 0x08
    pub alias: u8,                // 0x10
    pub unk11: u8,                // 0x11
    pub unk12: u16,               // 0x12
    pub unk14: u32,               // 0x14
}
const _: () = assert!(core::mem::size_of::<TESQuestTarget>() == 0x18);

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BGSQuestObjective {
    displayText: BSFixedString,        // 0x00 - NNAM
    ownerQuest: *mut TESQuest,         // 0x08
    targets: *mut *mut TESQuestTarget, // 0x10 - QSTA
    numTargets: u32,                   // 0x18
    index: u16,                        // 0x1C - QOBJ
    initialized: bool,                 // 0x1E
    state: QUEST_OBJECTIVE_STATE,      // 0x1F
    flags: QUEST_OBJECTIVE_FLAGS,      // 0x20 - FNAM
    pad24: u32,                        // 0x24
}
const _: () = assert!(core::mem::size_of::<BGSQuestObjective>() == 0x28);

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BGSStoryEvent {
    pub id: u32,
    pub index: u32,
    pub members: [u64; 6],
}
const _: () = assert!(core::mem::size_of::<BGSStoryEvent>() == 0x38);

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum ChangeFlag {
    QuestFlags = 1 << 1,
    QuestScriptDelay = 1 << 2,
    QuestAlreadyRun = 1 << 26,
    QuestInstanceData = 1 << 27,
    QuestRuntimeData = 1 << 28,
    QuestObjectives = 1 << 29,
    QuestScript = 1 << 30,
    QuestStages = 1 << 31,
}

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum RecordFlag {
    Deleted = 1 << 5,
    Ignored = 1 << 12,
}

const BRANCHED_TOTAL: usize = DIALOGUE_TYPE::BRANCHED_TOTAL;
const TOPICS_SIZE: usize = DIALOGUE_TYPE_CEnum::count() - BRANCHED_TOTAL;

#[repr(C)]
#[derive(Debug)]
pub struct TESQuest {
    pub __base: BGSStoryManagerTreeForm,                      // 0x000
    pub __base1: TESFullName,                                 // 0x028
    pub instanceData: BSTArray<*mut BGSQuestInstanceText>,    // 0x038
    pub currentInstanceID: u32,                               // 0x050
    pub pad054: u32,                                          // 0x054
    pub aliases: BSTArray<*mut BGSBaseAlias>,                 // 0x058
    pub refAliasMap: BSTHashMap<u32, ObjectRefHandle>,        // 0x070
    pub unk0A0: BSTHashMap<UnkKey, UnkValue>,                 // 0x0A0 - alias related
    pub aliasAccessLock: BSReadWriteLock,                     // 0x0D0
    pub data: QUEST_DATA,                                     // 0x0D8 - DNAM
    pub eventID: QuestEvent,                                  // 0x0E0 - ENAM
    pub pad0E4: u32,                                          // 0x0E4
    pub executedStages: *mut BSSimpleList<TESQuestStage>,     // 0x0E8
    pub waitingStages: *mut BSSimpleList<*mut TESQuestStage>, // 0x0F0
    pub objectives: BSSimpleList<*mut BGSQuestObjective>,     // 0x0F8
    pub objConditions: TESCondition,                          // 0x108
    pub storyManagerConditions: TESCondition,                 // 0x110
    pub branchedDialogue:
        [BSTHashMap<*mut BGSDialogueBranch, *mut BSTArray<*mut TESTopic>>; BRANCHED_TOTAL], // 0x118
    pub topics: [BSTArray<*mut TESTopic>; TOPICS_SIZE],       // 0x178
    pub scenes: BSTArray<*mut BGSScene>,                      // 0x208
    pub textGlobals: *mut BSTArray<*mut TESGlobal>,           // 0x220 - QTGL
    pub currentStage: u16,                                    // 0x228
    pub alreadyRun: bool,                                     // 0x22A
    pub pad22B: u8,                                           // 0x22B
    pub pad22C: u32,                                          // 0x22C
    pub formEditorID: BSString,                               // 0x230
    pub startEventData: *const BGSStoryEvent,                 // 0x240
    pub promoteTask: NiPointer<QueuedPromoteQuestTask>,       // 0x248
    pub promotedRefs: BSTArray<ObjectRefHandle>,              // 0x250
}
const _: () = assert!(core::mem::size_of::<TESQuest>() == 0x268);

impl TESQuest {
    pub const RTTI: VariantID = RTTI_TESQuest;
    pub const VTABLE: [VariantID; 2] = VTABLE_TESQuest;

    /// Get vtable this class.
    ///
    /// # Panics
    /// If vtable is null.
    #[inline]
    pub const fn vtable(&self) -> &TESQuestVtbl {
        let v_ptr = self.__base.__base.__base.vtable;
        debug_assert!(!v_ptr.is_null(), "BGSStoryTellerVtbl ptr must not be null ptr");
        unsafe { v_ptr.cast::<TESQuestVtbl>().as_ref().unwrap() }
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 24537, ae_id = 25066)]
    pub fn create_ref_handle_by_alias_id(
        handle: &ObjectRefHandle,
        alias_id: u32,
    ) -> *mut ObjectRefHandle {
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 24481, ae_id = 25003)]
    pub fn ensure_quest_started(result: &mut bool, start_now: bool) -> bool {}

    #[inline]
    pub const fn get_current_stage_id(&self) -> u16 {
        self.currentStage
    }

    #[inline]
    pub const fn is_active(&self) -> bool {
        self.data.flags.contains(QuestFlag::Active)
    }

    #[inline]
    pub const fn is_completed(&self) -> bool {
        self.data.flags.contains(QuestFlag::Completed)
    }

    #[inline]
    pub const fn is_enabled(&self) -> bool {
        self.data.flags.contains(QuestFlag::Enabled)
    }

    #[inline]
    pub fn is_running(&self) -> bool {
        !self.is_stopping() && self.promoteTask.is_null()
    }

    #[inline]
    pub fn is_starting(&self) -> bool {
        self.is_enabled()
            && (self.data.flags == QuestFlag::StopStart || !self.promoteTask.is_null())
    }

    #[inline]
    pub const fn is_stopped(&self) -> bool {
        !(self.data.flags.contains(QuestFlag::Enabled)
            || self.data.flags.contains(QuestFlag::StageWait))
    }

    #[inline]
    pub fn is_stopping(&self) -> bool {
        !self.is_enabled() && self.data.flags == QuestFlag::StopStart
    }

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 24486, ae_id = 25014)]
    pub fn reset(&mut self) {}

    pub fn reset_and_update(&mut self) {
        self.reset();

        let enabled = self.is_enabled();
        if enabled != self.starts_enabled() {
            if let Some(story_teller) = BGSStoryTeller::get_singleton_mut() {
                if enabled {
                    story_teller.begin_start_up_quest(self);
                } else {
                    story_teller.begin_shut_down_quest(self);
                }
            }
        }
    }

    pub fn set_enabled(&mut self, value: bool) {
        if value {
            self.data.flags.insert(QuestFlag::Enabled);
        } else {
            self.data.flags.remove(QuestFlag::Enabled);
        }

        let add_change_fn = self.__base.__base.vtable().AddChange;
        add_change_fn(&mut self.__base.__base, ChangeFlag::QuestFlags.bits());
    }

    pub fn start(&mut self) -> bool {
        if self.eventID != QuestEvent::None {
            #[cfg(feature = "tracing")]
            tracing::warn!("Attempting to start event scoped quest outside of story manager");
            return false;
        }

        let mut result = false;
        Self::ensure_quest_started(&mut result, true)
    }

    #[inline]
    pub const fn starts_enabled(&self) -> bool {
        self.data.flags.contains(QuestFlag::StartsEnabled)
    }

    #[inline]
    pub fn stop(&mut self) {
        if self.is_enabled() {
            self.set_enabled(false);
        }
    }
}

impl DerivedTESForm for TESQuest {
    const FORM_TYPE: FormType = FormType::Quest;

    #[inline]
    fn get_form(&self) -> &TESForm {
        &self.__base.__base
    }
}

#[repr(C)]
pub struct TESQuestVtbl {
    pub __base: BGSStoryManagerTreeFormVtbl, // 0x000
    pub __base1: TESFullNameVtbl,            //
}

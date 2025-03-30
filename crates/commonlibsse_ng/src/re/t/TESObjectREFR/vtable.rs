use core::ffi::c_void;

use crate::re::BSPointerHandle::ObjectRefHandle;
use crate::re::ExtraDataList::ExtraDataList;
use crate::re::NiPoint3::NiPoint3;
use crate::re::TESBoundObject::TESBoundObject;
use crate::re::TESForm::TESForm;
use crate::re::TESObjectCELL::TESObjectCELL;
use crate::re::{
    ActorCause, BGSAnimationSequencer, BGSDialogueBranch, BGSKeyword, BGSLocation, BGSScene,
    DialogueResponse, ITEM_REMOVE_REASON, TESPackage, TESTopicInfo, TargetEntry, TrapData,
    TrapEntry,
};

use super::TESObjectREFR;

#[repr(C)]
pub struct TESObjectREFRVtbl {
    pub Predestroy: unsafe extern "C" fn(this: *mut c_void),
    pub GetEditorLocation1: unsafe extern "C" fn(this: *const c_void) -> *const BGSLocation,
    pub GetEditorLocation2: unsafe extern "C" fn(
        this: *mut c_void,
        outPos: *mut NiPoint3,
        outRot: *mut NiPoint3,
        outWorldOrCell: *mut *mut TESForm,
        fallback: *mut TESObjectCELL,
    ) -> bool,
    pub ForceEditorLocation: unsafe extern "C" fn(this: *mut c_void, location: *mut BGSLocation),
    pub Update3DPosition: unsafe extern "C" fn(this: *mut c_void, warp: bool),
    pub UpdateSoundCallBack: unsafe extern "C" fn(this: *mut c_void, endSceneAction: bool),
    pub SetDialogueWithPlayer: unsafe extern "C" fn(
        this: *mut c_void,
        flag: bool,
        forceGreet: bool,
        topic: *mut TESTopicInfo,
    ) -> bool,
    pub DamageObject: unsafe extern "C" fn(this: *mut c_void, objectHealth: f32, arg3: bool),
    pub GetFullLODRef: unsafe extern "C" fn(this: *const c_void) -> bool,
    pub SetFullLODRef: unsafe extern "C" fn(this: *mut c_void, set: bool),
    pub GetSequencer: unsafe extern "C" fn(this: *const c_void) -> *const BGSAnimationSequencer,
    pub QCanUpdateSync: unsafe extern "C" fn(this: *const c_void) -> bool,
    pub GetAllowPromoteToPersistent: unsafe extern "C" fn(this: *const c_void) -> bool,
    pub HasKeywordHelper:
        unsafe extern "C" fn(this: *const c_void, keyword: *const BGSKeyword) -> bool,
    pub CheckForCurrentAliasPackage: unsafe extern "C" fn(this: *mut c_void) -> *const TESPackage,
    pub GetCurrentScene: unsafe extern "C" fn(this: *const c_void) -> *const BGSScene,
    pub SetCurrentScene: unsafe extern "C" fn(this: *mut c_void, scene: *mut BGSScene),
    pub UpdateInDialogue: unsafe extern "C" fn(
        this: *mut c_void,
        response: *mut DialogueResponse,
        unused: bool,
    ) -> bool,
    pub GetExclusiveBranch: unsafe extern "C" fn(this: *const c_void) -> *const BGSDialogueBranch,
    pub SetExclusiveBranch: unsafe extern "C" fn(this: *mut c_void, branch: *mut BGSDialogueBranch),
    pub PauseCurrentDialogue: unsafe extern "C" fn(this: *mut c_void),
    pub SetActorCause: unsafe extern "C" fn(this: *mut c_void, cause: *mut ActorCause),
    pub GetActorCause: unsafe extern "C" fn(this: *const c_void) -> *const ActorCause,
    pub GetStartingAngle: unsafe extern "C" fn(this: *const c_void) -> NiPoint3,
    pub GetStartingLocation: unsafe extern "C" fn(this: *const c_void) -> NiPoint3,
    pub SetStartingPosition: unsafe extern "C" fn(this: *mut c_void, pos: *const NiPoint3),
    pub UpdateRefLight: unsafe extern "C" fn(this: *mut c_void),
    pub RemoveItem: unsafe extern "C" fn(
        this: *mut c_void,
        item: *mut TESBoundObject,
        count: i32,
        reason: ITEM_REMOVE_REASON,
        extraList: *mut ExtraDataList,
        moveToRef: *mut TESObjectREFR,
        dropLoc: *const NiPoint3,
        rotate: *const NiPoint3,
    ) -> ObjectRefHandle,
    pub AddWornItem: unsafe extern "C" fn(
        this: *mut c_void,
        item: *mut TESBoundObject,
        count: i32,
        forceEquip: bool,
        arg4: u32,
        arg5: u32,
    ) -> bool,
    pub DoTrap1: unsafe extern "C" fn(this: *mut c_void, data: *mut TrapData),
    pub DoTrap2:
        unsafe extern "C" fn(this: *mut c_void, trap: *mut TrapEntry, target: *mut TargetEntry),
}

use core::ffi::c_void;

use crate::re::BSAnimationUpdateData::BSAnimationUpdateData;
use crate::re::BSPointerHandle::ObjectRefHandle;
use crate::re::BSTEvent::BSTEventSinkVtbl;
use crate::re::BSTSmartPointer::BSTSmartPointer;
use crate::re::ExtraDataList::ExtraDataList;
use crate::re::NiAVObject::NiAVObject;
use crate::re::NiPoint3::NiPoint3;
use crate::re::TESBoundObject::TESBoundObject;
use crate::re::TESForm::{TESForm, TESFormVtbl};
use crate::re::TESObjectCELL::TESObjectCELL;
use crate::re::i::IAnimationGraphManagerHolder::IAnimationGraphManagerHolderVtbl;
use crate::re::{
    ActorCause, BGSAnimationSequencer, BGSDialogueBranch, BGSKeyword, BGSLocation, BGSScene,
    BSAnimationGraphEvent, BSFaceGenAnimationData, BSFaceGenNiNode, BipedAnim, DialogueResponse,
    ITEM_REMOVE_REASON, MagicCaster, MagicTarget, NiNode, TESActorBase, TESPackage, TESTopicInfo,
    TargetEntry, TrapData, TrapEntry,
};

use super::TESObjectREFR;

#[repr(C)]
pub struct TESObjectREFRVtbl {
    pub __base: TESFormVtbl,
    pub __base1: BSTEventSinkVtbl<BSAnimationGraphEvent>,
    pub __base2: BSTEventSinkVtbl<BSAnimationGraphEvent>,
    pub __base3: IAnimationGraphManagerHolderVtbl,

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
    pub AddObjectToContainer: unsafe extern "C" fn(
        this: *mut c_void,
        object: *mut TESBoundObject,
        extrlist: *mut ExtraDataList,
        count: i32,
        from_refr: *mut TESObjectREFR,
    ),
    pub GetLookingAtLocation: unsafe extern "C" fn(this: *const c_void) -> NiPoint3,
    pub GetMagicCaster: unsafe extern "C" fn(this: *mut c_void, source: i32) -> *mut MagicCaster,
    pub GetMagicTarget: unsafe extern "C" fn(this: *mut c_void) -> *mut MagicTarget,
    pub IsChild: unsafe extern "C" fn(this: *const c_void) -> bool,
    pub GetTemplateActorBase: unsafe extern "C" fn(this: *mut c_void) -> *mut TESActorBase,
    pub SetTemplateActorBase: unsafe extern "C" fn(this: *mut c_void, template: *mut TESActorBase),
    pub GetFaceNodeSkinned: unsafe extern "C" fn(this: *mut c_void) -> *mut BSFaceGenNiNode,
    pub GetFaceNode: unsafe extern "C" fn(this: *mut c_void) -> *mut BSFaceGenNiNode,
    pub GetFaceGenAnimationData:
        unsafe extern "C" fn(this: *mut c_void) -> *mut BSFaceGenAnimationData,
    pub ClampToGround: unsafe extern "C" fn(this: *mut c_void) -> bool,
    pub DetachHavok: unsafe extern "C" fn(this: *mut c_void, obj3D: *mut NiAVObject) -> bool,
    pub InitHavok: unsafe extern "C" fn(this: *mut c_void),
    pub Unk_67: unsafe extern "C" fn(this: *mut c_void),
    pub Unk_68: unsafe extern "C" fn(this: *mut c_void),
    pub Unk_69: unsafe extern "C" fn(this: *mut c_void),
    pub Load3D:
        unsafe extern "C" fn(this: *mut c_void, background_loading: bool) -> *mut NiAVObject,
    pub Release3DRelatedData: unsafe extern "C" fn(this: *mut c_void),
    pub Set3D:
        unsafe extern "C" fn(this: *mut c_void, object: *mut NiAVObject, queue3D_tasks: bool),
    pub ShouldBackgroundClone: unsafe extern "C" fn(this: *const c_void) -> bool,
    pub Unk_6E: unsafe extern "C" fn(this: *mut c_void),
    pub Get3D1: unsafe extern "C" fn(this: *const c_void, first_person: bool) -> *mut NiAVObject,
    pub Get3D2: unsafe extern "C" fn(this: *const c_void) -> *mut NiAVObject,
    pub Is3rdPersonVisible: unsafe extern "C" fn(this: *const c_void) -> bool,
    pub PopulateGraphProjectsToLoad: unsafe extern "C" fn(this: *const c_void) -> bool,
    pub GetBoundMin: unsafe extern "C" fn(this: *const c_void) -> NiPoint3,
    pub GetBoundMax: unsafe extern "C" fn(this: *const c_void) -> NiPoint3,
    pub Unk_75: unsafe extern "C" fn(this: *mut c_void),
    pub InitNonNPCAnimation:
        unsafe extern "C" fn(this: *mut c_void, node_for_anim: *mut NiNode) -> bool,
    pub CheckAndFixSkinAndBoneOrder:
        unsafe extern "C" fn(this: *mut c_void, node_to_test: *mut NiNode) -> bool,
    pub Unk_78: unsafe extern "C" fn(this: *mut c_void),
    pub ModifyAnimationUpdateData:
        unsafe extern "C" fn(this: *mut c_void, data: *mut BSAnimationUpdateData),
    pub ShouldSaveAnimationOnUnloading: unsafe extern "C" fn(this: *const c_void) -> bool,
    pub ShouldSaveAnimationOnSaving: unsafe extern "C" fn(this: *const c_void) -> bool,
    pub ShouldPerformRevert: unsafe extern "C" fn(this: *const c_void) -> bool,
    pub UpdateAnimation: unsafe extern "C" fn(this: *mut c_void, delta: f32),
    pub GetBiped1:
        unsafe extern "C" fn(this: *const c_void, first_person: bool) -> BSTSmartPointer<BipedAnim>,
    pub GetBiped2: unsafe extern "C" fn(this: *const c_void) -> BSTSmartPointer<BipedAnim>,
    pub GetCurrentBiped: unsafe extern "C" fn(this: *const c_void) -> BSTSmartPointer<BipedAnim>,
    pub SetBiped: unsafe extern "C" fn(this: *mut c_void, biped: BSTSmartPointer<BipedAnim>),
}

const _: () = {
    use core::mem::size_of;
    const SIZE: usize = size_of::<TESObjectREFRVtbl>();
    assert!(SIZE == 0x82 * size_of::<usize>());
};

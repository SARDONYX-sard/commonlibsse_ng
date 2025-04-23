//! === Extern C++ ABI Dummy Types ===

#[derive(Debug)]
pub struct GFxMovieView;
impl crate::re::GPtr::RefCounted for GFxMovieView {
    fn add_ref(&mut self) {
        todo!()
    }
    fn release(&mut self) {
        todo!()
    }
}
#[derive(Debug)]
pub struct FxDelegate;
impl crate::re::GPtr::RefCounted for FxDelegate {
    fn add_ref(&mut self) {
        todo!()
    }
    fn release(&mut self) {
        todo!()
    }
}

pub struct TesWaterForm;

#[derive(Debug)]
pub struct BSTransformDeltaEvent;

#[repr(C)]
pub enum ITEM_REMOVE_REASON {
    Remove,
    Steal,
    Selling,
    Dropping,
    StoreInContainer,
    StoreInTeammate,
}
pub struct ObjectHandle;
pub struct NiExtraData;
#[derive(Debug)]
pub struct NiTimeController;

impl crate::re::NiSmartPointer::RefCountable for NiTimeController {
    fn inc_ref_count(&self) {
        todo!()
    }
    fn dec_ref_count(&mut self) {
        todo!()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct NiNode;
impl crate::re::NiSmartPointer::RefCountable for NiNode {
    fn inc_ref_count(&self) {
        todo!()
    }
    fn dec_ref_count(&mut self) {
        todo!()
    }
}

#[repr(C)]
pub struct NiSwitchNode;

#[repr(C)]
#[derive(Debug)]
pub struct BSFadeNode;
impl crate::re::NiSmartPointer::RefCountable for BSFadeNode {
    fn inc_ref_count(&self) {
        todo!()
    }
    fn dec_ref_count(&mut self) {
        todo!()
    }
}

#[repr(C)]
pub struct BSMultiBoundNode;
impl crate::re::NiSmartPointer::RefCountable for BSMultiBoundNode {
    fn inc_ref_count(&self) {
        todo!()
    }
    fn dec_ref_count(&mut self) {
        todo!()
    }
}

#[repr(C)]
pub struct BSGeometry;

#[repr(C)]
pub struct NiTriStrips;

#[repr(C)]
#[derive(Debug)]
pub struct BSTriShape;
impl crate::re::NiSmartPointer::RefCountable for BSTriShape {
    fn inc_ref_count(&self) {
        todo!()
    }
    fn dec_ref_count(&mut self) {
        todo!()
    }
}

#[repr(C)]
pub struct BSSegmentedTriShape;

#[repr(C)]
pub struct BSSubIndexTriShape;

#[repr(C)]
pub struct BSDynamicTriShape;

#[repr(C)]
pub struct NiGeometry;

#[repr(C)]
pub struct NiTriBasedGeom;

#[repr(C)]
pub struct NiTriShape;

#[repr(C)]
pub struct NiParticles;

#[repr(C)]
pub struct BSLines;

#[repr(C)]
pub struct bhkNiCollisionObject;

#[repr(C)]
pub struct bhkBlendCollisionObject;

#[repr(C)]
pub struct bhkAttachmentCollisionObject;

#[repr(C)]
#[derive(Debug)]
pub struct bhkRigidBody;
impl crate::re::NiSmartPointer::RefCountable for bhkRigidBody {
    fn inc_ref_count(&self) {
        todo!()
    }

    fn dec_ref_count(&mut self) {
        todo!()
    }
}

#[repr(C)]
pub struct bhkLimitedHingeConstraint;

#[repr(C)]
pub struct NiCloningProcess;
pub struct NiStream;
pub struct NiObjectGroup;
pub struct NiControllerManager;
pub struct bhkCollisionObject;

pub struct BGSLocation;
pub struct TESFaction;
#[derive(Debug)]
pub struct NiBillboardNode;
impl crate::re::NiSmartPointer::RefCountable for NiBillboardNode {
    fn inc_ref_count(&self) {
        todo!()
    }
    fn dec_ref_count(&mut self) {
        todo!()
    }
}

#[repr(C)]
pub struct TESTopicInfo;
#[repr(C)]
pub struct BGSAnimationSequencer;
#[repr(C)]
pub struct TESPackage;
#[repr(C)]
pub struct BGSScene;
#[repr(C)]
pub struct DialogueResponse;
#[repr(C)]
pub struct BGSDialogueBranch;
#[repr(C)]
pub struct ActorCause;
#[repr(C)]
pub struct MagicCaster;
#[repr(C)]
pub struct MagicTarget;
#[repr(C)]
pub struct TESActorBase;
#[repr(C)]
pub struct BSFaceGenNiNode;
#[repr(C)]
pub struct BSFaceGenAnimationData;
#[repr(C)]
pub struct TrapData;
#[repr(C)]
pub struct TrapEntry;
#[repr(C)]
pub struct TargetEntry;
#[repr(C)]
#[derive(Debug)]
pub struct BipedAnim;
impl crate::re::BSIntrusiveRefCounted::BSIntrusiveRefCountedTrait for BipedAnim {
    fn inc_ref(&self) -> u32 {
        0
    }

    fn dec_ref(&self) -> u32 {
        0
    }
}

#[derive(Debug)]
pub struct BSAnimationGraphManagerPtr;
#[derive(Debug)]
pub struct BSAnimationCache;

#[repr(C)]
#[derive(Debug)]
pub struct hkStatisticsCollector;

#[repr(C)]
#[derive(Debug)]
pub struct hkClass;
pub struct ahkpWorld;
pub struct hkpWorld;
pub struct hkbRagdollDriver;
impl crate::re::hkRefPtr::hkRefPtrCounted for hkbRagdollDriver {}

#[derive(Debug, Default)]
pub struct hkCriticalSection;
impl crate::re::hkRefPtr::hkRefPtrCounted for hkCriticalSection {}
#[derive(Debug)]
pub struct hkaMirroredSkeleton;
impl crate::re::hkRefPtr::hkRefPtrCounted for hkaMirroredSkeleton {}
#[derive(Debug)]
pub struct hkaSkeleton;
impl crate::re::hkRefPtr::hkRefPtrCounted for hkaSkeleton {}
#[derive(Debug)]
pub struct hkaSkeletonMapper;
impl crate::re::hkRefPtr::hkRefPtrCounted for hkaSkeletonMapper {}

#[derive(Debug)]
pub struct hkbAnimationBinding;
#[derive(Debug)]
pub struct hkbCharacterData;
impl crate::re::hkRefPtr::hkRefPtrCounted for hkbCharacterData {}
#[derive(Debug)]
pub struct hkbSymbolIdMap;
impl crate::re::hkRefPtr::hkRefPtrCounted for hkbSymbolIdMap {}
#[derive(Debug)]
pub struct hkbAnimationBindingSet;
impl crate::re::hkRefPtr::hkRefPtrCounted for hkbAnimationBindingSet {}
#[derive(Debug)]
pub struct BGSWaterUpdateI;
#[derive(Debug)]
pub struct TESObjectLAND;
#[derive(Debug)]
pub struct BGSLightingTemplate;
#[derive(Debug)]
pub struct BSPortalGraph;
impl crate::re::NiSmartPointer::RefCountable for BSPortalGraph {
    fn inc_ref_count(&self) {
        todo!()
    }

    fn dec_ref_count(&mut self) {
        todo!()
    }
}
pub struct NavMesh;

impl crate::re::BSIntrusiveRefCounted::BSIntrusiveRefCountedTrait for NavMesh {
    fn inc_ref(&self) -> u32 {
        todo!()
    }

    fn dec_ref(&self) -> u32 {
        todo!()
    }
}

#[derive(Debug)]
pub struct BSTMap<T, U> {
    /// dummy
    pub unk_opaque: [u8; 0x20],
    marker: core::marker::PhantomData<(T, U)>,
}
#[derive(Debug)]
pub struct BSTSet<T> {
    /// dummy
    pub unk_opaque: [u8; 0x20],
    marker: core::marker::PhantomData<T>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BGSActorDeathEvent;
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserEventEnabledEvent;

#[repr(C)]
#[derive(Debug)]
pub struct TESRace;
#[repr(C)]
#[derive(Debug)]
pub struct BSLight;
impl crate::re::NiSmartPointer::RefCountable for BSLight {
    fn inc_ref_count(&self) {
        todo!()
    }

    fn dec_ref_count(&mut self) {
        todo!()
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct CombatGroup;
#[repr(C)]
#[derive(Debug)]
pub struct TESClass;

pub struct EffectItem;
pub struct EffectSetting;
pub struct Effect;
#[derive(Debug, Clone)]
pub struct AlchemyItem;
#[derive(Debug, Clone, Copy)]
pub enum SoulLevel {}

#[derive(Debug, Clone, PartialEq)]
pub struct ActorMover;

#[derive(Debug, Clone, PartialEq)]
pub struct CombatController;

#[derive(Debug, Clone, PartialEq)]
pub struct AITimeStamp {
    dummy: [u8; 4],
}

#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum EmotionType {
    Dummy,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MovementControllerNPC;
impl crate::re::BSIntrusiveRefCounted::BSIntrusiveRefCountedTrait for MovementControllerNPC {
    fn inc_ref(&self) -> u32 {
        todo!()
    }

    fn dec_ref(&self) -> u32 {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActorMagicCaster;

#[repr(C)]
#[derive(Debug)]
pub struct Projectile;
impl crate::re::NiSmartPointer::RefCountable for Projectile {
    #[inline]
    fn inc_ref_count(&self) {
        todo!()
    }

    #[inline]
    fn dec_ref_count(&mut self) {
        todo!()
    }
}
#[derive(Debug)]
pub struct QueuedFile;
impl crate::re::NiSmartPointer::RefCountable for QueuedFile {
    fn inc_ref_count(&self) {
        todo!()
    }

    fn dec_ref_count(&mut self) {
        todo!()
    }
}
impl crate::re::BSIntrusiveRefCounted::BSIntrusiveRefCountedTrait for QueuedFile {
    fn inc_ref(&self) -> u32 {
        todo!()
    }

    fn dec_ref(&self) -> u32 {
        todo!()
    }
}

pub struct CallbackProcessor;

/// NOTE: Type unknown even in original implementation.
#[derive(Debug)]
#[repr(C)]
pub struct ActorPackageData;

#[repr(C)]
#[derive(Debug)]
pub struct ActiveEffect;

#[repr(C)]
#[derive(Debug)]
pub struct BSLightingShaderProperty;
impl crate::re::NiSmartPointer::RefCountable for BSLightingShaderProperty {
    fn inc_ref_count(&self) {
        todo!()
    }

    fn dec_ref_count(&mut self) {
        todo!()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct HitData;

#[repr(C)]
#[derive(Debug)]
pub struct TESIdleForm;

#[repr(C)]
#[derive(Debug)]
pub struct bhkCharacterController;
impl crate::re::NiSmartPointer::RefCountable for bhkCharacterController {
    fn inc_ref_count(&self) {
        todo!()
    }

    fn dec_ref_count(&mut self) {
        todo!()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct bhkRagdollPenetrationUtil;
impl crate::re::BSIntrusiveRefCounted::BSIntrusiveRefCountedTrait for bhkRagdollPenetrationUtil {
    fn inc_ref(&self) -> u32 {
        todo!()
    }

    fn dec_ref(&self) -> u32 {
        todo!()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct BSCloneReserver;
impl crate::re::NiSmartPointer::RefCountable for BSCloneReserver {
    fn inc_ref_count(&self) {
        todo!()
    }

    fn dec_ref_count(&mut self) {
        todo!()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct AnimResponse;
impl crate::re::BSIntrusiveRefCounted::BSIntrusiveRefCountedTrait for AnimResponse {
    fn inc_ref(&self) -> u32 {
        todo!()
    }

    fn dec_ref(&self) -> u32 {
        todo!()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct HighProcessData;
pub struct QueuedPromoteQuestTask;
impl crate::re::NiSmartPointer::RefCountable for QueuedPromoteQuestTask {
    fn inc_ref_count(&self) {
        todo!()
    }

    fn dec_ref_count(&mut self) {
        todo!()
    }
}
pub struct TESTopic;
pub struct BGSBaseAlias;

#[repr(C)]
#[derive(Debug)]
pub struct BGSImpactDataSet {
    _data: [u8; 0x08],
}

#[repr(C)]
#[derive(Debug)]
pub struct TESObjectSTAT {
    _data: [u8; 0x08],
}

#[repr(C)]
#[derive(Debug)]
pub struct TESEffectShader {
    _data: [u8; 0x08],
}

#[repr(C)]
#[derive(Debug)]
pub struct BGSDestructibleObjectForm {
    _data: [u8; 0x10], // 0x0D0 - 0x0C0 = 0x10
}
const _: () = assert!(core::mem::size_of::<BGSDestructibleObjectForm>() == 0x10);

#[repr(C)]
#[derive(Debug)]
pub struct BGSPreloadable {
    _data: [u8; 0x08], // 0x0F0 - 0x0E8 = 0x08
}
const _: () = assert!(core::mem::size_of::<BGSPreloadable>() == 0x08);

#[repr(C)]
#[derive(Debug)]
pub struct BGSMessageIcon {
    _data: [u8; 0x18], // 0x110 - 0x0F8 = 0x18
}
const _: () = assert!(core::mem::size_of::<BGSMessageIcon>() == 0x18);

#[repr(C)]
#[derive(Debug)]
pub struct BGSPickupPutdownSounds {
    _data: [u8; 0x18], // 0x128 - 0x110 = 0x18
}
const _: () = assert!(core::mem::size_of::<BGSPickupPutdownSounds>() == 0x18);

#[repr(C)]
#[derive(Debug)]
pub struct BGSBlockBashData {
    _data: [u8; 0x18], // 0x140 - 0x128 = 0x18
}
const _: () = assert!(core::mem::size_of::<BGSBlockBashData>() == 0x18);

/// NOTE: Type unknown even in original implementation.
#[repr(C)]
#[derive(Debug)]
pub struct TESQuestStageItemDoneEvent;

/// TODO: Definition unknown even in the original implementation.
#[repr(C)]
#[derive(Debug)]
pub struct PeriodicUpdateTimer;
pub struct BGSStoryManagerQuestNode;

/// BGSStoryManagerTreeVisitor
#[commonlibsse_ng_derive_internal::ffi_enum]
#[repr(C)]
pub enum VisitControl {
    Dummy,
}

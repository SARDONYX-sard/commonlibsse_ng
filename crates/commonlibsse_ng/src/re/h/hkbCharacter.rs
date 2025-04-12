use crate::re::hkArray::hkArray;
use crate::re::hkRefPtr::hkRefPtr;
use crate::re::hkRefVariant::hkRefVariant;
use crate::re::hkReferencedObject::hkReferencedObject;
use crate::re::hkReferencedObject::hkReferencedObjectVtbl;
use crate::re::hkStringPtr::hkStringPtr;
use crate::re::hkbBehaviorGraph::hkbBehaviorGraph;
use crate::re::hkbCharacterSetup::hkbCharacterSetup;
use crate::re::hkbProjectData::hkbProjectData;
use crate::re::hkbRagdollDriver;
use crate::re::offsets_rtti::RTTI_hkbCharacter;
use crate::re::offsets_vtable::VTABLE_hkbCharacter;
use crate::rel::id::VariantID;

/// Represents a character in the Havok behavior system.
///
/// Inherits from `hkReferencedObject` and contains data for character setup, behavior, and physics.
#[repr(C)]
#[derive(Debug)]
pub struct hkbCharacter {
    /// Base class `hkReferencedObject`.
    pub __base: hkReferencedObject,

    /// Array of pointers to nearby characters.
    /// - Offset: 0x10
    pub nearbyCharacters: hkArray<*mut hkbCharacter>,

    /// Current level of detail.
    /// - Offset: 0x20
    pub currentLOD: i16,

    /// Number of tracks in the current LOD.
    /// - Offset: 0x22
    pub numTracksInLOD: i16,

    /// Padding for alignment.
    /// - Offset: 0x24
    pub pad24: u32,

    /// Character name.
    /// - Offset: 0x28
    pub name: hkStringPtr,

    /// Reference to the ragdoll driver.
    /// - Offset: 0x30
    pub ragdollDriver: hkRefPtr<hkbRagdollDriver>,

    /// Character controller driver.
    /// - Offset: 0x38
    pub characterControllerDriver: hkRefVariant,

    /// Foot inverse kinematics driver.
    /// - Offset: 0x40
    pub footIkDriver: hkRefVariant,

    /// Hand inverse kinematics driver.
    /// - Offset: 0x48
    pub handIkDriver: hkRefVariant,

    /// Character setup reference.
    /// - Offset: 0x50
    pub setup: hkRefPtr<hkbCharacterSetup>,

    /// Behavior graph reference.
    /// - Offset: 0x58
    pub behaviorGraph: hkRefPtr<hkbBehaviorGraph>,

    /// Project data reference.
    /// - Offset: 0x60
    pub projectData: hkRefPtr<hkbProjectData>,

    /// Animation binding set.
    /// - Offset: 0x68
    pub animationBindingSet: hkRefVariant,

    /// Raycast interface.
    /// - Offset: 0x70
    pub raycastInterface: hkRefVariant,

    /// World reference.
    /// - Offset: 0x78
    pub world: hkRefVariant,

    /// Event queue.
    /// - Offset: 0x80
    pub eventQueue: hkRefVariant,

    /// World-from-model transform.
    /// - Offset: 0x88
    pub worldFromModel: hkRefVariant,

    /// Pointer to pose local data (hkSimpleArray<hkRefVariant>).
    /// - Offset: 0x90
    pub poseLocal: *const *const (), // Raw pointer to match const void**

    /// Number of pose local entries.
    /// - Offset: 0x98
    pub numPoseLocal: i32,

    /// Flag indicating whether to delete worldFromModel.
    /// - Offset: 0x9C
    pub deleteWorldFromModel: bool,

    /// Flag indicating whether to delete poseLocal.
    /// - Offset: 0x9D
    pub deletePoseLocal: bool,

    /// Padding for alignment.
    /// - Offset: 0x9E
    pub pad9E: u16,
}

// Compile-time memory layout verification
const _: () = {
    assert!(core::mem::offset_of!(hkbCharacter, __base) == 0x0);
    assert!(core::mem::offset_of!(hkbCharacter, nearbyCharacters) == 0x10);
    assert!(core::mem::offset_of!(hkbCharacter, currentLOD) == 0x20);
    assert!(core::mem::offset_of!(hkbCharacter, numTracksInLOD) == 0x22);
    assert!(core::mem::offset_of!(hkbCharacter, pad24) == 0x24);
    assert!(core::mem::offset_of!(hkbCharacter, name) == 0x28);
    assert!(core::mem::offset_of!(hkbCharacter, ragdollDriver) == 0x30);
    assert!(core::mem::offset_of!(hkbCharacter, characterControllerDriver) == 0x38);
    assert!(core::mem::offset_of!(hkbCharacter, footIkDriver) == 0x40);
    assert!(core::mem::offset_of!(hkbCharacter, handIkDriver) == 0x48);
    assert!(core::mem::offset_of!(hkbCharacter, setup) == 0x50);
    assert!(core::mem::offset_of!(hkbCharacter, behaviorGraph) == 0x58);
    assert!(core::mem::offset_of!(hkbCharacter, projectData) == 0x60);
    assert!(core::mem::offset_of!(hkbCharacter, animationBindingSet) == 0x68);
    assert!(core::mem::offset_of!(hkbCharacter, raycastInterface) == 0x70);
    assert!(core::mem::offset_of!(hkbCharacter, world) == 0x78);
    assert!(core::mem::offset_of!(hkbCharacter, eventQueue) == 0x80);
    assert!(core::mem::offset_of!(hkbCharacter, worldFromModel) == 0x88);
    assert!(core::mem::offset_of!(hkbCharacter, poseLocal) == 0x90);
    assert!(core::mem::offset_of!(hkbCharacter, numPoseLocal) == 0x98);
    assert!(core::mem::offset_of!(hkbCharacter, deleteWorldFromModel) == 0x9C);
    assert!(core::mem::offset_of!(hkbCharacter, deletePoseLocal) == 0x9D);
    assert!(core::mem::offset_of!(hkbCharacter, pad9E) == 0x9E);
    assert!(core::mem::size_of::<hkbCharacter>() == 0xA0);
};

impl Default for hkbCharacter {
    fn default() -> Self {
        Self::new()
    }
}

impl hkbCharacter {
    /// RTTI identifier for this type.
    pub const RTTI: VariantID = RTTI_hkbCharacter;

    /// Virtual function table addresses.
    pub const VTABLE: [VariantID; 1] = VTABLE_hkbCharacter;

    /// Creates a new `hkbCharacter` with default values.
    #[inline]
    pub fn new() -> Self {
        Self {
            __base: hkReferencedObject::new(),
            nearbyCharacters: hkArray::new(),
            currentLOD: 0,
            numTracksInLOD: 0,
            pad24: 0,
            name: hkStringPtr::default(),
            ragdollDriver: hkRefPtr::default(),
            characterControllerDriver: hkRefVariant::default(),
            footIkDriver: hkRefVariant::default(),
            handIkDriver: hkRefVariant::default(),
            setup: hkRefPtr::default(),
            behaviorGraph: hkRefPtr::default(),
            projectData: hkRefPtr::default(),
            animationBindingSet: hkRefVariant::default(),
            raycastInterface: hkRefVariant::default(),
            world: hkRefVariant::default(),
            eventQueue: hkRefVariant::default(),
            worldFromModel: hkRefVariant::default(),
            poseLocal: std::ptr::null(), // Null pointer for const void**
            numPoseLocal: 0,
            deleteWorldFromModel: false,
            deletePoseLocal: false,
            pad9E: 0,
        }
    }
}

/// Virtual function table for `hkbCharacter`.
#[repr(C)]
pub struct hkbCharacterVtbl {
    pub __base: hkReferencedObjectVtbl,

    /// Unknown function (placeholder).
    pub Unk_03: fn(this: &mut hkbCharacter),

    /// Unknown function (placeholder).
    pub Unk_04: fn(this: &mut hkbCharacter),
}

use crate::re::hkBaseTypes::hkHalf;
use crate::re::hkMatrix3::hkMatrix3;
use crate::re::hkMotionState::hkMotionState;
use crate::re::hkQuaternion::hkQuaternion;
use crate::re::hkReferencedObject::hkReferencedObject;
use crate::re::hkTransform::hkTransform;
use crate::re::hkVector4::hkVector4;
use crate::re::hkpKeyframedRigidMotion::hkpMaxSizeMotion;
use crate::re::offsets_rtti::RTTI_hkpMotion;
use crate::re::offsets_vtable::VTABLE_hkpMotion;
use crate::re::{hkClass, hkStatisticsCollector};
use crate::rel::id::VariantID;

/// Represents a physics motion object in the Havok system.
///
/// Inherits from `hkReferencedObject` and manages motion properties like mass, inertia, and velocity.
#[repr(C)]
#[derive(Debug)]
pub struct hkpMotion {
    /// Base class `hkReferencedObject`.
    pub __base: hkReferencedObject,

    /// Motion type enumeration.
    /// - Offset: 0x010
    pub type_: MotionType_CEnum,

    /// Counter for deactivation integration.
    /// - Offset: 0x011
    pub deactivationIntegrateCounter: u8,

    /// Number of inactive frames for deactivation.
    /// - Offset: 0x012
    pub deactivationNumInactiveFrames: [u16; 2],

    /// Padding to align memory.
    /// - Offset: 0x016
    pub pad016: u16,

    /// Padding to align memory.
    /// - Offset: 0x018
    pub pad018: u64,

    /// Motion state data.
    /// - Offset: 0x020
    pub motionState: hkMotionState,

    /// Inertia tensor and inverse mass.
    /// - Offset: 0x0D0
    pub inertiaAndMassInv: hkVector4,

    /// Linear velocity vector.
    /// - Offset: 0x0E0
    pub linearVelocity: hkVector4,

    /// Angular velocity vector.
    /// - Offset: 0x0F0
    pub angularVelocity: hkVector4,

    /// Deactivation reference positions.
    /// - Offset: 0x100
    pub deactivationRefPosition: [hkVector4; 2],

    /// Deactivation reference orientations.
    /// - Offset: 0x120
    pub deactivationRefOrientation: [u32; 2],

    /// Pointer to saved motion data.
    /// - Offset: 0x128
    pub savedMotion: *mut hkpMaxSizeMotion,

    /// Saved quality type index.
    /// - Offset: 0x130
    pub savedQualityTypeIndex: u16,

    /// Padding to align memory.
    /// - Offset: 0x132
    pub pad132: u16,

    /// Gravity factor.
    /// - Offset: 0x134
    pub gravityFactor: hkHalf,

    /// Padding to align memory.
    /// - Offset: 0x138
    pub pad138: u64,
}

// Compile-time memory layout verification
const _: () = {
    assert!(core::mem::offset_of!(hkpMotion, __base) == 0x0);
    assert!(core::mem::offset_of!(hkpMotion, type_) == 0x010);
    assert!(core::mem::offset_of!(hkpMotion, deactivationIntegrateCounter) == 0x011);
    assert!(core::mem::offset_of!(hkpMotion, deactivationNumInactiveFrames) == 0x012);
    assert!(core::mem::offset_of!(hkpMotion, pad016) == 0x016);
    assert!(core::mem::offset_of!(hkpMotion, pad018) == 0x018);
    assert!(core::mem::offset_of!(hkpMotion, motionState) == 0x020);
    assert!(core::mem::offset_of!(hkpMotion, inertiaAndMassInv) == 0x0D0);
    assert!(core::mem::offset_of!(hkpMotion, linearVelocity) == 0x0E0);
    assert!(core::mem::offset_of!(hkpMotion, angularVelocity) == 0x0F0);
    assert!(core::mem::offset_of!(hkpMotion, deactivationRefPosition) == 0x100);
    assert!(core::mem::offset_of!(hkpMotion, deactivationRefOrientation) == 0x120);
    assert!(core::mem::offset_of!(hkpMotion, savedMotion) == 0x128);
    assert!(core::mem::offset_of!(hkpMotion, savedQualityTypeIndex) == 0x130);
    assert!(core::mem::offset_of!(hkpMotion, pad132) == 0x132);
    assert!(core::mem::offset_of!(hkpMotion, gravityFactor) == 0x134);
    assert!(core::mem::offset_of!(hkpMotion, pad138) == 0x138);
    assert!(core::mem::size_of::<hkpMotion>() == 0x140);
};

/// Enumeration of motion types for `hkpMotion`.
#[commonlibsse_ng_derive_internal::ffi_enum]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum MotionType {
    Invalid = 0,
    Dynamic = 1,
    SphereInertia = 2,
    BoxInertia = 3,
    Keyframed = 4,
    Fixed = 5,
    ThinBoxInertia = 6,
    Character = 7,
    Total = 8,
}

impl hkpMotion {
    /// RTTI identifier for this type.
    pub const RTTI: VariantID = RTTI_hkpMotion;

    /// Virtual function table addresses.
    pub const VTABLE: [VariantID; 1] = VTABLE_hkpMotion;

    /// Number of inactive frames required to deactivate.
    pub const NUM_INACTIVE_FRAMES_TO_DEACTIVATE: i32 = 5;

    /// Creates a new `hkpMotion` instance with default values.
    #[inline]
    pub fn new() -> Self {
        Self {
            __base: hkReferencedObject::new(),
            type_: MotionType_CEnum::Invalid,
            deactivationIntegrateCounter: 0,
            deactivationNumInactiveFrames: [0; 2],
            pad016: 0,
            pad018: 0,
            motionState: hkMotionState::default(),
            inertiaAndMassInv: hkVector4::default(),
            linearVelocity: hkVector4::default(),
            angularVelocity: hkVector4::default(),
            deactivationRefPosition: [hkVector4::default(); 2],
            deactivationRefOrientation: [0; 2],
            savedMotion: core::ptr::null_mut(),
            savedQualityTypeIndex: 0,
            pad132: 0,
            gravityFactor: hkHalf::default(), // Assuming hkHalf has a default
            pad138: 0,
        }
    }

    /// Gets the mass of the motion object.
    #[inline]
    pub fn GetMass(&self) -> f32 {
        let mass_inv = self.inertiaAndMassInv.get_component(3); // Assuming hkVector4 has a method to access components
        if mass_inv != 0.0 { 1.0 / mass_inv } else { 0.0 }
    }
}

/// Virtual function table for `hkpMotion`.
#[repr(C)]
pub struct hkpMotionVtbl {
    /// Destructor function pointer.
    pub CxxDrop: fn(this: &mut hkpMotion),

    /// Gets the class type (inherited from hkReferencedObject).
    pub GetClassType: fn(this: &hkpMotion) -> Option<&hkClass>,

    /// Calculates content statistics (inherited from hkReferencedObject).
    pub CalcContentStatistics:
        fn(this: &hkpMotion, collector: &mut hkStatisticsCollector, cls: Option<&hkClass>),

    /// Sets the mass.
    pub SetMass: fn(this: &mut hkpMotion, a_mass: f32),

    /// Sets the inverse mass.
    pub SetMassInv: fn(this: &mut hkpMotion, a_massInv: f32),

    /// Gets the local inertia tensor.
    pub GetInertiaLocal: fn(this: &hkpMotion, a_inertiaOut: &mut hkMatrix3),

    /// Gets the world inertia tensor.
    pub GetInertiaWorld: fn(this: &hkpMotion, a_inertiaOut: &mut hkMatrix3),

    /// Sets the local inertia tensor.
    pub SetInertiaLocal: fn(this: &mut hkpMotion, a_inertia: &hkMatrix3),

    /// Sets the local inverse inertia tensor.
    pub SetInertiaInvLocal: fn(this: &mut hkpMotion, a_inertiaInv: &hkMatrix3),

    /// Gets the local inverse inertia tensor.
    pub GetInertiaInvLocal: fn(this: &hkpMotion, a_inertiaInvOut: &mut hkMatrix3),

    /// Gets the world inverse inertia tensor.
    pub GetInertiaInvWorld: fn(this: &hkpMotion, a_inertiaInvOut: &mut hkMatrix3),

    /// Sets the center of mass in local space.
    pub SetCenterOfMassInLocal: fn(this: &mut hkpMotion, a_centerOfMass: hkVector4),

    /// Sets the position.
    pub SetPosition: fn(this: &mut hkpMotion, a_position: hkVector4),

    /// Sets the rotation.
    pub SetRotation: fn(this: &mut hkpMotion, a_rotation: hkQuaternion),

    /// Sets both position and rotation.
    pub SetPositionAndRotation:
        fn(this: &mut hkpMotion, a_position: hkVector4, a_rotation: hkQuaternion),

    /// Sets the transform.
    pub SetTransform: fn(this: &mut hkpMotion, a_transform: hkTransform),

    /// Sets the linear velocity.
    pub SetLinearVelocity: fn(this: &mut hkpMotion, a_newVel: hkVector4),

    /// Sets the angular velocity.
    pub SetAngularVelocity: fn(this: &mut hkpMotion, a_newVel: hkVector4),

    /// Gets the projected point velocity.
    pub GetProjectedPointVelocity: fn(
        this: &hkpMotion,
        a_point: hkVector4,
        a_normal: hkVector4,
        a_velOut: &mut f32,
        a_invVirtMassOut: &mut f32,
    ),

    /// Applies a linear impulse.
    pub ApplyLinearImpulse: fn(this: &mut hkpMotion, a_impulse: hkVector4),

    /// Applies a point impulse.
    pub ApplyPointImpulse: fn(this: &mut hkpMotion, a_impulse: hkVector4, a_point: hkVector4),

    /// Applies an angular impulse.
    pub ApplyAngularImpulse: fn(this: &mut hkpMotion, a_impulse: hkVector4),

    /// Applies a force.
    pub ApplyForce: fn(this: &mut hkpMotion, a_deltaTime: f32, a_force: hkVector4),

    /// Applies a force at a point.
    pub ApplyForceAtPoint:
        fn(this: &mut hkpMotion, a_deltaTime: f32, a_force: hkVector4, a_point: hkVector4),

    /// Applies a torque.
    pub ApplyTorque: fn(this: &mut hkpMotion, a_deltaTime: f32, a_torque: hkVector4),

    /// Gets motion state, velocities, and deactivation type.
    pub GetMotionStateAndVelocitiesAndDeactivationType:
        fn(this: &hkpMotion, a_motionOut: &mut hkpMotion),
}

impl Default for hkpMotion {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

use core::ops::{Add as _, Sub as _};

use crate::re::hkMatrix3::hkMatrix3;
use crate::re::hkVector4::hkVector4;
use crate::re::hkpMotion::{MotionType, hkpMotion};
use crate::re::offsets_rtti::RTTI_hkpKeyframedRigidMotion;
use crate::re::offsets_rtti::RTTI_hkpMaxSizeMotion;
use crate::re::offsets_vtable::VTABLE_hkpKeyframedRigidMotion;
use crate::re::{hkClass, hkStatisticsCollector};
use crate::rel::id::VariantID;

use super::hkQuaternion::hkQuaternion;
use super::hkTransform::hkTransform;

/// Represents a keyframed rigid motion in the Havok system.
///
/// Inherits from `hkpMotion` and provides overrides for a keyframed (non-dynamic) motion type.
///
/// # Memory Layout:
/// - `__base`: Base class `hkpMotion` (0x00 - 0x13F)
#[repr(C)]
#[derive(Debug)]
pub struct hkpKeyframedRigidMotion {
    /// Base class `hkpMotion`.
    /// - Offset: 0x00
    pub __base: hkpMotion,
}

// Compile-time memory layout verification
const _: () = {
    assert!(core::mem::offset_of!(hkpKeyframedRigidMotion, __base) == 0x0);
    assert!(core::mem::size_of::<hkpKeyframedRigidMotion>() == 0x140);
};

impl hkpKeyframedRigidMotion {
    /// RTTI identifier for this type.
    pub const RTTI: VariantID = RTTI_hkpKeyframedRigidMotion;

    /// Virtual function table addresses.
    pub const VTABLE: [VariantID; 1] = VTABLE_hkpKeyframedRigidMotion;

    /// Creates a new `hkpKeyframedRigidMotion` with default values.
    #[inline]
    pub fn new() -> Self {
        let mut motion = hkpMotion::new();
        motion.type_ = MotionType::Keyframed.into(); // Set to keyframed motion type
        Self { __base: motion }
    }

    /// Computes the velocity of a point in world space.
    #[inline]
    pub fn GetPointVelocity(&self, a_point: &hkVector4) -> hkVector4 {
        let center_of_mass_in_world = self.__base.motionState.sweptTransform.centerOfMass1;
        let relative_point = a_point.sub(center_of_mass_in_world);
        self.__base.linearVelocity.add(self.__base.angularVelocity.Cross(relative_point))
    }
}

/// Virtual function table for `hkpKeyframedRigidMotion`.
#[repr(C)]
pub struct hkpKeyframedRigidMotionVtbl {
    /// Destructor function pointer.
    pub CxxDrop: fn(this: &mut hkpKeyframedRigidMotion),

    /// Gets the class type (inherited from hkReferencedObject).
    pub GetClassType: fn(this: &hkpKeyframedRigidMotion) -> Option<&hkClass>,

    /// Calculates content statistics (inherited from hkReferencedObject).
    pub CalcContentStatistics: fn(
        this: &hkpKeyframedRigidMotion,
        collector: &mut hkStatisticsCollector,
        cls: Option<&hkClass>,
    ),

    /// Sets the mass (no-op for keyframed motion).
    pub SetMass: fn(this: &mut hkpKeyframedRigidMotion, a_mass: f32),

    /// Sets the inverse mass (no-op for keyframed motion).
    pub SetMassInv: fn(this: &mut hkpKeyframedRigidMotion, a_massInv: f32),

    /// Gets the local inertia tensor.
    pub GetInertiaLocal: fn(this: &hkpKeyframedRigidMotion, a_inertiaOut: &mut hkMatrix3),

    /// Gets the world inertia tensor.
    pub GetInertiaWorld: fn(this: &hkpKeyframedRigidMotion, a_inertiaOut: &mut hkMatrix3),

    /// Sets the local inertia tensor (no-op for keyframed motion).
    pub SetInertiaLocal: fn(this: &mut hkpKeyframedRigidMotion, a_inertia: &hkMatrix3),

    /// Sets the local inverse inertia tensor (no-op for keyframed motion).
    pub SetInertiaInvLocal: fn(this: &mut hkpKeyframedRigidMotion, a_inertiaInv: &hkMatrix3),

    /// Gets the local inverse inertia tensor.
    pub GetInertiaInvLocal: fn(this: &hkpKeyframedRigidMotion, a_inertiaInvOut: &mut hkMatrix3),

    /// Gets the world inverse inertia tensor.
    pub GetInertiaInvWorld: fn(this: &hkpKeyframedRigidMotion, a_inertiaInvOut: &mut hkMatrix3),

    /// Sets the center of mass in local space (inherited from hkpMotion).
    pub SetCenterOfMassInLocal: fn(this: &mut hkpKeyframedRigidMotion, a_centerOfMass: hkVector4),

    /// Sets the position (inherited from hkpMotion).
    pub SetPosition: fn(this: &mut hkpKeyframedRigidMotion, a_position: hkVector4),

    /// Sets the rotation (inherited from hkpMotion).
    pub SetRotation: fn(this: &mut hkpKeyframedRigidMotion, a_rotation: hkQuaternion),

    /// Sets both position and rotation (inherited from hkpMotion).
    pub SetPositionAndRotation:
        fn(this: &mut hkpKeyframedRigidMotion, a_position: hkVector4, a_rotation: hkQuaternion),

    /// Sets the transform (inherited from hkpMotion).
    pub SetTransform: fn(this: &mut hkpKeyframedRigidMotion, a_transform: hkTransform),

    /// Sets the linear velocity (inherited from hkpMotion).
    pub SetLinearVelocity: fn(this: &mut hkpKeyframedRigidMotion, a_newVel: hkVector4),

    /// Sets the angular velocity (inherited from hkpMotion).
    pub SetAngularVelocity: fn(this: &mut hkpKeyframedRigidMotion, a_newVel: hkVector4),

    /// Gets the projected point velocity.
    pub GetProjectedPointVelocity: fn(
        this: &hkpKeyframedRigidMotion,
        a_point: hkVector4,
        a_normal: hkVector4,
        a_velOut: &mut f32,
        a_invVirtMassOut: &mut f32,
    ),

    /// Applies a linear impulse (no-op for keyframed motion).
    pub ApplyLinearImpulse: fn(this: &mut hkpKeyframedRigidMotion, a_impulse: hkVector4),

    /// Applies a point impulse (no-op for keyframed motion).
    pub ApplyPointImpulse:
        fn(this: &mut hkpKeyframedRigidMotion, a_impulse: hkVector4, a_point: hkVector4),

    /// Applies an angular impulse (no-op for keyframed motion).
    pub ApplyAngularImpulse: fn(this: &mut hkpKeyframedRigidMotion, a_impulse: hkVector4),

    /// Applies a force (no-op for keyframed motion).
    pub ApplyForce: fn(this: &mut hkpKeyframedRigidMotion, a_deltaTime: f32, a_force: hkVector4),

    /// Applies a force at a point (no-op for keyframed motion).
    pub ApplyForceAtPoint: fn(
        this: &mut hkpKeyframedRigidMotion,
        a_deltaTime: f32,
        a_force: hkVector4,
        a_point: hkVector4,
    ),

    /// Applies a torque (no-op for keyframed motion).
    pub ApplyTorque: fn(this: &mut hkpKeyframedRigidMotion, a_deltaTime: f32, a_torque: hkVector4),

    /// Gets motion state, velocities, and deactivation type (inherited from hkpMotion).
    pub GetMotionStateAndVelocitiesAndDeactivationType:
        fn(this: &hkpKeyframedRigidMotion, a_motionOut: &mut hkpMotion),

    /// Sets the step position (virtual function).
    pub SetStepPosition: fn(this: &mut hkpKeyframedRigidMotion, a_position: f32, a_timestep: f32),

    /// Sets the stored motion (virtual function).
    pub SetStoredMotion:
        fn(this: &mut hkpKeyframedRigidMotion, a_savedMotion: Option<&mut hkpMaxSizeMotion>),
}

impl Default for hkpKeyframedRigidMotion {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a maximum-size keyframed rigid motion in the Havok system.
///
/// Inherits from `hkpKeyframedRigidMotion` without adding new fields.
///
/// # Memory Layout:
/// - `__base`: Base class `hkpKeyframedRigidMotion` (0x00 - 0x13F)
#[repr(C)]
pub struct hkpMaxSizeMotion {
    /// Base class `hkpKeyframedRigidMotion`.
    /// - Offset: 0x00
    pub __base: hkpKeyframedRigidMotion,
}

// Compile-time memory layout verification
const _: () = {
    assert!(core::mem::offset_of!(hkpMaxSizeMotion, __base) == 0x0);
    assert!(core::mem::size_of::<hkpMaxSizeMotion>() == 0x140);
};

impl hkpMaxSizeMotion {
    /// RTTI identifier for this type.
    pub const RTTI: VariantID = RTTI_hkpMaxSizeMotion;

    /// Creates a new `hkpMaxSizeMotion` with default values.
    #[inline]
    pub fn new() -> Self {
        Self { __base: hkpKeyframedRigidMotion::new() }
    }
}

impl Default for hkpMaxSizeMotion {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

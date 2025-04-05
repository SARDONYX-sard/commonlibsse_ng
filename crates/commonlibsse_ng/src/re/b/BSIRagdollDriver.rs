use crate::re::bhkWorld::bhkWorld;
use crate::re::hkpMotion::MotionType as hkpMotionType;
use crate::re::offsets_rtti::RTTI_BSIRagdollDriver;
use crate::re::offsets_vtable::VTABLE_BSIRagdollDriver;
use crate::rel::id::VariantID;

/// Represents an interface for a ragdoll driver in the Skyrim engine.
///
/// This is an abstract base class with pure virtual functions for managing ragdoll physics.
///
/// # Memory Layout:
/// - Pure virtual class, size is 0x8 (vtable pointer only)
#[repr(C)]
#[derive(Debug)]
pub struct BSIRagdollDriver {
    // No fields; this is an abstract class with only a vtable in C++
    _vtable: *const BSIRagdollDriverVtbl, // Placeholder for vtable pointer; not directly accessible in Rust
}

// Compile-time memory layout verification
const _: () = {
    assert!(core::mem::size_of::<BSIRagdollDriver>() == 0x8); // Matches sizeof(void*) on 64-bit
};

impl BSIRagdollDriver {
    /// RTTI identifier for this type.
    pub const RTTI: VariantID = RTTI_BSIRagdollDriver;

    /// Virtual function table addresses.
    pub const VTABLE: [VariantID; 1] = VTABLE_BSIRagdollDriver;
}

/// Virtual function table for `BSIRagdollDriver`.
#[repr(C)]
pub struct BSIRagdollDriverVtbl {
    /// Destructor function pointer.
    pub CxxDrop: fn(this: &mut BSIRagdollDriver),

    /// Checks if a ragdoll exists.
    pub HasRagdoll: fn(this: &BSIRagdollDriver) -> bool,

    /// Adds the ragdoll to the world.
    pub AddRagdollToWorld: fn(this: &mut BSIRagdollDriver) -> bool,

    /// Removes the ragdoll from the world.
    pub RemoveRagdollFromWorld: fn(this: &mut BSIRagdollDriver) -> bool,

    /// Sets the world for the ragdoll driver.
    pub SetWorld: fn(this: &mut BSIRagdollDriver, world: Option<&mut bhkWorld>),

    /// Resets the ragdoll state.
    pub ResetRagdoll: fn(this: &mut BSIRagdollDriver),

    /// Unknown function (placeholder).
    pub Unk_06: fn(this: &mut BSIRagdollDriver),

    /// Sets ragdoll constraints from bhk constraints.
    pub SetRagdollConstraintsFromBhkConstraints: fn(this: &mut BSIRagdollDriver),

    /// Sets the motion type for the ragdoll.
    pub SetMotionType: fn(this: &mut BSIRagdollDriver, motionType: hkpMotionType),

    /// Unknown function (placeholder).
    pub Unk_09: fn(this: &mut BSIRagdollDriver),

    /// Toggles synchronization on update.
    pub ToggleSyncOnUpdate: fn(this: &mut BSIRagdollDriver, disable: bool),

    /// Unknown function (placeholder).
    pub Unk_0B: fn(this: &mut BSIRagdollDriver),

    /// Toggles constraints on or off.
    pub ToggleConstraints: fn(this: &mut BSIRagdollDriver, disable: bool),

    /// Unknown function (placeholder).
    pub Unk_0D: fn(this: &mut BSIRagdollDriver),
}

use crate::re::hkVector4::hkVector4;

/// Represents a quaternion in the Havok system.
///
/// This struct wraps a single `hkVector4` where the components typically represent
/// (x, y, z, w) with w being the real part and (x, y, z) the imaginary vector part.
///
/// # Memory Layout:
/// - `vec`: The quaternion as a 4D vector (0x00 - 0x0F)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct hkQuaternion {
    /// The 4D vector storing the quaternion components (x, y, z, w).
    /// - Offset: 0x00
    pub vec: hkVector4,
}

// Compile-time memory layout verification
const _: () = {
    assert!(core::mem::offset_of!(hkQuaternion, vec) == 0x0);
    assert!(core::mem::size_of::<hkQuaternion>() == 0x10);
};

impl hkQuaternion {
    /// Creates a new `hkQuaternion` with all components set to zero (invalid quaternion).
    #[inline]
    pub fn new() -> Self {
        Self { vec: hkVector4::new() }
    }

    /// Creates a new identity quaternion (w = 1, x = y = z = 0).
    #[inline]
    pub fn identity() -> Self {
        Self { vec: hkVector4::from_components(0.0, 0.0, 0.0, 1.0) }
    }
}

impl Default for hkQuaternion {
    #[inline]
    fn default() -> Self {
        Self::identity() // Default to identity quaternion, which is more useful than all zeros
    }
}

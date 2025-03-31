use crate::re::hkRotation::hkRotation;
use crate::re::hkVector4::hkVector4;

/// Represents a 3D transform in the Havok system, combining rotation and translation.
///
/// This struct consists of a rotation matrix and a translation vector.
///
/// # Memory Layout:
/// - `rotation`: 3x3 rotation matrix (0x00 - 0x2F)
/// - `translation`: 4D translation vector (0x30 - 0x3F)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct hkTransform {
    /// The rotation component of the transform.
    /// - Offset: 0x00
    pub rotation: hkRotation,

    /// The translation component of the transform.
    /// - Offset: 0x30
    pub translation: hkVector4,
}

// Compile-time memory layout verification
const _: () = {
    assert!(core::mem::offset_of!(hkTransform, rotation) == 0x0);
    assert!(core::mem::offset_of!(hkTransform, translation) == 0x30);
    assert!(core::mem::size_of::<hkTransform>() == 0x40);
};

impl hkTransform {
    /// Creates a new `hkTransform` with an identity rotation and zero translation.
    #[inline]
    pub fn new() -> Self {
        Self {
            rotation: hkRotation::new(),   // Assuming hkRotation has a new() method
            translation: hkVector4::new(), // Zero translation
        }
    }
}

impl Default for hkTransform {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

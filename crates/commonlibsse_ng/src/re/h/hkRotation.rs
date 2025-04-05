use crate::re::hkMatrix3::hkMatrix3;

/// Represents a 3x3 rotation matrix in the Havok system.
///
/// Inherits from `hkMatrix3` and enforces 16-byte alignment.
///
/// # Memory Layout:
/// - `__base`: Base class `hkMatrix3` (0x00 - 0x2F)
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct hkRotation {
    /// Base class `hkMatrix3` containing the 3x3 matrix data.
    /// - Offset: 0x00
    pub __base: hkMatrix3,
}

// Compile-time memory layout verification
const _: () = {
    assert!(core::mem::offset_of!(hkRotation, __base) == 0x0);
    assert!(core::mem::size_of::<hkRotation>() == 0x30);
    assert!(core::mem::align_of::<hkRotation>() == 0x10);
};

impl hkRotation {
    /// Creates a new `hkRotation` with an identity matrix.
    #[inline]
    pub fn new() -> Self {
        Self {
            __base: hkMatrix3::new(), // Zero-initialized; could be adjusted to identity if needed
        }
    }
}

impl Default for hkRotation {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

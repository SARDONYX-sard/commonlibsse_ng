use crate::re::hkVector4::hkVector4;

/// Represents a 3x3 matrix in the Havok system.
///
/// This struct stores three `hkVector4` columns, where each column represents a 4D vector
/// (x, y, z, w), though typically only the first three components (x, y, z) are used for 3x3 matrix operations.
///
/// # Memory Layout:
/// - `col0`: First column vector (0x00 - 0x0F)
/// - `col1`: Second column vector (0x10 - 0x1F)
/// - `col2`: Third column vector (0x20 - 0x2F)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct hkMatrix3 {
    /// First column of the matrix.
    /// - Offset: 0x00
    pub col0: hkVector4,

    /// Second column of the matrix.
    /// - Offset: 0x10
    pub col1: hkVector4,

    /// Third column of the matrix.
    /// - Offset: 0x20
    pub col2: hkVector4,
}

// Compile-time memory layout verification
const _: () = {
    assert!(core::mem::offset_of!(hkMatrix3, col0) == 0x0);
    assert!(core::mem::offset_of!(hkMatrix3, col1) == 0x10);
    assert!(core::mem::offset_of!(hkMatrix3, col2) == 0x20);
    assert!(core::mem::size_of::<hkMatrix3>() == 0x30);
};

impl hkMatrix3 {
    /// Creates a new `hkMatrix3` with all components set to zero.
    #[inline]
    pub fn new() -> Self {
        Self { col0: hkVector4::new(), col1: hkVector4::new(), col2: hkVector4::new() }
    }
}

impl Default for hkMatrix3 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

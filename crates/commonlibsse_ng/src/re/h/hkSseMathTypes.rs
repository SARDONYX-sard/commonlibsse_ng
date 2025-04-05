use core::arch::x86_64::__m128;

/// Alias for a 128-bit SSE vector type used in Havok.
///
/// This corresponds to `__m128` from the C++ code.
pub type hkQuadReal = __m128;

// Compile-time size verification for hkQuadReal
const _: () = {
    assert!(core::mem::size_of::<hkQuadReal>() == 0x10);
};

/// Represents a comparison mask for `hkVector4` operations in the Havok system.
///
/// This struct uses a `hkQuadReal` to store a mask for vector component comparisons.
///
/// # Memory Layout:
/// - `mask`: SSE-aligned 128-bit vector (0x00 - 0x0F)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct hkVector4Comparison {
    /// The 128-bit SSE vector used as a comparison mask.
    /// - Offset: 0x00
    pub mask: hkQuadReal,
}

// Compile-time memory layout verification
const _: () = {
    assert!(core::mem::offset_of!(hkVector4Comparison, mask) == 0x0);
    assert!(core::mem::size_of::<hkVector4Comparison>() == 0x10);
};

impl Default for hkVector4Comparison {
    #[inline]
    fn default() -> Self {
        Self {
            mask: unsafe { core::arch::x86_64::_mm_setzero_ps() }, // Zero-initialized mask
        }
    }
}

/// Enumeration of mask values for component-wise comparisons.
#[commonlibsse_ng_derive_internal::to_bitflags]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mask {
    #[default]
    None = 0,

    X = 1,
    Y = 2,
    XY = 3,

    Z = 4,
    XZ = 5,
    YZ = 6,
    XYZ = 7,

    W = 8,
    XW = 9,
    YW = 10,
    XYW = 11,

    ZW = 12,
    XZW = 13,
    YZW = 14,
    XYZW = 15,
}

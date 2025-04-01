use crate::re::NiPoint3::NiPoint3;
use crate::re::hkSseMathTypes::hkQuadReal;
use core::arch::x86_64::{
    __m128, _mm_add_ps, _mm_cvtss_f32, _mm_div_ps, _mm_hadd_ps, _mm_mul_ps, _mm_set1_ps,
    _mm_setr_ps, _mm_setzero_ps, _mm_sub_ps,
};
use core::ops::{Add, Div, Mul, Sub};

/// Represents a 4D vector using SSE-aligned floating-point values in the Havok system.
///
/// This struct wraps a `hkQuadReal` (__m128) for vector operations.
///
/// # Memory Layout:
/// - `quad`: SSE-aligned 128-bit vector (0x00 - 0x0F)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct hkVector4 {
    /// The 128-bit SSE vector containing x, y, z, and w components.
    /// - Offset: 0x00
    pub quad: hkQuadReal,
}

// Compile-time memory layout verification
const _: () = {
    assert!(core::mem::offset_of!(hkVector4, quad) == 0x0);
    assert!(core::mem::size_of::<hkVector4>() == 0x10);
};

impl hkVector4 {
    /// Creates a new `hkVector4` with all components set to 0.0.
    #[inline]
    pub fn new() -> Self {
        unsafe { Self { quad: _mm_setzero_ps() } }
    }

    /// Creates a new `hkVector4` with all components set to the same value.
    #[inline]
    pub fn from_scalar(x: f32) -> Self {
        unsafe { Self { quad: _mm_set1_ps(x) } }
    }

    /// Creates a new `hkVector4` from individual x, y, z, and w components.
    #[inline]
    pub fn from_components(x: f32, y: f32, z: f32, w: f32) -> Self {
        unsafe { Self { quad: _mm_setr_ps(x, y, z, w) } }
    }

    /// Creates a new `hkVector4` from an `NiPoint3`, with w set to 0.0.
    #[inline]
    pub fn from_ni_point3(point: NiPoint3) -> Self {
        unsafe { Self { quad: _mm_setr_ps(point.x, point.y, point.z, 0.0) } }
    }

    /// Assigns the value of another `hkVector4` to this one.
    #[inline]
    pub fn assign(&mut self, rhs: Self) {
        self.quad = rhs.quad;
    }

    /// Checks if this vector is equal to another within a given epsilon.
    #[inline]
    pub fn IsEqual(&self, pt: Self, epsilon: f32) -> bool {
        let diff = self.sub(pt);
        diff.SqrLength4() < epsilon * epsilon
    }

    /// Computes the cross product with another `hkVector4` (using x, y, z components).
    #[inline]
    pub fn Cross(&self, pt: Self) -> Self {
        let x1 = self.get_component(0);
        let y1 = self.get_component(1);
        let z1 = self.get_component(2);
        let x2 = pt.get_component(0);
        let y2 = pt.get_component(1);
        let z2 = pt.get_component(2);
        Self::from_components(y1.mul_add(z2, -(z1 * y2)), z1.mul_add(x2, -(x1 * z2)), x1.mul_add(y2, -(y1 * x2)), 0.0)
    }

    /// Computes the 3D dot product with another `hkVector4` (ignoring w).
    #[inline]
    pub fn Dot3(&self, pt: Self) -> f32 {
        let prod = self.mul(pt);
        prod.get_component(0) + prod.get_component(1) + prod.get_component(2)
    }

    /// Computes the 4D dot product with another `hkVector4`.
    #[inline]
    pub fn Dot4(&self, pt: Self) -> f32 {
        unsafe {
            let prod = _mm_mul_ps(self.quad, pt.quad);
            let sum1 = _mm_hadd_ps(prod, prod); // Horizontal add (x+y, z+w, x+y, z+w)
            let sum2 = _mm_hadd_ps(sum1, sum1); // (x+y+z+w, x+y+z+w, x+y+z+w, x+y+z+w)
            _mm_cvtss_f32(sum2) // Extract first component
        }
    }

    /// Gets the 3D distance to another `hkVector4`.
    #[inline]
    pub fn GetDistance3(&self, pt: Self) -> f32 {
        self.GetSquaredDistance3(pt).sqrt()
    }

    /// Gets the squared 3D distance to another `hkVector4`.
    #[inline]
    pub fn GetSquaredDistance3(&self, pt: Self) -> f32 {
        let diff = self.sub(pt);
        diff.SqrLength3()
    }

    /// Gets the 3D length of the vector.
    #[inline]
    pub fn Length3(&self) -> f32 {
        self.SqrLength3().sqrt()
    }

    /// Gets the squared 3D length of the vector.
    #[inline]
    pub fn SqrLength3(&self) -> f32 {
        let x = self.get_component(0);
        let y = self.get_component(1);
        let z = self.get_component(2);
        z.mul_add(z, x.mul_add(x, y * y))
    }

    /// Gets the 4D length of the vector.
    #[inline]
    pub fn Length4(&self) -> f32 {
        self.SqrLength4().sqrt()
    }

    /// Gets the squared 4D length of the vector.
    #[inline]
    pub fn SqrLength4(&self) -> f32 {
        unsafe {
            let sq = _mm_mul_ps(self.quad, self.quad);
            let sum1 = _mm_hadd_ps(sq, sq);
            let sum2 = _mm_hadd_ps(sum1, sum1);
            _mm_cvtss_f32(sum2)
        }
    }

    /// Gets a component of the vector by index (0 = x, 1 = y, 2 = z, 3 = w).
    #[inline]
    pub fn get_component(&self, index: usize) -> f32 {
        unsafe {
            let arr = core::mem::transmute::<__m128, [f32; 4]>(self.quad);
            arr[index]
        }
    }

    /// Sets a component of the vector by index (0 = x, 1 = y, 2 = z, 3 = w).
    #[inline]
    pub fn set_component(&mut self, index: usize, value: f32) {
        unsafe {
            let mut arr = core::mem::transmute::<__m128, [f32; 4]>(self.quad);
            arr[index] = value;
            self.quad = core::mem::transmute::<[f32; 4], __m128>(arr);
        }
    }
}

impl Default for hkVector4 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl From<hkQuadReal> for hkVector4 {
    #[inline]
    fn from(quad: hkQuadReal) -> Self {
        Self { quad }
    }
}

impl From<NiPoint3> for hkVector4 {
    #[inline]
    fn from(point: NiPoint3) -> Self {
        Self::from_ni_point3(point)
    }
}

// Operator overloads using traits
impl core::ops::Add for hkVector4 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        unsafe { Self { quad: _mm_add_ps(self.quad, rhs.quad) } }
    }
}

impl core::ops::Sub for hkVector4 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        unsafe { Self { quad: _mm_sub_ps(self.quad, rhs.quad) } }
    }
}

impl core::ops::Mul for hkVector4 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        unsafe { Self { quad: _mm_mul_ps(self.quad, rhs.quad) } }
    }
}

impl core::ops::Div for hkVector4 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        unsafe { Self { quad: _mm_div_ps(self.quad, rhs.quad) } }
    }
}

impl core::ops::AddAssign for hkVector4 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.quad = self.add(rhs).quad;
    }
}

impl core::ops::SubAssign for hkVector4 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.quad = self.sub(rhs).quad;
    }
}

impl core::ops::MulAssign for hkVector4 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.quad = self.mul(rhs).quad;
    }
}

impl core::ops::DivAssign for hkVector4 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.quad = self.div(rhs).quad;
    }
}

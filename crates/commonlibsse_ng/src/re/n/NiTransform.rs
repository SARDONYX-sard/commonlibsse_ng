use crate::re::NiMatrix3::NiMatrix3;
use crate::re::NiPoint3::NiPoint3;

/// Represents a 3D transformation with rotation, translation, and scale.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NiTransform {
    pub rotate: NiMatrix3,
    pub translate: NiPoint3,
    pub scale: f32,
}

impl NiTransform {
    /// Creates a new identity transform.
    #[inline]
    pub const fn new() -> Self {
        Self { rotate: NiMatrix3::new(), translate: NiPoint3::zero(), scale: 1.0 }
    }

    /// Inverts the transform.
    pub fn invert(&self) -> Self {
        let inv_scale = 1.0 / self.scale;
        let inv_rotate = self.rotate.transpose();
        let inv_translate = (inv_rotate * -self.translate) * inv_scale;

        Self { rotate: inv_rotate, translate: inv_translate, scale: inv_scale }
    }

    /// Multiplies two transformations together.
    #[inline]
    pub fn mul_transform(&self, rhs: &Self) -> Self {
        Self {
            scale: self.scale * rhs.scale,
            rotate: self.rotate * rhs.rotate,
            translate: self.translate + (self.rotate * rhs.translate) * self.scale,
        }
    }

    /// Transforms a point by the transform.
    #[inline]
    pub fn mul_point(&self, point: &NiPoint3) -> NiPoint3 {
        ((self.rotate * *point) * self.scale) + self.translate
    }
}

impl Default for NiTransform {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl core::ops::Mul for NiTransform {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_transform(&rhs)
    }
}

impl core::ops::Mul<NiPoint3> for NiTransform {
    type Output = NiPoint3;

    #[inline]
    fn mul(self, rhs: NiPoint3) -> Self::Output {
        self.mul_point(&rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invert_transform() {
        let transform = NiTransform {
            rotate: NiMatrix3::new(),
            translate: NiPoint3::new(1.0, 2.0, 3.0),
            scale: 2.0,
        };

        let inverted = transform.invert();
        let expected = NiTransform {
            rotate: NiMatrix3 { entry: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] },
            translate: NiPoint3 { x: -0.5, y: -1.0, z: -1.5 },
            scale: 0.5,
        };
        assert_eq!(inverted, expected);
    }

    #[test]
    fn test_mul_transform() {
        let t1 = NiTransform {
            rotate: NiMatrix3::new(),
            translate: NiPoint3::new(1.0, 2.0, 3.0),
            scale: 2.0,
        };

        let t2 = NiTransform {
            rotate: NiMatrix3::new(),
            translate: NiPoint3::new(4.0, 5.0, 6.0),
            scale: 3.0,
        };

        let result = t1 * t2;
        let expected = NiTransform {
            rotate: NiMatrix3::new(),
            translate: NiPoint3::new(9.0, 12.0, 15.0),
            scale: 6.0,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_mul_point() {
        let transform = NiTransform {
            rotate: NiMatrix3::new(),
            translate: NiPoint3::new(1.0, 2.0, 3.0),
            scale: 2.0,
        };

        let point = NiPoint3::new(1.0, 1.0, 1.0);
        let result = transform * point;
        assert_eq!(result, NiPoint3::new(3.0, 4.0, 5.0));
    }
}

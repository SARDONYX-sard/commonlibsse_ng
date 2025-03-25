use crate::re::NiPoint3::NiPoint3;
use core::f32::consts::PI;

/// 3x3 matrix structure compatible with C++ layout.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NiMatrix3 {
    /// 3x3 matrix represented as an array of arrays
    pub entry: [[f32; 3]; 3],
}

impl NiMatrix3 {
    /// Constructs an identity matrix.
    #[inline]
    pub const fn new() -> Self {
        Self { entry: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] }
    }

    /// Constructs a matrix from three vectors.
    #[inline]
    pub const fn from_vectors(x: NiPoint3, y: NiPoint3, z: NiPoint3) -> Self {
        Self { entry: [[x.x, x.y, x.z], [y.x, y.y, y.z], [z.x, z.y, z.z]] }
    }

    /// Returns the X axis vector.
    #[inline]
    pub const fn get_vector_x(&self) -> NiPoint3 {
        NiPoint3::new(self.entry[0][0], self.entry[1][0], self.entry[2][0])
    }

    /// Returns the Y axis vector.
    #[inline]
    pub const fn get_vector_y(&self) -> NiPoint3 {
        NiPoint3::new(self.entry[0][1], self.entry[1][1], self.entry[2][1])
    }

    /// Returns the Z axis vector.
    #[inline]
    pub const fn get_vector_z(&self) -> NiPoint3 {
        NiPoint3::new(self.entry[0][2], self.entry[1][2], self.entry[2][2])
    }

    /// Transposes the matrix.
    pub fn transpose(&self) -> Self {
        let mut result = *self;
        for i in 0..3 {
            for j in 0..3 {
                result.entry[i][j] = self.entry[j][i];
            }
        }
        result
    }

    /// Multiplies the matrix by another matrix.
    pub fn mul_matrix(&self, rhs: &Self) -> Self {
        let mut result = Self::new();
        for i in 0..3 {
            for j in 0..3 {
                result.entry[i][j] = (0..3).map(|k| self.entry[i][k] * rhs.entry[k][j]).sum();
            }
        }
        result
    }

    /// Multiplies the matrix by a scalar.
    pub fn mul_scalar(&self, scalar: f32) -> Self {
        let mut result = *self;
        for row in &mut result.entry {
            for val in row.iter_mut() {
                *val *= scalar;
            }
        }
        result
    }

    /// Multiplies the matrix by a vector.
    pub fn mul_vector(&self, v: &NiPoint3) -> NiPoint3 {
        NiPoint3 {
            x: self.entry[0][2].mul_add(v.z, self.entry[0][0].mul_add(v.x, self.entry[0][1] * v.y)),
            y: self.entry[1][2].mul_add(v.z, self.entry[1][0].mul_add(v.x, self.entry[1][1] * v.y)),
            z: self.entry[2][2].mul_add(v.z, self.entry[2][0].mul_add(v.x, self.entry[2][1] * v.y)),
        }
    }

    /// Converts the matrix to Euler angles (XYZ) and returns the angles.
    pub fn to_euler_xyz(&self) -> Option<NiPoint3> {
        let y_angle = -self.entry[0][2].asin();
        if y_angle.abs() < PI / 2.0 {
            let x_angle = (-self.entry[1][2]).atan2(self.entry[2][2]);
            let z_angle = (-self.entry[0][1]).atan2(self.entry[0][0]);
            Some(NiPoint3::new(x_angle, y_angle, z_angle))
        } else {
            None
        }
    }

    /// Sets the matrix from Euler angles (XYZ).
    #[inline]
    pub fn set_euler_xyz(&mut self, x: f32, y: f32, z: f32) {
        *self = Self::from_euler_xyz(x, y, z);
    }

    /// Constructs a matrix from Euler angles.
    pub fn from_euler_xyz(x: f32, y: f32, z: f32) -> Self {
        let sin_x = x.sin();
        let cos_x = x.cos();
        let sin_y = y.sin();
        let cos_y = y.cos();
        let sin_z = z.sin();
        let cos_z = z.cos();

        Self {
            entry: [
                [cos_y * cos_z, cos_y * sin_z, -sin_y],
                [
                    (sin_x * sin_y).mul_add(sin_z, cos_x * cos_z),
                    (sin_x * sin_y).mul_add(sin_z, cos_x * cos_z),
                    sin_x * cos_y,
                ],
                [
                    (cos_x * sin_y).mul_add(cos_z, sin_x * sin_z),
                    (cos_x * sin_y).mul_add(sin_z, -(sin_x * cos_z)),
                    cos_x * cos_y,
                ],
            ],
        }
    }
}

impl Default for NiMatrix3 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl core::ops::Mul for NiMatrix3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_matrix(&rhs)
    }
}

impl core::ops::Mul<NiPoint3> for NiMatrix3 {
    type Output = NiPoint3;

    fn mul(self, rhs: NiPoint3) -> Self::Output {
        self.mul_vector(&rhs)
    }
}

impl core::ops::Mul<f32> for NiMatrix3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        self.mul_scalar(rhs)
    }
}

impl core::ops::Add for NiMatrix3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        for i in 0..3 {
            for j in 0..3 {
                result.entry[i][j] += rhs.entry[i][j];
            }
        }
        result
    }
}

impl core::ops::Sub for NiMatrix3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self;
        for i in 0..3 {
            for j in 0..3 {
                result.entry[i][j] -= rhs.entry[i][j];
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_operations() {
        let m1 = NiMatrix3::from_euler_xyz(0.1, 0.2, 0.3);
        let m2 = NiMatrix3::from_euler_xyz(0.3, 0.2, 0.1);
        let result = m1 * m2;

        let expected = NiMatrix3 {
            entry: [
                [1.146675, 0.42327216, -0.28814036],
                [1.8687906, 0.9814126, 0.17860672],
                [0.39051157, -0.28224778, 0.8589622],
            ],
        };
        assert_eq!(result, expected);
    }
}

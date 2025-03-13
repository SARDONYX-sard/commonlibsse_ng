use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NiPoint3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

const _: () = {
    assert!(core::mem::size_of::<NiPoint3>() == 0xc);
};

impl NiPoint3 {
    /// Creates a new `NiPoint3`.
    ///
    /// # Example
    /// ```
    /// let point = NiPoint3::new(1.0, 2.0, 3.0);
    /// assert_eq!(point.x, 1.0);
    /// ```
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Computes the dot product with another `NiPoint3`.
    ///
    /// # Example
    /// ```
    /// let a = NiPoint3::new(1.0, 2.0, 3.0);
    /// let b = NiPoint3::new(4.0, 5.0, 6.0);
    /// assert_eq!(a.dot(&b), 32.0);
    /// ```
    #[inline]
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Computes the cross product with another `NiPoint3`.
    ///
    /// # Example
    /// ```
    /// let a = NiPoint3::new(1.0, 0.0, 0.0);
    /// let b = NiPoint3::new(0.0, 1.0, 0.0);
    /// let cross = a.cross(&b);
    /// assert_eq!(cross, NiPoint3::new(0.0, 0.0, 1.0));
    /// ```
    #[inline]
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Computes the length (magnitude) of the vector.
    ///
    /// # Example
    /// ```
    /// let v = NiPoint3::new(3.0, 4.0, 0.0);
    /// assert_eq!(v.length(), 5.0);
    /// ```
    #[inline]
    pub fn length(&self) -> f32 {
        self.sqr_length().sqrt()
    }

    /// Returns the squared length of the vector.
    #[inline]
    pub fn sqr_length(&self) -> f32 {
        self.dot(self)
    }

    /// Computes the distance to another `NiPoint3`.
    ///
    /// # Example
    /// ```
    /// let a = NiPoint3::new(1.0, 2.0, 3.0);
    /// let b = NiPoint3::new(4.0, 6.0, 3.0);
    /// assert_eq!(a.distance(&b), 5.0);
    /// ```
    #[inline]
    pub fn distance(&self, other: &Self) -> f32 {
        (*self - *other).length()
    }

    /// Computes the squared distance to another `NiPoint3`.
    #[inline]
    pub fn squared_distance(&self, other: &Self) -> f32 {
        (*self - *other).sqr_length()
    }

    /// Normalizes the vector in place and returns its original length.
    ///
    /// # Example
    /// ```
    /// let mut v = NiPoint3::new(3.0, 4.0, 0.0);
    /// let len = v.unitize();
    /// assert_eq!(len, 5.0);
    /// assert_eq!(v, NiPoint3::new(0.6, 0.8, 0.0));
    /// ```
    #[inline]
    pub fn unitize(&mut self) -> f32 {
        let len = self.length();
        if len > 0.0 {
            *self /= len;
        }
        len
    }

    /// Computes the unit cross product with another `NiPoint3`.
    ///
    /// # Example
    /// ```
    /// let a = NiPoint3::new(1.0, 0.0, 0.0);
    /// let b = NiPoint3::new(0.0, 1.0, 0.0);
    /// let unit_cross = a.unit_cross(&b);
    /// assert_eq!(unit_cross, NiPoint3::new(0.0, 0.0, 1.0));
    /// ```
    #[inline]
    pub fn unit_cross(&self, other: &Self) -> Self {
        let mut cross = self.cross(other);
        cross.unitize();
        cross
    }

    /// Returns the zero vector.
    ///
    /// # Example
    /// ```
    /// let zero = NiPoint3::zero();
    /// assert_eq!(zero, NiPoint3::new(0.0, 0.0, 0.0));
    /// ```
    #[inline]
    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Index<usize> for NiPoint3 {
    type Output = f32;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            invalid_index => {
                panic!("NiPoint3 expects an index in the range 0..=2, but got {invalid_index}.")
            }
        }
    }
}

impl IndexMut<usize> for NiPoint3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            invalid_index => {
                panic!("NiPoint3 expects an index in the range 0..=2, but got {invalid_index}.")
            }
        }
    }
}

impl Add for NiPoint3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for NiPoint3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for NiPoint3 {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: f32) -> Self::Output {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Div<f32> for NiPoint3 {
    type Output = Self;

    #[inline]
    fn div(self, scalar: f32) -> Self::Output {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl Neg for NiPoint3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl AddAssign for NiPoint3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for NiPoint3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign<f32> for NiPoint3 {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl DivAssign<f32> for NiPoint3 {
    #[inline]
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl core::fmt::Display for NiPoint3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({:.1}, {:.1}, {:.1})", self.x, self.y, self.z)
    }
}

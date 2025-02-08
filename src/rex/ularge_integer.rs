/// A structure representing a 64-bit unsigned integer that can be accessed as high and low 32-bit parts,
/// similar to `ULARGE_INTEGER` in Windows API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ULargeInteger(u64);

impl ULargeInteger {
    /// Creates a new `ULargeInteger` from a [`u64`] value.
    ///
    /// # Example
    ///
    /// ```
    /// use commonlibsse_ng::rex::ularge_integer::ULargeInteger;
    ///
    /// let uli = ULargeInteger::new(0x1234_5678_9ABC_DEF0);
    /// assert_eq!(uli.to_u64(), 0x1234_5678_9ABC_DEF0);
    /// ```
    #[inline]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the higher 32 bits as a [`u32`].
    ///
    /// # Example
    ///
    /// ```
    /// use commonlibsse_ng::rex::ularge_integer::ULargeInteger;
    ///
    /// let uli = ULargeInteger::new(0x1234_5678_9ABC_DEF0);
    /// assert_eq!(uli.to_u32_high(), 0x1234_5678);
    ///
    /// let uli = ULargeInteger::new(0x9ABC_DEF0);
    /// assert_eq!(uli.to_u32_high(), 0x0);
    /// ```
    #[inline]
    pub const fn to_u32_high(&self) -> u32 {
        (self.0 >> 32) as u32
    }

    /// Returns the lower 32 bits as a [`u32`].
    ///
    /// # Example
    ///
    /// ```
    /// use commonlibsse_ng::rex::ularge_integer::ULargeInteger;
    ///
    /// let uli = ULargeInteger::new(0x1234_5678_9ABC_DEF0);
    /// assert_eq!(uli.to_u32_low(), 0x9ABC_DEF0);
    /// ```
    #[inline]
    pub const fn to_u32_low(&self) -> u32 {
        self.0 as u32
    }

    /// Returns a tuple `(high, low)`, where:
    /// - `high`: The upper 32 bits.
    /// - `low`: The lower 32 bits.
    ///
    /// # Example
    ///
    /// ```
    /// use commonlibsse_ng::rex::ularge_integer::ULargeInteger;
    ///
    /// let uli = ULargeInteger::new(0x1234_5678_9ABC_DEF0);
    /// assert_eq!(uli.split(), (0x1234_5678, 0x9ABC_DEF0));
    /// ```
    #[inline]
    pub const fn split(&self) -> (u32, u32) {
        (self.to_u32_high(), self.to_u32_low())
    }

    /// Returns the original 64-bit value.
    ///
    /// # Example
    ///
    /// ```
    /// use commonlibsse_ng::rex::ularge_integer::ULargeInteger;
    ///
    /// let uli = ULargeInteger::new(0x1234_5678_9ABC_DEF0);
    /// assert_eq!(uli.to_u64(), 0x1234_5678_9ABC_DEF0);
    /// ```
    #[inline]
    pub const fn to_u64(&self) -> u64 {
        self.0
    }
}

// impl  arithmetic operations for `ULargeInteger`.

impl core::ops::Add for ULargeInteger {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.0.wrapping_add(rhs.0))
    }
}

impl core::ops::Sub for ULargeInteger {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.0.wrapping_sub(rhs.0))
    }
}

impl core::ops::Mul for ULargeInteger {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.0.wrapping_mul(rhs.0))
    }
}

impl core::ops::Div for ULargeInteger {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self::new(self.0 / rhs.0)
    }
}

impl core::ops::Rem for ULargeInteger {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self::new(self.0 % rhs.0)
    }
}

// implements bitwise operations for `ULargeInteger`.

impl core::ops::BitAnd for ULargeInteger {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self::new(self.0 & rhs.0)
    }
}

impl core::ops::BitOr for ULargeInteger {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self::new(self.0 | rhs.0)
    }
}

impl core::ops::BitXor for ULargeInteger {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Self::new(self.0 ^ rhs.0)
    }
}

impl core::ops::Not for ULargeInteger {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        Self::new(!self.0)
    }
}

// implements shift operations for `ULargeInteger`.

impl core::ops::Shl<u32> for ULargeInteger {
    type Output = Self;

    #[inline]
    fn shl(self, rhs: u32) -> Self {
        Self::new(self.0.wrapping_shl(rhs))
    }
}

impl core::ops::Shr<u32> for ULargeInteger {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: u32) -> Self {
        Self::new(self.0.wrapping_shr(rhs))
    }
}

#[cfg(test)]
mod tests {
    use super::ULargeInteger;

    #[test]
    fn test_to_u32_high() {
        let uli = ULargeInteger::new(0x1234_5678_9ABC_DEF0);
        assert_eq!(uli.to_u32_high(), 0x1234_5678);
    }

    #[test]
    fn test_to_u32_low() {
        let uli = ULargeInteger::new(0x1234_5678_9ABC_DEF0);
        assert_eq!(uli.to_u32_low(), 0x9ABC_DEF0);
    }

    #[test]
    fn test_to_u32_tuple() {
        let uli = ULargeInteger::new(0x1234_5678_9ABC_DEF0);
        assert_eq!(uli.split(), (0x1234_5678, 0x9ABC_DEF0));
    }

    #[test]
    fn test_add() {
        let a = ULargeInteger::new(10);
        let b = ULargeInteger::new(20);
        assert_eq!((a + b).to_u64(), 30);
    }

    #[test]
    fn test_sub() {
        let a = ULargeInteger::new(50);
        let b = ULargeInteger::new(20);
        assert_eq!((a - b).to_u64(), 30);
    }

    #[test]
    fn test_mul() {
        let a = ULargeInteger::new(5);
        let b = ULargeInteger::new(6);
        assert_eq!((a * b).to_u64(), 30);
    }

    #[test]
    fn test_div() {
        let a = ULargeInteger::new(100);
        let b = ULargeInteger::new(10);
        assert_eq!((a / b).to_u64(), 10);
    }

    #[test]
    fn test_bitwise_operations() {
        let a = ULargeInteger::new(0b1100);
        let b = ULargeInteger::new(0b1010);
        assert_eq!((a & b).to_u64(), 0b1000);
        assert_eq!((a | b).to_u64(), 0b1110);
        assert_eq!((a ^ b).to_u64(), 0b0110);
        assert_eq!((!a).to_u64(), !0b1100);
    }

    #[test]
    fn test_shift_operations() {
        let a = ULargeInteger::new(0b0001);
        assert_eq!((a << 2).to_u64(), 0b0100);
        assert_eq!((a >> 1).to_u64(), 0b0000);
    }
}

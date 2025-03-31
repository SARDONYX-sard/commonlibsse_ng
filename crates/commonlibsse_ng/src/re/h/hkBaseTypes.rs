/// Alias for a 16-bit unsigned integer used as an object index in Havok.
pub type hkObjectIndex = u16;

/// Alias for a 32-bit float used as time in Havok.
pub type hkTime = f32;

/// Enumeration representing the result of a Havok operation.
#[commonlibsse_ng_derive_internal::ffi_enum]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum hkResult {
    Success = 0,
    Failure = 1,
}

/// Represents a 16-bit half-precision float with 7-bit precision in the Havok system.
///
/// This is a wrapper around a 16-bit integer that stores a compressed float value.
///
/// # Memory Layout:
/// - `_value`: 16-bit integer storing the half-precision float (0x00 - 0x01)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct hkHalf {
    /// The underlying 16-bit integer storing the half-precision float.
    /// - Offset: 0x00
    _value: i16,
}

// Compile-time memory layout verification
const _: () = {
    assert!(core::mem::offset_of!(hkHalf, _value) == 0x0);
    assert!(core::mem::size_of::<hkHalf>() == 0x2);
};

impl hkHalf {
    /// Creates a new `hkHalf` with a default value of 0.0.
    #[inline]
    pub fn new() -> Self {
        Self { _value: 0 }
    }

    /// Creates a new `hkHalf` from a 32-bit float.
    #[inline]
    pub fn from_f32(a_val: f32) -> Self {
        let mut half = Self::new();
        half.set_f32(a_val);
        half
    }

    /// Sets the internal half-precision value from a 32-bit float.
    #[inline]
    fn set_f32(&mut self, a_val: f32) {
        // Simplified conversion; actual half-precision float conversion would use IEEE 754 rules
        let bits = a_val.to_bits();
        let sign = (bits >> 31) as i16;
        let exp = ((bits >> 23) & 0xFF) as i16 - 127 + 15; // Adjust bias: 127 (f32) to 15 (f16)
        let mantissa = (bits & 0x7FFFFF) >> 13; // Take top 10 bits of 23-bit mantissa
        if exp <= 0 {
            self._value = 0; // Underflow to zero
        } else if exp > 31 {
            self._value = (sign << 15) | 0x7C00; // Infinity
        } else {
            self._value = (sign << 15) | (exp << 10) | mantissa as i16;
        }
    }

    /// Gets the 32-bit float value from this `hkHalf`.
    #[inline]
    fn to_f32(&self) -> f32 {
        // Simplified conversion; actual half-precision float conversion would use IEEE 754 rules
        let sign = (self._value >> 15) & 0x1;
        let exp = ((self._value >> 10) & 0x1F) as u32;
        let mantissa = (self._value & 0x3FF) as u32;
        if exp == 0 {
            if mantissa == 0 {
                0.0 // Zero
            } else {
                // Subnormal number
                f32::from_bits(((sign as u32) << 31) | (mantissa << 13))
                    * 2.0_f32.powi(-14 - 127 + 15)
            }
        } else if exp == 0x1F {
            if mantissa == 0 {
                if sign == 0 { f32::INFINITY } else { f32::NEG_INFINITY } // Infinity
            } else {
                f32::NAN // NaN
            }
        } else {
            f32::from_bits(((sign as u32) << 31) | ((exp + 127 - 15) << 23) | (mantissa << 13))
        }
    }
}

impl Default for hkHalf {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl From<f32> for hkHalf {
    #[inline]
    fn from(a_val: f32) -> Self {
        Self::from_f32(a_val)
    }
}

impl Into<f32> for hkHalf {
    #[inline]
    fn into(self) -> f32 {
        self.to_f32()
    }
}

/// Represents an 8-bit unsigned float in the Havok system.
///
/// This is an index into a lookup table for values ranging from 0.01 to 1,000,000.0.
///
/// # Memory Layout:
/// - `value`: 8-bit unsigned integer (0x00)
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct hkUFloat8 {
    /// The 8-bit index into a lookup table.
    /// - Offset: 0x00
    pub value: u8,
}

// Compile-time memory layout verification
const _: () = {
    assert!(core::mem::offset_of!(hkUFloat8, value) == 0x0);
    assert!(core::mem::size_of::<hkUFloat8>() == 0x1);
};

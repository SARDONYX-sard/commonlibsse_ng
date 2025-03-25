use crate::re::NiColor::NiColor;
use core::fmt;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

// NOTE: Color needs `Copy` to simplify the four arithmetic operations.

/// Represents an RGBA color with 8-bit channels.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
const _: () = assert!(core::mem::size_of::<Color>() == 0x4);

impl Color {
    /// Creates a new `Color` with the specified RGBA values.
    #[inline]
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self { red, green, blue, alpha }
    }

    /// Creates a `Color` from a 32-bit hex value (0xRRGGBB).
    #[inline]
    pub const fn from_hex(hex: u32) -> Self {
        Self {
            red: ((hex >> 16) & 0xFF) as u8,
            green: ((hex >> 8) & 0xFF) as u8,
            blue: (hex & 0xFF) as u8,
            alpha: 0,
        }
    }

    /// Converts the color to a 32-bit integer (0xRRGGBBAA).
    #[inline]
    pub const fn to_u32(&self) -> u32 {
        ((self.red as u32) << 24)
            | ((self.green as u32) << 16)
            | ((self.blue as u32) << 8)
            | (self.alpha as u32)
    }

    /// Converts the color to a hex string in the format `#RRGGBBAA`.
    pub fn to_hex_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", self.red, self.green, self.blue, self.alpha)
    }

    /// Returns the color components as a slice.
    #[inline]
    pub const fn as_slice(&self) -> [u8; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }
}

impl fmt::Display for Color {
    // Formats the color as `#RRGGBBAA` in hexadecimal.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}{:02X}", self.red, self.green, self.blue, self.alpha)
    }
}

// --- Operator Overloads ---

impl Add for Color {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red.saturating_add(rhs.red),
            green: self.green.saturating_add(rhs.green),
            blue: self.blue.saturating_add(rhs.blue),
            alpha: self.alpha.saturating_add(rhs.alpha),
        }
    }
}

impl AddAssign for Color {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.red = self.red.saturating_add(rhs.red);
        self.green = self.green.saturating_add(rhs.green);
        self.blue = self.blue.saturating_add(rhs.blue);
        self.alpha = self.alpha.saturating_add(rhs.alpha);
    }
}

impl Sub for Color {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red.saturating_sub(rhs.red),
            green: self.green.saturating_sub(rhs.green),
            blue: self.blue.saturating_sub(rhs.blue),
            alpha: self.alpha.saturating_sub(rhs.alpha),
        }
    }
}

impl SubAssign for Color {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.red = self.red.saturating_sub(rhs.red);
        self.green = self.green.saturating_sub(rhs.green);
        self.blue = self.blue.saturating_sub(rhs.blue);
        self.alpha = self.alpha.saturating_sub(rhs.alpha);
    }
}

impl Mul for Color {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red.saturating_mul(rhs.red),
            green: self.green.saturating_mul(rhs.green),
            blue: self.blue.saturating_mul(rhs.blue),
            alpha: self.alpha.saturating_mul(rhs.alpha),
        }
    }
}

impl MulAssign for Color {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.red = self.red.saturating_mul(rhs.red);
        self.green = self.green.saturating_mul(rhs.green);
        self.blue = self.blue.saturating_mul(rhs.blue);
        self.alpha = self.alpha.saturating_mul(rhs.alpha);
    }
}

impl Div for Color {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red.saturating_div(rhs.red.max(1)), // Avoid division by zero
            green: self.green.saturating_div(rhs.green.max(1)),
            blue: self.blue.saturating_div(rhs.blue.max(1)),
            alpha: self.alpha.saturating_div(rhs.alpha.max(1)),
        }
    }
}

impl DivAssign for Color {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.red = self.red.saturating_div(rhs.red.max(1));
        self.green = self.green.saturating_div(rhs.green.max(1));
        self.blue = self.blue.saturating_div(rhs.blue.max(1));
        self.alpha = self.alpha.saturating_div(rhs.alpha.max(1));
    }
}

// --- Scalar Operations ---

impl Mul<u8> for Color {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: u8) -> Self::Output {
        Self {
            red: self.red.saturating_mul(scalar),
            green: self.green.saturating_mul(scalar),
            blue: self.blue.saturating_mul(scalar),
            alpha: self.alpha.saturating_mul(scalar),
        }
    }
}

impl Div<u8> for Color {
    type Output = Self;

    #[inline]
    fn div(self, scalar: u8) -> Self::Output {
        Self {
            red: self.red.saturating_div(scalar.max(1)),
            green: self.green.saturating_div(scalar.max(1)),
            blue: self.blue.saturating_div(scalar.max(1)),
            alpha: self.alpha.saturating_div(scalar.max(1)),
        }
    }
}

impl From<NiColor> for Color {
    #[inline]
    fn from(value: NiColor) -> Self {
        Self { red: value.red as u8, green: value.green as u8, blue: value.blue as u8, alpha: 0 }
    }
}

#[test]
fn test_color_operations() {
    let color1 = Color::new(100, 150, 200, 255);
    let color2 = Color::new(50, 50, 50, 128);

    assert_eq!(color1 + color2, Color::new(150, 200, 250, 255));
    assert_eq!(color1 - color2, Color::new(50, 100, 150, 127));
    assert_eq!(color1 * color2, Color::new(255, 255, 255, 255));
    assert_eq!(color1 / color2, Color::new(2, 3, 4, 1));

    // Scalar
    assert_eq!(color1 * 2, Color::new(200, 255, 255, 255));
    assert_eq!(color1 / 2, Color::new(50, 75, 100, 127));
}

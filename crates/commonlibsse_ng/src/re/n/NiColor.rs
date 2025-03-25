//! # NiColor and NiColorA Module
//!
//! This module provides two color structs, `NiColor` and `NiColorA`, representing RGB and RGBA colors respectively.
//! It includes support for arithmetic operations, hex and integer conversion, and display formatting.
//!

use core::fmt;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Represents an RGB color with `f32` components for red, green, and blue.
///
/// # Example
/// ```
/// # use commonlibsse_ng::re::NiColor::NiColor;
/// let color = NiColor::new(0.5, 0.3, 0.8);
/// ```
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NiColor {
    /// Red component (range: 0.0..=1.0)
    pub red: f32,
    /// Green component (range: 0.0..=1.0)
    pub green: f32,
    /// Blue component (range: 0.0..=1.0)
    pub blue: f32,
}

impl NiColor {
    /// Creates a new `NiColor` with the specified RGB values.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::NiColor::NiColor;
    /// let color = NiColor::new(0.5, 0.3, 0.8);
    /// ```
    #[inline]
    pub const fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }

    /// Creates a `NiColor` from a 32-bit hex value in the format `0xRRGGBB`.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::NiColor::NiColor;
    /// let color = NiColor::from_hex(0xFF5733);
    /// ```
    #[inline]
    pub const fn from_hex(hex: u32) -> Self {
        Self {
            red: ((hex >> 16) & 0xFF) as f32 / 255.0,
            green: ((hex >> 8) & 0xFF) as f32 / 255.0,
            blue: (hex & 0xFF) as f32 / 255.0,
        }
    }

    /// Converts the color to a 32-bit integer in the format `0xRRGGBB`.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::NiColor::NiColor;
    /// let color = NiColor::new(1.0, 0.5, 0.0);
    /// assert_eq!(color.to_u32(), 0xFF7F00);
    /// ```
    #[inline]
    pub fn to_u32(&self) -> u32 {
        (((self.red * 255.0) as u32) << 16)
            | (((self.green * 255.0) as u32) << 8)
            | (self.blue * 255.0) as u32
    }

    /// Converts the color to a hex string in the format `#RRGGBB`.
    ///
    /// # Example
    /// ```
    /// # use commonlibsse_ng::re::NiColor::NiColor;
    /// let color = NiColor::new(1.0, 0.5, 0.0);
    /// assert_eq!(color.to_hex(), "#FF7F00");
    /// ```
    #[inline]
    pub fn to_hex(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}",
            (self.red * 255.0) as u32,
            (self.green * 255.0) as u32,
            (self.blue * 255.0) as u32
        )
    }
}

// Arithmetic operators for `NiColor`
impl Add for NiColor {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self { red: self.red + rhs.red, green: self.green + rhs.green, blue: self.blue + rhs.blue }
    }
}

impl AddAssign for NiColor {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
    }
}

impl Sub for NiColor {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self { red: self.red - rhs.red, green: self.green - rhs.green, blue: self.blue - rhs.blue }
    }
}

impl SubAssign for NiColor {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.red -= rhs.red;
        self.green -= rhs.green;
        self.blue -= rhs.blue;
    }
}

impl Mul<f32> for NiColor {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self { red: self.red * rhs, green: self.green * rhs, blue: self.blue * rhs }
    }
}

impl MulAssign<f32> for NiColor {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.red *= rhs;
        self.green *= rhs;
        self.blue *= rhs;
    }
}

impl Div<f32> for NiColor {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self { red: self.red / rhs, green: self.green / rhs, blue: self.blue / rhs }
    }
}

impl DivAssign<f32> for NiColor {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.red /= rhs;
        self.green /= rhs;
        self.blue /= rhs;
    }
}

// Display formatting for `NiColor`
impl fmt::Display for NiColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.1}, {:.1}, {:.1})", self.red, self.green, self.blue)
    }
}

/// Represents an RGBA color with `f32` components for red, green, blue, and alpha.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NiColorA {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl NiColorA {
    /// Creates a new `NiColorA` with the specified RGBA values.
    #[inline]
    pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self { red, green, blue, alpha }
    }

    /// Converts the color to a 32-bit integer in the format `0xRRGGBBAA`.
    #[inline]
    pub fn to_u32(&self) -> u32 {
        (((self.red * 255.0) as u32) << 24)
            | (((self.green * 255.0) as u32) << 16)
            | (((self.blue * 255.0) as u32) << 8)
            | (self.alpha * 255.0) as u32
    }

    /// Converts the color to a hex string in the format `#RRGGBBAA`.
    #[inline]
    pub fn to_hex(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            (self.red * 255.0) as u32,
            (self.green * 255.0) as u32,
            (self.blue * 255.0) as u32,
            (self.alpha * 255.0) as u32
        )
    }
}

// Arithmetic operators for `NiColorA`
impl Add for NiColorA {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
            alpha: self.alpha + rhs.alpha,
        }
    }
}

impl Mul<f32> for NiColorA {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
            alpha: self.alpha * rhs,
        }
    }
}

impl fmt::Display for NiColorA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.1}, {:.1}, {:.1}, {:.1})", self.red, self.green, self.blue, self.alpha)
    }
}

impl From<NiColor> for NiColorA {
    #[inline]
    fn from(color: NiColor) -> Self {
        Self { red: color.red, green: color.green, blue: color.blue, alpha: 0.0 }
    }
}

impl From<NiColorA> for NiColor {
    #[inline]
    fn from(color: NiColorA) -> Self {
        Self { red: color.red, green: color.green, blue: color.blue }
    }
}

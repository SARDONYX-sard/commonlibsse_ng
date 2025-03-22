//! NiMath module with math utility functions equivalent to NiMath.h and NiMath.cpp in C++.
//!
//! Provides functions for angle conversions, trigonometry, and fast arctangent calculation.

/// Infinity constant, equivalent to `FLT_MAX`.
pub const NI_INFINITY: f32 = f32::MAX;

/// π
pub const NI_PI: f32 = core::f32::consts::PI;

/// π/2
pub const NI_HALF_PI: f32 = core::f32::consts::FRAC_PI_2;

/// 2π
pub const NI_TWO_PI: f32 = 2.0 * NI_PI;

/// Converts degrees to radians.
///
/// # Examples
/// ```
/// # use commonlibsse_ng::re::NiMath::deg_to_rad;
/// assert_eq!(deg_to_rad(180.0), core::f32::consts::PI);
/// ```
#[inline]
pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * (NI_PI / 180.0)
}

/// Converts radians to degrees.
///
/// # Examples
/// ```
/// # use commonlibsse_ng::re::NiMath::rad_to_deg;
/// assert_eq!(rad_to_deg(core::f32::consts::PI), 180.0);
/// assert_eq!(rad_to_deg(core::f32::consts::PI * 2.0), 360.0);
/// ```
#[inline]
pub fn rad_to_deg(radians: f32) -> f32 {
    radians * (180.0 / NI_PI)
}

/// Normalizes an angle in radians to the range `[-π, π]`.
///
/// # Examples
/// ```
/// # use commonlibsse_ng::re::NiMath::normalize_angle;
/// // It is usually a mistake to rely on the equivalence of any float,
/// // but we use `assert_eq` in our tests because the same output is expected for the same input.
/// assert_eq!(normalize_angle(4.0), -2.283_185_2);
/// assert_eq!(normalize_angle(-5.0), 1.283_185_5);
/// assert_eq!(normalize_angle(7.0), 0.716_814_76);
/// ```
#[inline]
pub fn normalize_angle(radians: f32) -> f32 {
    // mod(%) 2π: rounds the angle from 0..=2π
    match (radians + NI_PI) % NI_TWO_PI {
        n if n >= 0.0 => n - NI_PI,
        n => n + NI_PI,
    }
}

/// Returns the absolute value of a float.
///
/// # Examples
/// ```
/// # use commonlibsse_ng::re::NiMath::ni_abs;
/// const _: () = assert!(ni_abs(-3.5) == 3.5);
/// const _: () = assert!(ni_abs(2.0) == 2.0);
/// ```
#[inline]
pub const fn ni_abs(value: f32) -> f32 {
    value.abs()
}

/// Computes the arcsine of a value with clamping to `[-1, 1]`.
///
/// - Special cases:
///     - `value >= 1.0` -> returns `π/2`
///     - `value <= -1.0` -> returns `-π/2`
///
/// # Unspecified precision
///
/// Note that the precision for -1.0..1.0 depends on the Rust version and OS, due to the use of `f32::asin`.
///
/// # Examples
///
/// ```
/// # use commonlibsse_ng::re::NiMath::ni_asin;
/// assert_eq!(ni_asin(core::f32::consts::FRAC_PI_2.sin()), core::f32::consts::FRAC_PI_2);
/// // overflow -> `π/2`
/// assert_eq!(ni_asin(1.0), core::f32::consts::PI * 0.5);
/// // underflow -> `-π/2`
/// assert_eq!(ni_asin(-1.1), -core::f32::consts::PI * 0.5);
/// ```
#[inline]
pub fn ni_asin(value: f32) -> f32 {
    match value {
        v if (-1.0..1.0).contains(&v) => v.asin(),
        v if v >= 1.0 => NI_HALF_PI,
        _ => -NI_HALF_PI,
    }
}

/// Approximates the arctangent of `y/x` using a fast polynomial expansion.
///
/// - Special cases:
///     - `atan2(0, 0)` → `0.0`
///     - `atan2(1, 0)` → `π/2`
///     - `atan2(0, -1)` → `π`
///
/// # Examples
///
/// ```
/// # use commonlibsse_ng::re::NiMath::ni_fast_atan2;
/// // `atan2(1, 0)` -> `π/2`
/// assert!((ni_fast_atan2(1.0, 0.0) - core::f32::consts::FRAC_PI_2).abs() < 1e-6);
/// // `atan2(0, -1)` -> `π`
/// assert!((ni_fast_atan2(0.0, -1.0) - core::f32::consts::PI).abs() < 1e-6);
/// ```
///
/// # Explanation:
/// This function uses a polynomial approximation of `atan(z)`:
///
/// ```txt
/// atan(z) ≈ z × (0.9998660 + z² × (-0.3302995 + z² × (0.1801410 +
///            z² × (-0.0851330 + z² × 0.0208351))))
/// ```
///
/// The nested form ([`Horner's method`]) is used for **faster computation**:
/// - Reduces the number of multiplication operations.
/// - Avoids explicit power calculations (e.g., `z^3`, `z^5`) by reusing `z²`.
///
/// [`Horner's method`]: https://en.wikipedia.org/wiki/Horner%27s_method
#[inline]
pub fn ni_fast_atan2(y: f32, x: f32) -> f32 {
    if x == 0.0 && y == 0.0 {
        return 0.0;
    }

    let mut offset = 0.0;
    let z;

    if ni_abs(y) > ni_abs(x) {
        z = x / y;
        if z > 0.0 {
            offset = NI_HALF_PI;
        } else if z < 0.0 {
            offset = -NI_HALF_PI;
        } else {
            return if y > 0.0 { NI_HALF_PI } else { -NI_HALF_PI };
        }
    } else {
        z = y / x;
        if z == 0.0 {
            return if x > 0.0 { 0.0 } else { NI_PI };
        }
    }

    let z2 = z * z;

    // 5th-degree polynomial expansion for atan(z)
    let mut result = 0.0208351; // z^5 coefficient
    result *= z2; // z^2
    result -= 0.0851330; // z^4 coefficient
    result *= z2; // z^4
    result += 0.180141; // z^3 coefficient
    result *= z2; // z^6
    result -= 0.3302995; // z^2 coefficient
    result *= z2; // z^8
    result += 0.999866; // z^1 coefficient
    result *= z; // Multiply by z

    result = offset - result;

    // Adjust the result for quadrant corrections
    if y < 0.0 && x < 0.0 {
        result -= NI_PI; // 3rd quadrant correction
    }
    if y > 0.0 && x < 0.0 {
        result += NI_PI; // 2nd quadrant correction
    }

    result
}

/// Options for configuring the floating-point comparison behavior.
#[derive(Debug, Clone, Copy)]
pub struct ComparisonOptions {
    /// The relative tolerance used for comparison (epsilon).
    /// This is typically a small value like `FLT_EPSILON`.
    pub epsilon: f32,

    /// The absolute tolerance used for comparison.
    /// This is typically used for very small numbers.
    pub abs_th: f32,
}

impl ComparisonOptions {
    /// Creates a new `ComparisonOptions` instance with default values.
    ///
    /// The default values are `epsilon = 128 * FLT_EPSILON` and `abs_th = FLT_MIN`.
    #[inline]
    pub const fn new(epsilon: f32, abs_th: f32) -> Self {
        Self { epsilon, abs_th }
    }

    /// Returns default comparison options with typical values:
    /// - `epsilon`: 128 * FLT_EPSILON
    /// - `abs_th`: FLT_MIN
    #[inline]
    pub const fn const_default() -> Self {
        Self { epsilon: 128.0 * f32::EPSILON, abs_th: f32::MIN }
    }
}

impl Default for ComparisonOptions {
    #[inline]
    fn default() -> Self {
        Self { epsilon: 128.0 * f32::EPSILON, abs_th: f32::MIN }
    }
}

/// Compares two floating point numbers `a` and `b` to see if they are nearly equal.
///
/// This function uses both relative and absolute comparisons to handle cases where the values
/// differ by a small but significant amount due to floating-point precision limitations.
///
/// - origin: https://stackoverflow.com/questions/4915462/how-should-i-do-floating-point-comparison
///
/// # Panics
/// Ensure that `epsilon` is within a reasonable range:
/// - It should not be smaller than the smallest representable float (`FLT_EPSILON`)
/// - It should not be equal to or greater than 1.0 to avoid overly lenient comparisons
#[inline]
pub fn nearly_equal(a: f32, b: f32, options: ComparisonOptions) -> bool {
    assert!(f32::EPSILON <= options.epsilon, "epsilon must be >= FLT_EPSILON");
    assert!(options.epsilon < 1.0, "epsilon must be < 1.0");

    #[allow(clippy::float_cmp)]
    if a == b {
        return true; // No need to do further comparison if they are exactly equal
    }

    let diff = (a - b).abs();
    let norm = (a.abs() + b.abs()).min(f32::MAX);
    diff < f32::max(options.abs_th, options.epsilon * norm)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_nearly_eq;

    #[test]
    fn test_deg_to_rad() {
        assert_nearly_eq!(deg_to_rad(180.0), core::f32::consts::PI, epsilon = 1e-6);
        assert_nearly_eq!(deg_to_rad(90.0), core::f32::consts::FRAC_PI_2, epsilon = 1e-6);
    }

    #[test]
    fn test_rad_to_deg() {
        assert_nearly_eq!(rad_to_deg(core::f32::consts::PI), 180.0, epsilon = 1e-6);
        assert_nearly_eq!(rad_to_deg(core::f32::consts::FRAC_PI_2), 90.0, epsilon = 1e-6);
    }

    #[test]
    fn test_normalize_angle() {
        assert_nearly_eq!(normalize_angle(4.0), -2.283_185_2, epsilon = 1e-6);
        assert_nearly_eq!(normalize_angle(-5.0), 1.283_185_5, epsilon = 1e-6);
        assert_nearly_eq!(normalize_angle(7.0), 0.716_814_76, epsilon = 1e-6);
    }

    #[test]
    fn test_ni_abs() {
        assert_nearly_eq!(ni_abs(-5.0), 5.0, epsilon = 1e-6);
        assert_nearly_eq!(ni_abs(5.0), 5.0, epsilon = 1e-6);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_ni_asin() {
        assert_eq!(ni_asin(1.0), core::f32::consts::FRAC_PI_2);
        assert_eq!(ni_asin(-1.0), -core::f32::consts::FRAC_PI_2);
        assert_eq!(ni_asin(0.0), 0.0);
    }

    #[test]
    fn test_ni_fast_atan2() {
        assert_nearly_eq!(ni_fast_atan2(1.0, 0.0), core::f32::consts::FRAC_PI_2);
        assert_nearly_eq!(ni_fast_atan2(0.0, -1.0), core::f32::consts::PI);
    }
}

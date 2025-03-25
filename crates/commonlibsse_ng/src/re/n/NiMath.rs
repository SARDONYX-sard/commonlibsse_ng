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
pub const NI_TWO_PI: f32 = core::f32::consts::TAU;
#[allow(clippy::float_cmp_const)]
const _: () = debug_assert!(NI_TWO_PI == 2.0 * NI_PI);

/// Converts degrees to radians.
///
/// # Examples
/// ```
/// # use commonlibsse_ng::re::NiMath::deg_to_rad;
/// assert_eq!(deg_to_rad(180.0), core::f32::consts::PI);
/// ```
#[inline]
pub const fn deg_to_rad(degrees: f32) -> f32 {
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
pub const fn rad_to_deg(radians: f32) -> f32 {
    radians * (180.0 / NI_PI)
}

/// Normalizes an angle in radians to the range `[-π, π]`.
///
/// - Test with C++: [See Compiler Explorer](https://godbolt.org/z/j56qT1KP5)
///
/// # Examples
/// ```
/// // Note: 450° causes a rounding error, but 540° does not, so there is no problem using `assert`.
/// # use commonlibsse_ng::re::NiMath::{normalize_angle, rad_to_deg};
/// use core::f32::consts::{TAU, PI};
/// const _: () = assert!(TAU == 2.0 * PI);
///
/// const RAD_OF_540DEG: f32 = TAU + PI;
/// assert!(rad_to_deg(RAD_OF_540DEG) == 540.0);
/// assert!(normalize_angle(RAD_OF_540DEG) == -PI);
/// ```
#[inline]
pub const fn normalize_angle(radians: f32) -> f32 {
    use core::f32::consts::{PI, TAU};

    // Expand `(radians + PI).rem_euclid(TAU) - PI` for compile time evaluation.
    let r = (radians + PI) % TAU;
    (if r < 0.0 { r + TAU.abs() } else { r }) - PI
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

/// Computes the arcsine(tilt to angle) of a value with clamping to `[-1, 1]`.
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
/// use core::f32::consts::FRAC_PI_2;
/// assert_eq!(ni_asin(FRAC_PI_2.sin()), FRAC_PI_2);
/// assert_eq!(ni_asin(1.0), FRAC_PI_2);   // overflow -> `π/2`
/// assert_eq!(ni_asin(-1.1), -FRAC_PI_2); // underflow -> `-π/2`
/// ```
#[inline]
pub fn ni_asin(tilt: f32) -> f32 {
    match tilt {
        v if (-1.0..1.0).contains(&v) => v.asin(),
        v if v >= 1.0 => NI_HALF_PI,
        _ => -NI_HALF_PI,
    }
}

/// Approximates the arctangent of `y/x` using a fast polynomial expansion.
///
/// - Special cases:
///     - `atan2(0, 0)` → `0.0` rad
///     - `atan2(1, 0)` → `π/2` rad
///     - `atan2(0, -1)` → `π` rad
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
pub const fn ni_fast_atan2(y: f32, x: f32) -> f32 {
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
pub const fn nearly_equal(a: f32, b: f32, options: ComparisonOptions) -> bool {
    assert!(f32::EPSILON <= options.epsilon, "epsilon must be >= FLT_EPSILON");
    assert!(options.epsilon < 1.0, "epsilon must be < 1.0");

    #[allow(clippy::float_cmp)]
    if a == b {
        return true; // No need to do further comparison if they are exactly equal
    }

    let diff = (a - b).abs();
    let norm = (a + b).abs().min(f32::MAX);
    diff < f32::max(options.abs_th, options.epsilon * norm)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]
    #![allow(clippy::float_cmp_const)]
    #![allow(clippy::missing_const_for_fn)]
    use super::*;
    use crate::assert_nearly_eq;
    use core::f32::consts::{FRAC_PI_2, PI, TAU};

    #[test]
    const fn test_deg_to_rad() {
        assert!(deg_to_rad(180.0) == PI);
        assert!(deg_to_rad(90.0) == FRAC_PI_2);
    }

    #[test]
    const fn test_rad_to_deg() {
        assert!(rad_to_deg(PI) == 180.0);
        assert!(rad_to_deg(FRAC_PI_2) == 90.0);
    }

    #[test]
    fn test_normalize_angle() {
        // 450 - 360 = 90 degrees
        const _: () = {
            const RAD_OF_90DEG: f32 = FRAC_PI_2;
            assert!(rad_to_deg(RAD_OF_90DEG) == 90.0);

            const RAD_OF_450DEG: f32 = (5.0 * PI) / 2.0;
            assert!(rad_to_deg(RAD_OF_450DEG) == 450.0);

            const EPSILON: f32 = 4.0 * 1e-7;
            assert!(EPSILON == 0.0000004);

            assert!(normalize_angle(RAD_OF_450DEG) == RAD_OF_90DEG + EPSILON);
        };

        const _: () = {
            const RAD_OF_540DEG: f32 = TAU + PI;
            assert!(TAU == 2.0 * PI);
            assert!(rad_to_deg(RAD_OF_540DEG) == 540.0);
            assert!(normalize_angle(RAD_OF_540DEG) == -PI);
        };
    }

    #[test]
    fn test_ni_abs() {
        assert!(ni_abs(-5.0) == 5.0);
        assert!(ni_abs(5.0) == 5.0);
    }

    #[test]
    fn test_ni_asin() {
        assert_eq!(ni_asin(1.0), FRAC_PI_2);
        assert_eq!(ni_asin(-1.0), -FRAC_PI_2);
        assert_eq!(ni_asin(0.0), 0.0);
    }

    #[test]
    fn test_ni_fast_atan2() {
        assert_nearly_eq!(ni_fast_atan2(1.0, 0.0), FRAC_PI_2);
        assert_nearly_eq!(ni_fast_atan2(0.0, -1.0), PI);
    }
}

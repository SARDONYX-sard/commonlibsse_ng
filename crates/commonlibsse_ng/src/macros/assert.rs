// SPDX-License-Identifier: Apache-2.0 OR MIT
// Provided by cxx crate
// https://github.com/dtolnay/cxx/blob/master/src/macros/assert.rs

/// Compile time assertion.
#[macro_export]
#[doc(hidden)]
macro_rules! const_assert_eq {
    ($left:expr, $right:expr $(,)?) => {
        const _: [(); $left] = [(); $right];
    };
}

/// Custom macro to assert that two floating-point values are nearly equal.
/// - origin: https://stackoverflow.com/questions/4915462/how-should-i-do-floating-point-comparison
///
/// # Examples
///
/// ```
/// commonlibsse_ng::assert_nearly_eq!(4.9303807e-32, 4.930381e-32);
/// ```
///
/// - panic code
/// ```should_panic
/// commonlibsse_ng::assert_nearly_eq!(4.930381e-32, 4.9309825e-32, epsilon = 1e-4);
/// ```
#[macro_export]
macro_rules! assert_nearly_eq {
    // This variant takes both comparison values and optional named parameters (epsilon and abs_th)
    ($left:expr, $right:expr, $(epsilon = $epsilon:expr),* $(, abs_th = $abs_th:expr)?) => {{
        #[allow(unused_mut)]
        let mut epsilon = 128.0 * f32::EPSILON;
        #[allow(unused_mut)]
        let mut abs_th = f32::MIN;

        $(
            if stringify!($epsilon) == "epsilon" {
                epsilon = $epsilon;
            }
        )*
        $(
            #[allow(clippy::multi_assignments)]
            if stringify!($abs_th) == "abs_th" {
                abs_th = $abs_th;
            }
        )*

        if !$crate::re::NiMath::nearly_equal($left, $right, $crate::re::NiMath::ComparisonOptions::new(epsilon, abs_th)) {
            panic!(
                "Assertion failed: (left: {}, right: {}) are not nearly equal with epsilon: {}, abs_th: {}",
                $left, $right, epsilon, abs_th
            );
        }
    }};
    ($left:expr, $right:expr) => {
        $crate::assert_nearly_eq!($left, $right, epsilon = 128.0 * f32::EPSILON, abs_th = f32::MIN);
    };
}

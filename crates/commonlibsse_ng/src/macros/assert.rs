// SPDX-License-Identifier: Apache-2.0 OR MIT
// Provided by cxx crate
// https://github.com/dtolnay/cxx/blob/master/src/macros/assert.rs

#[macro_export]
#[doc(hidden)]
macro_rules! const_assert_eq {
    ($left:expr, $right:expr $(,)?) => {
        const _: [(); $left] = [(); $right];
    };
}

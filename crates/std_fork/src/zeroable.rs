// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2019 Daniel "Lokathor" Gee
//
// This file is derived from `bytemuck` version 1.22.0,
// available at https://crates.io/crates/bytemuck
//
// - https://docs.rs/bytemuck/1.22.0/src/bytemuck/zeroable.rs.html#25-35
//
// Copyright (c) 2019 Daniel "Lokathor" Gee
// Licensed under either of Apache License, Version 2.0 or MIT license at your option.
//
// See the original license texts at:
// - http://www.apache.org/licenses/LICENSE-2.0
// - https://opensource.org/licenses/MIT

/// Marker Trait for types that can be safely created with
/// [`zeroed`](core::mem::zeroed).
///
/// An all-zeroes value may or may not be the same value as the
/// [Default](core::default::Default) value of the type.
///
/// ## Safety
///
/// * Your type must be inhabited (eg: no
///   [Infallible](core::convert::Infallible)).
/// * Your type must be allowed to be an "all zeroes" bit pattern (eg: no
///   [`NonNull<T>`](core::ptr::NonNull)).
///
pub unsafe trait Zeroable: Sized {}
unsafe impl Zeroable for () {}
unsafe impl Zeroable for bool {}
unsafe impl Zeroable for char {}
unsafe impl Zeroable for u8 {}
unsafe impl Zeroable for i8 {}
unsafe impl Zeroable for u16 {}
unsafe impl Zeroable for i16 {}
unsafe impl Zeroable for u32 {}
unsafe impl Zeroable for i32 {}
unsafe impl Zeroable for u64 {}
unsafe impl Zeroable for i64 {}
unsafe impl Zeroable for usize {}
unsafe impl Zeroable for isize {}
unsafe impl Zeroable for u128 {}
unsafe impl Zeroable for i128 {}
unsafe impl Zeroable for f32 {}
unsafe impl Zeroable for f64 {}
unsafe impl<T: Zeroable> Zeroable for core::num::Wrapping<T> {}
unsafe impl<T: Zeroable> Zeroable for core::cmp::Reverse<T> {}
unsafe impl<T: Zeroable> Zeroable for core::num::Saturating<T> {}

// Note: we can't implement this for all `T: ?Sized` types because it would
// create NULL pointers for vtables.
// Maybe one day this could be changed to be implemented for
// `T: ?Sized where <T as core::ptr::Pointee>::Metadata: Zeroable`.
unsafe impl<T> Zeroable for *mut T {}
unsafe impl<T> Zeroable for *const T {}
unsafe impl<T> Zeroable for *mut [T] {}
unsafe impl<T> Zeroable for *const [T] {}
unsafe impl Zeroable for *mut str {}
unsafe impl Zeroable for *const str {}

unsafe impl<T: ?Sized> Zeroable for core::marker::PhantomData<T> {}
unsafe impl Zeroable for core::marker::PhantomPinned {}
unsafe impl<T: Zeroable> Zeroable for core::mem::ManuallyDrop<T> {}
unsafe impl<T: Zeroable> Zeroable for core::cell::UnsafeCell<T> {}
unsafe impl<T: Zeroable> Zeroable for core::cell::Cell<T> {}

unsafe impl<A: Zeroable> Zeroable for (A,) {}
unsafe impl<A: Zeroable, B: Zeroable> Zeroable for (A, B) {}
unsafe impl<A: Zeroable, B: Zeroable, C: Zeroable> Zeroable for (A, B, C) {}
unsafe impl<A: Zeroable, B: Zeroable, C: Zeroable, D: Zeroable> Zeroable for (A, B, C, D) {}
unsafe impl<A: Zeroable, B: Zeroable, C: Zeroable, D: Zeroable, E: Zeroable> Zeroable
    for (A, B, C, D, E)
{
}
unsafe impl<A: Zeroable, B: Zeroable, C: Zeroable, D: Zeroable, E: Zeroable, F: Zeroable> Zeroable
    for (A, B, C, D, E, F)
{
}
unsafe impl<
    A: Zeroable,
    B: Zeroable,
    C: Zeroable,
    D: Zeroable,
    E: Zeroable,
    F: Zeroable,
    G: Zeroable,
> Zeroable for (A, B, C, D, E, F, G)
{
}
unsafe impl<
    A: Zeroable,
    B: Zeroable,
    C: Zeroable,
    D: Zeroable,
    E: Zeroable,
    F: Zeroable,
    G: Zeroable,
    H: Zeroable,
> Zeroable for (A, B, C, D, E, F, G, H)
{
}

unsafe impl<T, const N: usize> Zeroable for [T; N] where T: Zeroable {}

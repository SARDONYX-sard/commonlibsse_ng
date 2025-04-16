// Unstable Rust code
//
// SPDX-FileCopyrightText: (c) The Rust Project Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// - https://github.com/rust-lang/rust/blob/master/LICENSE-MIT
//! Unstable Memory allocation APIs
mod allocator;
mod impls;

pub use self::impls::global::Global;
pub use self::allocator::{AllocError, Allocator, non_null_from_layout_dangling};

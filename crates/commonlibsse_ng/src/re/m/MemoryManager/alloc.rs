//! Memory allocation functions.
//! This module provides functions for memory allocation, deallocation, and reallocation.
pub mod scrap_alloc;
pub mod tes_global;

#[cfg(not(feature = "test_on_ci"))]
mod rust_api;

#[cfg(not(feature = "test_on_ci"))]
pub use self::rust_api::{alloc, alloc_zeroed, dealloc, realloc};
#[cfg(feature = "test_on_ci")] // Since TESAllocator is not available for CI, use Rust's.
pub use std::alloc::{alloc, alloc_zeroed, dealloc, realloc};

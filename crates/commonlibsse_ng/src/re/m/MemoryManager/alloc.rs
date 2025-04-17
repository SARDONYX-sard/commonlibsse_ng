pub mod global;
mod rust_api;
pub mod scrap_alloc;

#[cfg(not(feature = "test_on_ci"))]
pub use self::rust_api::{alloc, alloc_zeroed, dealloc, realloc};
#[cfg(feature = "test_on_ci")] // Since TESAllocator is not available for CI, use Rust's.
pub use std::alloc::{alloc, alloc_zeroed, dealloc, realloc};

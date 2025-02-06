//! REL dir portion of `CommonLibSSE-NG` written by hand.

pub mod common;
pub mod id;
#[cfg(feature = "win_api")]
pub mod module;
pub mod offset;
pub mod pattern;
pub mod relocation;
pub mod version;

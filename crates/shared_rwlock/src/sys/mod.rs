mod rwlock_inner;
#[cfg(target_os = "windows")]
mod windows;

pub use self::rwlock_inner::RwLock;

#[cfg(target_os = "windows")]
pub(crate) use windows::{errors::MemoryMapError, shared_memory};

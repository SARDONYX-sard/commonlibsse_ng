/// Defines errors that may occur when working with `MemoryMap`.
#[derive(Debug, Clone, PartialEq, Eq, snafu::Snafu)]
pub enum LockError {
    #[cfg(target_os = "windows")]
    #[snafu(transparent)]
    WindowsError { source: crate::sys::MemoryMapError },
}

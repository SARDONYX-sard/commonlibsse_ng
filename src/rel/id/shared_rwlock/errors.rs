/// Defines errors that may occur when working with `MemoryMap`.
#[derive(Debug, snafu::Snafu)]
pub enum MemoryMapError {
    /// Failed to open memory mapping: {source}
    OpenMapping { source: windows::core::Error },

    /// Failed to create memory mapping: {source}
    CreateMapping { source: windows::core::Error },

    /// Failed to map view of file.
    MapView,

    /// Failed to unmap memory view: {source}
    UnmapView { source: windows::core::Error },

    /// Failed to close handle: {source}
    CloseHandle { source: windows::core::Error },
}

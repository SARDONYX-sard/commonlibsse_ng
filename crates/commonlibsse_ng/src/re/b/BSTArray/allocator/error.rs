use core::alloc::Layout;

#[derive(Debug, snafu::Snafu)]
pub enum AllocatorError {
    /// The heap memory you tried to allocate is too large. The `BSTArrayHeapAllocator` only supports allocating less than isize::MAX but requested {requested_size}.
    InvalidLayout { requested_size: usize },
    /// Heap allocation failed. Layout attempted to allocate: {layout:?}
    AllocationFailed { layout: Layout },
}

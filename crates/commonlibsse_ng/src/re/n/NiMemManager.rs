use core::ffi::{c_char, c_void};
use core::ptr::{self, NonNull};

use crate::re::NiAllocator::{NiAllocator, NiMemEventType};

#[derive(Debug)]
#[repr(C)]
pub struct NiMemManager {
    pub allocator: Option<NonNull<NiAllocator>>,
}
const _: () = assert!(core::mem::size_of::<NiMemManager>() == 0x8);

#[allow(clippy::too_many_arguments)]
impl NiMemManager {
    /// Returns the singleton instance of `Self`.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut NiMemManager",
        default = "None",
        id(se = 523759, ae = 410319)
    )]
    pub fn get_singleton() -> Option<&'static NiMemManager> {
        |as_type: AsType| unsafe { as_type.as_ref() }
    }

    /// Returns the singleton instance of `Self`.
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut NiMemManager",
        default = "None",
        id(se = 523759, ae = 410319)
    )]
    pub fn get_singleton_mut() -> Option<&'static mut NiMemManager> {
        |as_type: AsType| unsafe { as_type.as_mut() }
    }

    pub fn allocate(
        &mut self,
        size_in_bytes: usize,
        alignment: usize,
        event_type: NiMemEventType,
        provide_accurate_size_on_deallocate: bool,
        file: Option<*const c_char>,
        line: Option<i32>,
        function: Option<*const c_char>,
    ) -> Option<NonNull<u8>> {
        let allocator = unsafe { self.allocator?.as_mut() };
        let mut size_in_bytes = size_in_bytes;
        let mut alignment = alignment;

        let file = file.unwrap_or(ptr::null_mut());
        let line = line.unwrap_or(-1);
        let function = function.unwrap_or(ptr::null_mut());

        let mem_ptr = (allocator.vtable().Allocate)(
            allocator,
            &mut size_in_bytes,
            &mut alignment,
            event_type,
            provide_accurate_size_on_deallocate,
            file,
            line,
            function,
        )
        .cast();

        NonNull::new(mem_ptr)
    }

    pub fn deallocate(
        &mut self,
        memory: *mut c_void,
        event_type: NiMemEventType,
        size_in_bytes: Option<usize>,
    ) {
        let size_in_bytes = size_in_bytes.unwrap_or(usize::MAX);

        if let Some(allocator) = self.allocator.as_mut() {
            let deallocate = unsafe { allocator.as_mut().vtable().Deallocate };
            (deallocate)(allocator.as_ptr(), memory, event_type, size_in_bytes);
        };
    }

    pub fn reallocate(
        &mut self,
        memory: *mut c_void,
        size_in_bytes: *mut usize,
        alignment: *mut usize,
        event_type: NiMemEventType,
        provide_accurate_size_on_deallocate: bool,
        size_current: usize,
        file: *const c_char,
        line: i32,
        function: *const c_char,
    ) -> Option<NonNull<u8>> {
        let allocator = unsafe { self.allocator?.as_mut() };
        let mem_ptr = (allocator.vtable().Reallocate)(
            allocator,
            memory,
            size_in_bytes,
            alignment,
            event_type,
            provide_accurate_size_on_deallocate,
            size_current,
            file,
            line,
            function,
        )
        .cast();

        NonNull::new(mem_ptr)
    }

    pub fn track_allocate(
        &mut self,
        memory: *const c_void,
        size_in_bytes: usize,
        event_type: NiMemEventType,
        file: *const c_char,
        line: i32,
        function: *const c_char,
    ) -> bool {
        let allocator = match self.allocator {
            Some(mut allocator) => unsafe { allocator.as_mut() },
            None => return false,
        };

        (allocator.vtable().TrackAllocate)(
            allocator,
            memory,
            size_in_bytes,
            event_type,
            file,
            line,
            function,
        )
    }

    pub fn track_deallocate(&mut self, memory: *const c_void, event_type: NiMemEventType) -> bool {
        let allocator = match self.allocator {
            Some(mut allocator) => unsafe { allocator.as_mut() },
            None => return false,
        };

        (allocator.vtable().TrackDeallocate)(allocator, memory, event_type)
    }
}

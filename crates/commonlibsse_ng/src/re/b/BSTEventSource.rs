use crate::re::BSAtomic::BSSpinLock;
use crate::re::BSTArray::BSTArray;

#[commonlibsse_ng_derive_internal::ffi_enum]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BSEventNotifyControl {
    Continue = 0,
    Stop = 1,
}

#[derive(Debug)]
#[repr(C)]
pub struct BSTEventSink<Event> {
    pub vtable: *const BSTEventSinkVtbl<Event>, // VTable pointer
}
const _: () = assert!(size_of::<BSTEventSink<()>>() == 0x8);

#[repr(C)]
pub struct BSTEventSinkVtbl<Event> {
    /// C++ class Destructor equivalent
    pub CxxDrop: unsafe extern "C" fn(this: *mut BSTEventSink<Event>), // 00

    /// - BSTEventSink: pure virtual
    pub ProcessEvent:
        unsafe extern "C" fn(this: *mut BSTEventSink<Event>) -> BSEventNotifyControlFlags, // 01
}

impl<Event> BSTEventSink<Event> {
    pub const fn process_event(
        &self,
        _event: &Event,
        _event_source: &BSTEventSource<Event>,
    ) -> BSEventNotifyControlFlags {
        BSEventNotifyControlFlags::Stop
    }
}

#[repr(C)]
pub struct BSTEventSource<Event> {
    sinks: BSTArray<*mut BSTEventSink<Event>>,             // 00
    pending_registers: BSTArray<*mut BSTEventSink<Event>>, // 18
    pending_unregisters: BSTArray<*mut BSTEventSink<Event>>, // 30
    lock: BSSpinLock,                                      // 48
    notifying: bool,                                       // 50
    pad51: u8,                                             // 51
    pad52: u16,                                            // 52
    pad54: u32,                                            // 54
}
const_assert_eq!(core::mem::size_of::<BSTEventSource<()>>(), 0x58);

impl<Event> Default for BSTEventSource<Event> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<Event> BSTEventSource<Event> {
    pub const fn new() -> Self {
        Self {
            sinks: BSTArray::new(),
            pending_registers: BSTArray::new(),
            pending_unregisters: BSTArray::new(),
            lock: BSSpinLock::new(),
            notifying: false,
            pad51: 0,
            pad52: 0,
            pad54: 0,
        }
    }

    /// # Panics
    pub fn add_event_sink(&mut self, event_sink: *mut BSTEventSink<Event>) {
        let _ = event_sink;
        todo!()
    }

    /// # Panics
    pub fn remove_event_sink(&mut self, event_sink: *mut BSTEventSink<Event>) {
        let _ = event_sink;
        todo!()
    }

    /// # Panics
    pub fn send_event(&mut self, event: &Event) {
        let _ = event;
        todo!()
    }
}

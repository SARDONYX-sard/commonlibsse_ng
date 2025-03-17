use super::{BSAtomic::BSSpinLock, BSTArray::BSTArray};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BSEventNotifyControl {
    Continue = 0,
    Stop = 1,
}

#[derive(Debug)]
#[repr(C)]
pub struct Sink<Event> {
    _phantom: std::marker::PhantomData<Event>,
}

impl<Event> Sink<Event> {
    pub const fn process_event(
        &self,
        _event: &Event,
        _event_source: &BSTEventSource<Event>,
    ) -> BSEventNotifyControl {
        BSEventNotifyControl::Continue
    }
}

#[repr(C)]
pub struct BSTEventSource<Event> {
    sinks: BSTArray<*mut Sink<Event>>,               // 00
    pending_registers: BSTArray<*mut Sink<Event>>,   // 18
    pending_unregisters: BSTArray<*mut Sink<Event>>, // 30
    lock: BSSpinLock,                                // 48
    notifying: bool,                                 // 50
    pad51: u8,                                       // 51
    pad52: u16,                                      // 52
    pad54: u32,                                      // 54
}
const_assert_eq!(core::mem::size_of::<BSTEventSource<()>>(), 0x58);

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
    pub fn add_event_sink(&mut self, event_sink: *mut Sink<Event>) {
        todo!()
    }

    /// # Panics
    pub fn remove_event_sink(&mut self, event_sink: *mut Sink<Event>) {
        todo!()
    }

    /// # Panics
    pub fn send_event(&mut self, event: &Event) {
        todo!()
    }
}

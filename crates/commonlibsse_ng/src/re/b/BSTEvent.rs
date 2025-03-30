use crate::re::BSAtomic::BSSpinLock;
use crate::re::BSTArray::BSTArray;

/// Represents the event notification control, mapping to the C++ enum.
#[commonlibsse_ng_derive_internal::ffi_enum]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BSEventNotifyControl {
    Continue = 0,
    Stop = 1,
}

pub struct BSTEventSink<Event> {
    vtable: *const BSTEventSinkVtbl<Event>,
}
const _: () = assert!(core::mem::size_of::<BSTEventSink<*mut ()>>() == 0x8);

impl<Event> BSTEventSink<Event> {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn process_event(
        &mut self,
        event: *const Self,
        event_source: *const BSTEventSource<Event>,
    ) -> BSEventNotifyControl {
        #[allow(clippy::option_if_let_else)]
        match self.vtable() {
            Some(vtable) => unsafe { (vtable.ProcessEvent)(self, event, event_source) }.to_enum(),
            None => None,
        }
        .unwrap_or(BSEventNotifyControl::Stop)
    }

    #[inline]
    pub const fn vtable(&self) -> Option<&BSTEventSinkVtbl<Event>> {
        unsafe { self.vtable.as_ref() }
    }
}

pub struct BSTEventSinkVtbl<Event> {
    /// C++ destructor `~BSTEventSink`
    pub CxxDrop: unsafe extern "C" fn(this: *mut BSTEventSink<Event>),

    pub ProcessEvent: unsafe extern "C" fn(
        this: *mut BSTEventSink<Event>,
        event: *const BSTEventSink<Event>,
        eventSource: *const BSTEventSource<Event>,
    ) -> BSEventNotifyControlFlags,
}

#[repr(C)]
pub struct BSTEventSource<Event> {
    sinks: BSTArray<*mut BSTEventSink<Event>>,
    pendingRegisters: BSTArray<*mut BSTEventSink<Event>>,
    pendingUnregisters: BSTArray<*mut BSTEventSink<Event>>,
    lock: BSSpinLock,
    notifying: bool,
    pad51: u8,
    pad52: u16,
    pad54: u32,
}
const _: () = {
    assert!(core::mem::offset_of!(BSTEventSource<*mut ()>, sinks) == 0x00);
    assert!(core::mem::offset_of!(BSTEventSource<*mut ()>, pendingRegisters) == 0x18);
    assert!(core::mem::offset_of!(BSTEventSource<*mut ()>, pendingUnregisters) == 0x30);
    assert!(core::mem::offset_of!(BSTEventSource<*mut ()>, lock) == 0x48);
    assert!(core::mem::offset_of!(BSTEventSource<*mut ()>, notifying) == 0x50);
    assert!(core::mem::offset_of!(BSTEventSource<*mut ()>, pad51) == 0x51);
    assert!(core::mem::offset_of!(BSTEventSource<*mut ()>, pad52) == 0x52);
    assert!(core::mem::offset_of!(BSTEventSource<*mut ()>, pad54) == 0x54);

    assert!(core::mem::size_of::<BSTEventSource<*mut ()>>() == 0x58);
};

impl<Event> BSTEventSource<Event> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            sinks: BSTArray::new(),
            pendingRegisters: BSTArray::new(),
            pendingUnregisters: BSTArray::new(),
            lock: BSSpinLock::new(),
            notifying: false,
            pad51: 0,
            pad52: 0,
            pad54: 0,
        }
    }

    pub fn add_event_sink(&mut self, sink: *mut BSTEventSink<Event>) {
        let _guard = self.lock.lock();

        if sink.is_null() {
            return;
        }

        if self.notifying {
            if !self.pendingRegisters.contains(&sink) {
                self.pendingRegisters.push(sink);
            }
        } else if !self.sinks.contains(&sink) {
            self.sinks.push(sink);
        }

        self.pendingUnregisters.retain(|&s| s != sink);
    }

    pub fn remove_event_sink(&mut self, sink: *mut BSTEventSink<Event>) {
        let _guard = self.lock.lock();

        if self.notifying {
            if !self.pendingUnregisters.contains(&sink) {
                self.pendingUnregisters.push(sink);
            }
        } else {
            self.sinks.retain(|&s| s != sink);
        }

        self.pendingUnregisters.retain(|&s| s != sink);
    }

    pub fn send_event(&mut self, event: *const BSTEventSink<Event>) {
        let _guard = self.lock.lock();

        let was_notifying = self.notifying;
        self.notifying = true;

        if !was_notifying && !self.pendingRegisters.is_empty() {
            for sink in self.pendingRegisters.drain(..) {
                if !self.sinks.contains(&sink) {
                    self.sinks.push(sink);
                }
            }
        }

        for sink in self.sinks.iter() {
            let ret = (unsafe { &mut **sink }).process_event(event, self);
            if !self.pendingUnregisters.contains(sink) && ret == BSEventNotifyControl::Stop {
                break;
            }
        }

        self.notifying = was_notifying;

        if !was_notifying && !self.pendingUnregisters.is_empty() {
            for sink in self.pendingUnregisters.drain(..) {
                self.sinks.retain(|&s| s != sink);
            }
        }
    }
}

impl<Event> Default for BSTEventSource<Event> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

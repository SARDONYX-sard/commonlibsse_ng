use core::ptr::NonNull;

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

#[derive(Debug)]
pub struct BSTEventSink<Event> {
    pub vtable: Option<NonNull<BSTEventSinkVtbl<Event>>>,
}
const _: () = assert!(core::mem::size_of::<BSTEventSink<*mut ()>>() == 0x8);

impl<Event> BSTEventSink<Event> {
    /// # Safety
    pub unsafe fn process_event(
        &mut self,
        event: *const Self,
        event_source: *const BSTEventSource<Event>,
    ) -> BSEventNotifyControl {
        self.vtable
            .map(|ptr| unsafe { ptr.as_ref() })
            .and_then(|vtable| {
                unsafe { (vtable.ProcessEvent)(self, event, event_source) }.to_enum()
            })
            .unwrap_or(BSEventNotifyControl::Stop)
    }
}

pub struct BSTEventSinkVtbl<Event> {
    /// C++ destructor `~BSTEventSink`
    pub CxxDrop: unsafe extern "C" fn(this: *mut BSTEventSink<Event>),

    pub ProcessEvent: unsafe extern "C" fn(
        this: *mut BSTEventSink<Event>,
        event: *const BSTEventSink<Event>,
        eventSource: *const BSTEventSource<Event>,
    ) -> BSEventNotifyControl_CEnum,
}

#[repr(C)]
#[derive(Debug)]
pub struct BSTEventSource<Event> {
    pub sinks: BSTArray<*mut BSTEventSink<Event>>,
    pub pendingRegisters: BSTArray<*mut BSTEventSink<Event>>,
    pub pendingUnregisters: BSTArray<*mut BSTEventSink<Event>>,
    pub lock: BSSpinLock,
    pub notifying: bool,
    pub pad51: u8,
    pub pad52: u16,
    pub pad54: u32,
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
    pub fn new() -> Self {
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

    /// # Safety
    pub unsafe fn send_event(&mut self, event: *const BSTEventSink<Event>) {
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
            let ret = unsafe { (**sink).process_event(event, self) };
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

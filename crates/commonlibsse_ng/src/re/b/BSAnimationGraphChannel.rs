use crate::re::BSFixedString::BSFixedString;
use crate::re::BSIntrusiveRefCounted::BSIntrusiveRefCounted;
use crate::re::offsets_rtti::RTTI_BSAnimationGraphChannel;
use crate::re::offsets_vtable::VTABLE_BSAnimationGraphChannel;
use crate::rel::id::VariantID;

use super::BSIntrusiveRefCounted::BSIntrusiveRefCountedTrait;

#[repr(C)]
#[derive(Debug)]
pub struct BSAnimationGraphChannel {
    /// NOTE:
    /// If the parent does not have a virtual function even if base is inherited, vtable comes before the
    /// inherited parent member if the child has a virtual function.
    /// - See [playground](https://godbolt.org/z/96rGaTG4e)
    pub vtable: *const BSAnimationGraphChannelVtbl,

    /// Base class `BSIntrusiveRefCounted`.
    pub __base: BSIntrusiveRefCounted,

    /// Padding for alignment.
    pub pad0C: u32,

    /// Channel name.
    pub channelName: BSFixedString,

    /// Value associated with the channel.
    pub value: u32,

    /// Padding for alignment.
    pub pad1C: u32,
}

const _: () = {
    assert!(core::mem::offset_of!(BSAnimationGraphChannel, __base) == 0x8);
    assert!(core::mem::offset_of!(BSAnimationGraphChannel, pad0C) == 0x0C);
    assert!(core::mem::offset_of!(BSAnimationGraphChannel, channelName) == 0x10);
    assert!(core::mem::offset_of!(BSAnimationGraphChannel, value) == 0x18);
    assert!(core::mem::offset_of!(BSAnimationGraphChannel, pad1C) == 0x1C);
    assert!(core::mem::size_of::<BSAnimationGraphChannel>() == 0x20);
};

impl BSIntrusiveRefCountedTrait for BSAnimationGraphChannel {
    #[inline]
    fn inc_ref(&self) -> u32 {
        self.__base.inc_ref()
    }

    #[inline]
    fn dec_ref(&self) -> u32 {
        self.__base.dec_ref()
    }
}

impl Default for BSAnimationGraphChannel {
    #[inline]
    fn default() -> Self {
        Self {
            vtable: &BS_ANIMATION_GRAPH_CHANNEL_VTBL,
            __base: BSIntrusiveRefCounted::default(),
            pad0C: 0,
            channelName: BSFixedString::default(),
            value: 0,
            pad1C: 0,
        }
    }
}

impl BSAnimationGraphChannel {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_BSAnimationGraphChannel;

    /// Address & offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_BSAnimationGraphChannel;

    /// Virtual function to poll channel update (abstract function in C++).
    pub fn poll_channel_update_impl(&self, _a_arg1: bool) {
        // Implementation in derived classes.
    }

    /// Virtual function to reset the channel (abstract function in C++).
    pub fn reset_impl(&self) {
        // Implementation in derived classes.
    }
}

/// The virtual function table for `BSAnimationGraphChannel`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
#[derive(Debug)]
pub struct BSAnimationGraphChannelVtbl {
    /// Destructor function pointer.
    pub CxxDrop: fn(this: &mut BSAnimationGraphChannel),

    /// Function pointer for polling channel update.
    pub PollChannelUpdateImpl: fn(this: &BSAnimationGraphChannel, a_arg1: bool),

    /// Function pointer for resetting the channel.
    pub ResetImpl: fn(this: &BSAnimationGraphChannel),
}

impl Default for BSAnimationGraphChannelVtbl {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl BSAnimationGraphChannelVtbl {
    /// Creates a new default virtual table with stubbed functions.
    pub const fn new() -> Self {
        const fn CxxDrop(_this: &mut BSAnimationGraphChannel) {}

        const fn PollChannelUpdateImpl(_this: &BSAnimationGraphChannel, _a_arg1: bool) {}

        const fn ResetImpl(_this: &BSAnimationGraphChannel) {}

        Self { CxxDrop, PollChannelUpdateImpl, ResetImpl }
    }
}
static BS_ANIMATION_GRAPH_CHANNEL_VTBL: BSAnimationGraphChannelVtbl =
    BSAnimationGraphChannelVtbl::new();

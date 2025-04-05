/// related to cell transitions
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PositionPlayerEvent {
    type_: EVENT_TYPEFlags,
}
const _: () = assert!(core::mem::size_of::<PositionPlayerEvent>() == 0x4);

#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum EVENT_TYPE {
    #[default]
    Pre = 0,
    PreUpdatePackages = 1,
    PostUpdatePackages = 2,
    Post = 3,
    Finish = 4,
}

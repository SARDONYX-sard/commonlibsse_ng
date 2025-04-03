use crate::re::BSFixedString::BSFixedString;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MenuOpenCloseEvent {
    menuName: BSFixedString, // 0x00
    opening: bool,           // 0x08
    pad09: u8,               // 0x09
    pad0A: u16,              // 0x0A
    pad0C: u32,              // 0x0C
}
const _: () = assert!(core::mem::size_of::<MenuOpenCloseEvent>() == 0x10);

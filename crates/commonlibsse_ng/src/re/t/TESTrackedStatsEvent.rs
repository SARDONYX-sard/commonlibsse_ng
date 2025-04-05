use crate::re::BSFixedString::BSFixedString;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TESTrackedStatsEvent {
    stat: BSFixedString, // 0x00
    value: u32,          // 0x08
    pad0C: u32,          // 0x0C
}
const _: () = assert!(core::mem::size_of::<TESTrackedStatsEvent>() == 0x10);

use crate::re::SpellItem::SpellItem;

#[commonlibsse_ng_derive_internal::to_bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Flag {
    None = 0,
    OnDeath = 1 << 0,
}

#[repr(C)]
#[derive(Debug)]
pub struct CriticalData {
    pub percentMulti: f32,      // 0x00
    pub pad04: u32,             // 0x04
    pub effect: *mut SpellItem, // 0x08
    pub damage: u16,            // 0x10
    pub flags: Flag,            // 0x12
    pub pad13: u8,              // 0x13
    pub pad14: u32,             // 0x14
}
const _: () = assert!(core::mem::size_of::<CriticalData>() == 0x18);

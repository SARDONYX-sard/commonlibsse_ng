use crate::re::BSFixedString::BSFixedString;

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MenuModeChangeEvent {
    menu: BSFixedString, // 0x00
    mode: Mode_CEnum,    // 0x08
}
const _: () = assert!(core::mem::size_of::<MenuModeChangeEvent>() == 0x10);

#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Mode {
    #[default]
    None = 0xff,
    Hidden = 0,
    Displayed = 1,
}

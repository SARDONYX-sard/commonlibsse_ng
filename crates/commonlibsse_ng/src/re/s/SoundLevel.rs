#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum SOUND_LEVEL {
    Loud = 0,
    Normal = 1,
    Silent = 2,
    VeryLoud = 3,
    Quiet = 4,
}

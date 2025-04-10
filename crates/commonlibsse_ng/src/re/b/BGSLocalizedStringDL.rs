#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BGSLocalizedStringDL {
    id: u32
}
const _: () = assert!(core::mem::size_of::<BGSLocalizedStringDL>() == 0x4);

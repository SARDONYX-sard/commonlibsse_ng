use core::ffi::c_char;

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileID {
    pub file: u32,
    pub ext: [c_char; 4],
}
const _: () = assert!(core::mem::size_of::<FileID>() == 0x8);

#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ID {
    pub __base: FileID,
    pub dir: u32,
}
const _: () = assert!(core::mem::size_of::<ID>() == 0xC);

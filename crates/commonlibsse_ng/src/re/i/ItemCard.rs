use core::ffi::c_void;

use crate::re::BSString::BSString;
use crate::re::GFxMovieView;
use crate::re::GFxValue::GFxValue;
use crate::re::GPtr::GPtr;

#[derive(Debug)]
#[repr(C)]
pub struct ItemCard {
    pub obj: GFxValue,            // 0x00 - kObject
    pub infoText: BSString,       // 0x18
    pub unk28: *mut c_void,       // 0x28
    pub unk30: u32,               // 0x30
    pub pad34: u32,               // 0x34
    pub view: GPtr<GFxMovieView>, // 0x38
}
const _: () = assert!(core::mem::size_of::<ItemCard>() == 0x40);

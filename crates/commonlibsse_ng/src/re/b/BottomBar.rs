use crate::re::GFxValue::GFxValue;

#[derive(Debug)]
#[repr(C)]
pub struct BottomBar {
    pub ob: GFxValue, // 0x00 - kObject
}
const _: () = assert!(core::mem::size_of::<BottomBar>() == 0x18);

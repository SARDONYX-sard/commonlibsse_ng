/// FIXME: current implementation
#[repr(C)]
#[derive(Debug)]
pub struct TESValueForm {
    _data: [u8; 0x10], // 0x0A0 - 0x090 = 0x10
}
const _: () = assert!(core::mem::size_of::<TESValueForm>() == 0x10);

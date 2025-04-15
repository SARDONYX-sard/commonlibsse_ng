/// FIXME: current implementation
#[repr(C)]
#[derive(Debug)]
pub struct TESWeightForm {
    _data: [u8; 0x10], // 0x0B0 - 0x0A0 = 0x10
}
const _: () = assert!(core::mem::size_of::<TESWeightForm>() == 0x10);

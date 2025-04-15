/// FIXME: current implementation
#[repr(C)]
#[derive(Debug)]
pub struct TESAttackDamageForm {
    _data: [u8; 0x10], // 0x0C0 - 0x0B0 = 0x10
}
const _: () = assert!(core::mem::size_of::<TESAttackDamageForm>() == 0x10);

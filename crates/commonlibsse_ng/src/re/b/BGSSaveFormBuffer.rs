use crate::re::BGSSaveGameBuffer::BGSSaveGameBuffer;
use crate::re::TESForm::TESForm;
use crate::re::offsets_rtti::RTTI_BGSSaveFormBuffer;
use crate::re::offsets_vtable::VTABLE_BGSSaveFormBuffer;
use crate::rel::id::VariantID;

#[derive(Debug)]
pub struct BGSSaveFormBuffer {
    /// Inherited class
    pub _base: BGSSaveGameBuffer,
    pub unk18: u64,
    pub unk20: u64,
    pub form: *mut TESForm,
}

const _: () = {
    assert!(core::mem::offset_of!(BGSSaveFormBuffer, unk18) == 0x18);
    assert!(core::mem::offset_of!(BGSSaveFormBuffer, unk20) == 0x20);

    assert!(core::mem::size_of::<BGSSaveFormBuffer>() == 0x30);
};

impl BGSSaveFormBuffer {
    pub const RTTI: VariantID = RTTI_BGSSaveFormBuffer;
    pub const VTABLE: [VariantID; 1] = VTABLE_BGSSaveFormBuffer;
}

use crate::re::offsets_rtti::RTTI_BSISoundDescriptor;
use crate::re::offsets_rtti::RTTI_BSISoundDescriptor__BSIPlaybackCharacteristics;
use crate::re::offsets_vtable::VTABLE_BSISoundDescriptor;
use crate::re::offsets_vtable::VTABLE_BSISoundDescriptor__BSIPlaybackCharacteristics;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BSISoundDescriptor {
    pub vtbl: *const BSISoundDescriptorVtbl,
}
const _: () = assert!(core::mem::size_of::<BSISoundDescriptor>() == 0x8);

impl BSISoundDescriptor {
    pub const RTTI: VariantID = RTTI_BSISoundDescriptor;
    pub const VTABLE: [VariantID; 1] = VTABLE_BSISoundDescriptor;
}

#[repr(C)]
pub struct BSISoundDescriptorVtbl {
    pub CxxDrop: extern "C" fn(this: *mut BSISoundDescriptor), // 0x00
    pub Unk_01: extern "C" fn(this: *mut BSISoundDescriptor),  // 0x01
    pub Unk_02: extern "C" fn(this: *mut BSISoundDescriptor),  // 0x02
}
const _: () = {
    const ACTUAL_SIZE: usize = core::mem::size_of::<BSISoundDescriptorVtbl>();
    const EXPECTED_SIZE: usize = (0x02 + 1) * core::mem::size_of::<usize>();
    assert!(ACTUAL_SIZE == EXPECTED_SIZE);
};

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BSIPlaybackCharacteristics {
    pub vtbl: *const BSIPlaybackCharacteristicsVtbl,
}
const _: () = assert!(core::mem::size_of::<BSIPlaybackCharacteristics>() == 0x8);

impl BSIPlaybackCharacteristics {
    pub const RTTI: VariantID = RTTI_BSISoundDescriptor__BSIPlaybackCharacteristics;
    pub const VTABLE: [VariantID; 1] = VTABLE_BSISoundDescriptor__BSIPlaybackCharacteristics;
}

#[repr(C)]
pub struct BSIPlaybackCharacteristicsVtbl {
    pub CxxDrop: extern "C" fn(this: *mut BSIPlaybackCharacteristics), // 0x0
    pub GetFrequencyShift: extern "C" fn(this: *mut BSIPlaybackCharacteristics) -> u8, // 0x1
    pub GetFrequencyVariance: extern "C" fn(this: *mut BSIPlaybackCharacteristics) -> u8, // 0x2
    pub GetPriority: extern "C" fn(this: *mut BSIPlaybackCharacteristics) -> u8, // 0x3
    pub GetStaticAttenuation: extern "C" fn(this: *mut BSIPlaybackCharacteristics) -> u16, // 0x4
    pub GetDBVariance: extern "C" fn(this: *mut BSIPlaybackCharacteristics) -> u8, // 0x5
}
const _: () = {
    const ACTUAL_SIZE: usize = core::mem::size_of::<BSIPlaybackCharacteristicsVtbl>();
    const EXPECTED_SIZE: usize = (0x05 + 1) * core::mem::size_of::<usize>();
    assert!(ACTUAL_SIZE == EXPECTED_SIZE);
};

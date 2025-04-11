use core::ptr::NonNull;

use crate::re::BSExtraData::BSExtraData;
use crate::re::ExtraDataList::ExtraDataList;
use crate::re::ExtraDataType::ExtraDataType;

const CACHED_EXTRA_DATA_SIZE: usize = (ExtraDataType::ResourcesPreload.bits() + 1) as usize;

#[derive(Debug)]
#[repr(C)]
pub struct TLSData {
    pub unk000: [u8; 0x10], // 0x000
    ///  - on GetExtraData(), this gets checked against the GlobalStateCounter,
    ///    which is incremented every time extra data is changed or removed:if they're not equal,
    ///    then the following cached extra data is zeroed-out and reached
    pub stateCounter: u32, // 0x010
    pub pad014: u32,        // 0x014
    pub cachedExtraDataList: *mut ExtraDataList, // 0x018

    /// - ExtraData types up to ResourcesPreload (0xB5) are cached
    pub cachedExtraData: [*mut BSExtraData; CACHED_EXTRA_DATA_SIZE], // 0x020
    pub unk5D0: [u8; 0x30],  // 0x5D0
    pub consoleMode: bool,   // 0x600
    pub unk601: [u8; 0x167], // 0x601
    pub taskFlag: u32,       // 0x768 -- unknown enum

                             // ... many others ...
}
const _: () = {
    assert!(core::mem::offset_of!(TLSData, consoleMode) == 0x600);
    assert!(core::mem::offset_of!(TLSData, taskFlag) == 0x768);
};

impl TLSData {
    #[inline]
    pub fn get_static_tls_data() -> Option<NonNull<Self>> {
        let index = get_tls_index()? as usize;
        unsafe {
            let tls_data_array = read_gs_qword::<*mut Self>(0x58);
            let tls_data = *(tls_data_array.add(index));
            NonNull::new(tls_data)
        }
    }
}

#[commonlibsse_ng_derive_internal::relocate(
    cast_as = "*const u32",
    deref_once,
    id(se = 515064, ae = 401203),
    default = "None"
)]
#[inline]
fn get_tls_index() -> Option<u32> {
    |tls_index: DerefType| Some(tls_index)
}

#[inline]
unsafe fn read_gs_qword<T>(offset: u64) -> *mut T {
    let value: *mut T;
    unsafe {
        core::arch::asm!(
            "mov {}, gs:[{}]",
            out(reg) value,
            in(reg) offset,
            options(nostack, preserves_flags),
        );
    }
    value
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FORM {
    pub form: u32,            // 00
    pub length: u32,          // 04
    pub flags: u32,           // 08
    pub form_id: u32,         // 0C
    pub version_control: u32, // 10
    pub form_version: u16,    // 14
    pub vc_version: u16,      // 16
}
const_assert_eq!(core::mem::size_of::<FORM>(), 0x18);

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FORM_GROUP {
    pub group_data: FORM,  // 00
    pub group_offset: u64, // 18
}
const_assert_eq!(core::mem::size_of::<FORM_GROUP>(), 0x20);

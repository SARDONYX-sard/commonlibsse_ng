use crate::re::BSFixedString::BSFixedString;
use crate::re::IUIMessageData::IUIMessageData;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UIMessage {
    menu: BSFixedString,          // 0x00
    type_: UI_MESSAGE_TYPE_CEnum, // 0x08
    pad0C: u32,                   // 0x0C
    data: *mut IUIMessageData,    // 0x10
    isPooled: bool,               // 0x18
    pad19: u8,                    // 0x19
    pad1A: u16,                   // 0x1A
    pad1C: u32,                   // 0x1C
}
const _: () = assert!(core::mem::size_of::<IUIMessageData>() == 0x10);

#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum UI_MESSAGE_TYPE {
    #[default]
    Update = 0,
    Show = 1,
    Reshow = 2,
    Hide = 3,
    ForceHide = 4,

    /// BSUIScaleformData
    ScaleformEvent = 6,
    /// BSUIMessageData
    UserEvent = 7,
    /// InventoryUpdateData
    InventoryUpdate = 8,
    UserProfileChange = 9,
    MUStatusChange = 10,
    ResumeCaching = 11,
    UpdateController = 12,
    ChatterEvent = 13,
}

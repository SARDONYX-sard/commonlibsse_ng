use crate::re::BSTArray::BSTArray;
use crate::re::GFxMovieView;
use crate::re::GFxValue::GFxValue;
use crate::re::GPtr::GPtr;
use crate::re::StandardItemData::StandardItemData;

#[repr(C)]
#[derive(Debug)]
pub struct Item {
    pub data: StandardItemData, // 0x00
    pub obj: GFxValue,          // 0x18 - kObject
    pub unk30: u64,             // 0x30
    pub unk38: u64,             // 0x38
}
const _: () = assert!(core::mem::size_of::<Item>() == 0x40);

#[repr(C)]
#[derive(Debug)]
pub struct ItemList {
    // - offset: 0x00
    pub view: GPtr<GFxMovieView>,
    // - offset: 0x08 - kDisplayObject - "_level0.Menu_mc.inventoryLists.panelContainer.itemList"
    pub root: GFxValue,
    // - offset: 0x20 - kArray - root.GetMember("entryList", &entryList);
    pub entryList: GFxValue,
    // - offset: 0x38
    pub items: BSTArray<*mut Item>,
    // - offset: 0x50
    pub unk50: bool,
    // - offset: 0x51
    pub pad51: u8,
    // - offset: 0x52
    pub pad52: u16,
    // - offset: 0x54
    pub pad54: u32,
}
const _: () = assert!(core::mem::size_of::<ItemList>() == 0x58);

use crate::re::BSTArray::BSTArray;
use crate::re::GFxMovieView;
use crate::re::GFxValue::{GFxValue, Value};
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

impl ItemList {
    pub fn get_selected_item(&self) -> Option<*mut Item> {
        if self.unk50 {
            return None;
        }

        let selected_index = self.root.get_member(c"selectedIndex")?;
        let Value::Number(index) = selected_index.get_value()? else {
            return None;
        };

        if (0.0..(self.items.len() as f64)).contains(&index) {
            Some(self.items[index as usize])
        } else {
            None
        }
    }
}

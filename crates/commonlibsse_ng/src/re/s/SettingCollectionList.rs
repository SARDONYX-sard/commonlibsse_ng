//! # SettingCollectionList

use crate::re::BSTList::BSSimpleList;
use crate::re::Setting::Setting;
use crate::re::SettingCollection::SettingCollection;
use crate::re::SettingCollectionMap::SettingCollectionMapVtbl;

#[derive(Debug)]
#[repr(C)]
pub struct SettingCollectionList<T> {
    /// Base `SettingCollection<T>` struct.
    pub __base: SettingCollection<T>, // 0x000
    pub settings: BSSimpleList<*mut T>, // 0x118
}
const _: () = {
    assert!(core::mem::offset_of!(SettingCollectionList::<Setting>, __base) == 0x00);
    assert!(core::mem::offset_of!(SettingCollectionList::<Setting>, settings) == 0x118);
    assert!(core::mem::size_of::<SettingCollectionList::<Setting>>() == 0x128);
};

/// The virtual function table for `SettingCollectionList<T>`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
pub struct SettingCollectionListVtbl<T> {
    pub __base: SettingCollectionMapVtbl<T>,
}

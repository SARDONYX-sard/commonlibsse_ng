use crate::re::BSTEvent::BSTEventSource;
use crate::re::InventoryEntryData::InventoryEntryData;
use crate::re::TESObjectREFR::TESObjectREFR;

#[commonlibsse_ng_derive_internal::relocate_fn(se_id = 15980, ae_id = 16225)]
unsafe fn get_event_source() -> BSTEventSource<Event> {}

#[derive(Debug)]
#[repr(C)]
pub struct Event {
    obj_refr: TESObjectREFR,
    entry_data: InventoryEntryData,
    next_count: i32,
    prev_count: i32,
}

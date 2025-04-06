use crate::re::BGSLocation;
use crate::re::BSPointerHandle::ObjectRefHandle;
use crate::re::TESForm::TESForm;

#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum ACQUIRE_TYPE {
    None = 0,
    Steal = 1,
    Buy = 2,
    PickPocket = 3,
    Pickup = 4,
    Container = 5,
    DeadBody = 6,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BGSAddToPlayerInventoryEvent {
    ownerRef: ObjectRefHandle,
    containerRef: ObjectRefHandle,
    location: *mut BGSLocation,
    itemBase: *mut TESForm,
    acquireType: ACQUIRE_TYPEFlags,
}
const _: () = assert!(core::mem::size_of::<BGSAddToPlayerInventoryEvent>() == 0x20);

impl BGSAddToPlayerInventoryEvent {
    #[commonlibsse_ng_derive_internal::relocate(
        cast_as = "*mut u32",
        default = "None",
        id(se = 508412, ae = 380074)
    )]
    pub fn get_index() -> Option<u32> {
        |as_type: AsType| unsafe { as_type.as_ref().map(|p| *p) }
    }
}

use crate::re::BSFixedString::BSFixedString;
use crate::re::TESObjectREFR::TESObjectREFR;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BSAnimationGraphEvent {
    pub tag: BSFixedString,
    pub holder: *mut TESObjectREFR,
    pub payload: BSFixedString,
}
const _: () = assert!(core::mem::size_of::<BSAnimationGraphEvent>() == 0x18);

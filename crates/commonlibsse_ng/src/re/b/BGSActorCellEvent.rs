use crate::re::BGSActorEvent::BGSActorEvent;
use crate::re::BSCoreTypes::FormID;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct BGSActorCellEvent {
    pub __base: BGSActorEvent,
    cellID: FormID,
    flags: CellFlag,
}
const _: () = assert!(core::mem::size_of::<BGSActorCellEvent>() == 0xc);

bitflags::bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct CellFlag: u32 {
        const ENTER = 0;
        const Leave = 1;
    }
}

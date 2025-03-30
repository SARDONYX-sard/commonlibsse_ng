use crate::re::BSPointerHandle::ActorHandle;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct BGSActorEvent {
    pub actor: ActorHandle,
}
const _: () = assert!(core::mem::size_of::<BGSActorEvent>() == 0x4);

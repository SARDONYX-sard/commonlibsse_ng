mod enums;
mod local_map;
mod runtime_data;

pub use enums::*;
pub use local_map::*;

use crate::re::BSAnimationGraphEvent::BSAnimationGraphEvent;
use crate::re::BSCoreTypes::RefHandle;
use crate::re::BSTEvent::BSTEventSink;
use crate::re::FormTypes::FormType;
use crate::re::Misc::LookupReferenceByHandle_ActorImpl;
use crate::re::NiSmartPointer::NiPointer;
use crate::re::TESObjectREFR::{TESObjectREFR, TESObjectREFRVtbl};
use crate::re::offsets_rtti::RTTI_Actor;
use crate::re::offsets_vtable::VTABLE_Actor;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct Actor {
    pub __base: TESObjectREFR,
}
const _: () = assert!(core::mem::size_of::<Actor>() == 0x78);

impl Actor {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_Actor;

    /// Address & offset of the virtual function table.
    ///
    /// The number of tables is the same as the number of classes with inherited virtual functions.
    pub const VTABLE: [VariantID; 10] = VTABLE_Actor;

    /// The `FormType` value for Actor.
    pub const FORM_TYPE: FormType = FormType::ActorCharacter;

    #[inline]
    pub fn lookup_reference_by_handle(ref_handle: RefHandle) -> NiPointer<Self> {
        let mut actor_ptr = NiPointer::new();
        LookupReferenceByHandle_ActorImpl(&ref_handle, &mut actor_ptr);
        actor_ptr
    }

    #[inline]
    pub fn lookup_by_handle_actor(ref_handle: RefHandle, refr_out: &mut NiPointer<Self>) -> bool {
        LookupReferenceByHandle_ActorImpl(&ref_handle, refr_out)
    }

    pub fn add_animation_graph_event_sink(
        &self,
        sink: *mut BSTEventSink<BSAnimationGraphEvent>,
    ) -> bool {
        let mut graph_manager = match self.__base.__base3.get_animation_graph_manager() {
            Some(graph) => graph,
            None => return false,
        };

        let mut sinked = false;
        for anim_graph in &graph_manager.graphs {
            if sinked {
                break;
            }

            for other_sink in &anim_graph.__base3.sinks {
                if sink == *other_sink {
                    sinked = true;
                    break;
                }
            }
        }

        if !sinked {
            let anim_graph = match graph_manager.graphs.get_mut(0) {
                Some(sink) => sink,
                None => return false,
            };
            anim_graph.__base3.add_event_sink(sink);
            return true;
        };

        false
    }
}

impl crate::re::NiSmartPointer::RefCountable for Actor {
    #[inline]
    fn inc_ref_count(&self) {
        self.__base.__base1.__base.inc_ref_count();
    }

    #[inline]
    fn dec_ref_count(&mut self) {
        self.__base.__base1.__base.dec_ref_count();
    }
}

pub struct ActorVtbl {
    pub __base: TESObjectREFRVtbl,
}
const _: () = {
    const VTABLE_SIZE: usize = core::mem::size_of::<ActorVtbl>();
    const EXPECTED_SIZE: usize = (0x129 + 1) * core::mem::size_of::<usize>();
    // assert!(VTABLE_SIZE == EXPECTED_SIZE);
};

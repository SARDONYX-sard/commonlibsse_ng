mod enums;
mod local_map;
mod runtime_data;

pub use self::enums::*;
pub use self::local_map::*;
pub use self::runtime_data::ACTOR_RUNTIME_DATA;

use crate::re::BSAnimationGraphEvent::BSAnimationGraphEvent;
use crate::re::BSCoreTypes::RefHandle;
use crate::re::BSTEvent::BSTEventSink;
use crate::re::FormTypes::FormType;
use crate::re::InventoryEntryData::InventoryEntryData;
use crate::re::Misc::LookupReferenceByHandle_ActorImpl;
use crate::re::NiSmartPointer::NiPointer;
use crate::re::SpellItem::SpellItem;
use crate::re::TESFaction;
use crate::re::TESObjectREFR::{TESObjectREFR, TESObjectREFRVtbl};
use crate::re::offsets_rtti::RTTI_Actor;
use crate::re::offsets_vtable::VTABLE_Actor;
use crate::rel::id::VariantID;
use crate::rel::relocation::RelocationError;
use crate::rel::relocation::relocate_member_if_newer;
use crate::rel::relocation::relocate_member_if_newer_mut;
use crate::skse::version::RUNTIME_SSE_1_6_629;
use core::ptr::NonNull;

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

    /// Get runtime offset definition fields.
    ///
    /// # Errors
    /// - This function may return an error if the module's state cannot be accessed, or if the `map_active` call fails when fetching the current version.
    /// - If the pointer is null
    /// - If the pointer is unaligned
    #[inline]
    pub fn get_actor_runtime_data(&self) -> Result<&ACTOR_RUNTIME_DATA, RelocationError> {
        unsafe { relocate_member_if_newer(RUNTIME_SSE_1_6_629, self, 0xE0, 0xE8) }
    }

    /// Get mutable runtime offset definition fields.
    ///
    /// # Errors
    /// - This function may return an error if the module's state cannot be accessed, or if the `map_active` call fails when fetching the current version.
    /// - If the pointer is null
    /// - If the pointer is unaligned
    #[inline]
    pub fn get_actor_runtime_data_mut(
        &mut self,
    ) -> Result<&mut ACTOR_RUNTIME_DATA, RelocationError> {
        unsafe { relocate_member_if_newer_mut(RUNTIME_SSE_1_6_629, self, 0xE0, 0xE8) }
    }

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

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 37787, ae_id = 38736)]
    pub fn add_cast_power(&mut self, power: Option<NonNull<SpellItem>>) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 36218, ae_id = 3719371937198)]
    pub fn add_death_items(&mut self) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 37771, ae_id = 38716)]
    pub fn add_spell(&mut self, spell: Option<NonNull<SpellItem>>) {}

    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 36678, ae_id = 37686)]
    pub fn add_to_faction(&mut self, faction: Option<NonNull<TESFaction>>, rank: u8) {}

    #[inline]
    pub fn get_equipped_entry_data(&self, left_hand: bool) -> Option<NonNull<InventoryEntryData>> {
        let proc = unsafe {
            let current_process = self.get_actor_runtime_data().ok()?.currentProcess.as_ref()?;
            current_process.middleHigh.as_ref()
        }?;
        match left_hand {
            true => NonNull::new(proc.leftHand),
            false => NonNull::new(proc.rightHand),
        }
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

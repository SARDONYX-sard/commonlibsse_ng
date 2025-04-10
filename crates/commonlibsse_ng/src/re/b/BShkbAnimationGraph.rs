//! # BShkbAnimationGraph
//!
//! Represents an animation graph with ragdoll control and event handling.
//!
//! Inherits from:
//! - `BSIRagdollDriver`
//! - `BSIntrusiveRefCounted`
//! - `BSTEventSource<BSTransformDeltaEvent>`
//! - `BSTEventSource<BSAnimationGraphEvent>`
//!
//! # Memory Layout:
//! - `character_instance`: The character instance (0xC0)
//! - `bone_nodes`: Array of bone node entries (0x160)
//! - `fade_controllers`: Array of float controllers (0x178)
//! - `project_name`: Name of the project (0x1F0)
//! - `physics_world`: Physics world reference (0x238)

use crate::re::Actor::Actor;
use crate::re::BSAnimationGraphEvent::BSAnimationGraphEvent;
use crate::re::BSFixedString::BSFixedString;
use crate::re::BSIRagdollDriver::BSIRagdollDriver;
use crate::re::BSIntrusiveRefCounted::BSIntrusiveRefCounted;
use crate::re::BSIntrusiveRefCounted::BSIntrusiveRefCountedTrait;
use crate::re::BSResource::ID;
use crate::re::BSTArray::BSTArray;
use crate::re::BSTEvent::BSTEventSource;
use crate::re::BSTransformDeltaEvent;
use crate::re::BShkFloatController::BShkFloatController;
use crate::re::bhkWorld::bhkWorld;
use crate::re::hkbBehaviorGraph::hkbBehaviorGraph;
use crate::re::hkbCharacter::hkbCharacter;
use crate::re::hkbGeneratorOutput::hkbGeneratorOutput;
use crate::re::{BSFadeNode, NiNode};

#[repr(C)]
#[derive(Debug)]
pub struct BoneNodeEntry {
    /// Pointer to the node
    pub node: *mut NiNode, // 0x00

    /// Unknown field
    pub unk08: u32, // 0x08

    /// Unknown field
    pub unk0C: u32, // 0x0C
}

#[repr(C)]
#[derive(Debug)]
pub struct BShkbAnimationGraph {
    /// Base classes
    pub __base: BSIRagdollDriver, // 0x000
    pub __base1: BSIntrusiveRefCounted,                 // 0x008
    pub __base2: BSTEventSource<BSTransformDeltaEvent>, // 0x010
    pub __base3: BSTEventSource<BSAnimationGraphEvent>, // 0x068

    /// Members
    pub character_instance: hkbCharacter, // 0x0C0
    pub bone_nodes: BSTArray<BoneNodeEntry>, // 0x160
    pub fade_controllers: BSTArray<*mut BShkFloatController>, // 0x178
    pub unk190: BSTArray<*mut core::ffi::c_void>, // 0x190
    pub unk1A8: BSTArray<*mut core::ffi::c_void>, // 0x1A8
    pub unk1C0: BSTArray<u8>,                // 0x1C0
    pub unk1D8: u64,                         // 0x1D8
    pub unk1E0: u64,                         // 0x1E0
    pub interpolation_time_offsets: [f32; 2], // 0x1E8
    pub project_name: BSFixedString,         // 0x1F0
    pub project_db_data: *mut ID,            // 0x1F8
    pub project_data: *mut core::ffi::c_void, // 0x200
    pub behavior_graph: *mut hkbBehaviorGraph, // 0x208
    pub holder: *mut Actor,                  // 0x210
    pub root_node: *mut BSFadeNode,          // 0x218
    pub generator_outputs: [*mut hkbGeneratorOutput; 2], // 0x220
    pub interpolation_amounts: [f32; 2],     // 0x230
    pub physics_world: *mut bhkWorld,        // 0x238
    pub num_anim_bones: u16,                 // 0x240
    pub unk242: u8,                          // 0x242
    pub unk243: u8,                          // 0x243
    pub unk244: u16,                         // 0x244
    pub unk246: u8,                          // 0x246
    pub unk247: u8,                          // 0x247
    pub unk248: u8,                          // 0x248
    pub do_foot_ik: u8,                      // 0x249
    pub unk24A: u16,                         // 0x24A
    pub unk24C: u32,                         // 0x24C
}

const _: () = {
    assert!(core::mem::size_of::<BShkbAnimationGraph>() == 0x250);
};

impl BSIntrusiveRefCountedTrait for BShkbAnimationGraph {
    #[inline]
    fn inc_ref(&self) -> u32 {
        self.__base1.inc_ref()
    }

    #[inline]
    fn dec_ref(&self) -> u32 {
        self.__base1.dec_ref()
    }
}

impl BShkbAnimationGraph {
    /// Gets a boolean graph variable.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 62696, ae_id = 63613)]
    pub fn get_graph_variable_bool(&self, variable_name: &BSFixedString, out: &mut bool) -> bool {}

    /// Gets a float graph variable.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 62695, ae_id = 63614)]
    pub fn get_graph_variable_float(&self, variable_name: &BSFixedString, out: &mut f32) -> bool {}

    /// Gets an integer graph variable.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 62694, ae_id = 63615)]
    pub fn get_graph_variable_int(&self, variable_name: &BSFixedString, out: &mut i32) -> bool {}

    /// Sets a boolean graph variable.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 63609, ae_id = 62708)]
    pub fn set_graph_variable_bool(&mut self, variable_name: &BSFixedString, value: bool) -> bool {}

    /// Sets a float graph variable.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 63608, ae_id = 62709)]
    pub fn set_graph_variable_float(&mut self, variable_name: &BSFixedString, value: f32) -> bool {}

    /// Sets an integer graph variable.
    #[commonlibsse_ng_derive_internal::relocate_fn(se_id = 63607, ae_id = 62710)]
    pub fn set_graph_variable_int(&mut self, variable_name: &BSFixedString, value: i32) -> bool {}
}

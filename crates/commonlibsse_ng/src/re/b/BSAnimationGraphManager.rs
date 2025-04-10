//! # BSAnimationGraph
use crate::re::BSAnimationCache;
use crate::re::BSAnimationGraphChannel::BSAnimationGraphChannel;
use crate::re::BSAnimationGraphEvent::BSAnimationGraphEvent;
use crate::re::BSAnimationGraphManagerPtr;
use crate::re::BSAtomic::BSSpinLock;
use crate::re::BSFixedString::BSFixedString;
use crate::re::BSIntrusiveRefCounted::BSIntrusiveRefCounted;
use crate::re::BSIntrusiveRefCounted::BSIntrusiveRefCountedTrait;
use crate::re::BSTArray::BSTArray;
use crate::re::BSTEvent::BSTEventSink;
use crate::re::BSTSmartPointer::BSTSmartPointer;
use crate::re::BShkbAnimationGraph::BShkbAnimationGraph;
use crate::re::offsets_rtti::RTTI_BSAnimationGraphManager;
use crate::re::offsets_vtable::VTABLE_BSAnimationGraphManager;
use crate::rel::id::VariantID;
use crate::rel::module::ModuleState;
use crate::rel::relocation::{
    PhantomMember, RelocationError, relocate_member_if_newer, relocate_member_mut,
};
use crate::skse::version::RUNTIME_SSE_1_6_629;

#[repr(C)]
pub struct BSAnimationGraphManager {
    pub __base: BSTEventSink<BSAnimationGraphEvent>,
    pub __base1: BSIntrusiveRefCounted,
    pub pad0C: u32,
    pub boundChannels: BSTArray<BSTSmartPointer<BSAnimationGraphChannel>>,
    pub bumpedChannels: BSTArray<BSTSmartPointer<BSAnimationGraphChannel>>,
    pub graphs: BSTArray<BSTSmartPointer<BShkbAnimationGraph>>,
    pub subManagers: BSTArray<BSAnimationGraphManagerPtr>,
    pub variableCache: BSAnimationCache,

    //////////////////////////////////////////////// ///////////////////////////////////////////////////////////////////
    // runtime data
    //
    /// Lock for updating data.
    /// Offset: `0x98` (SE), `0xA0` (AE)
    pub updateLock: PhantomMember<BSSpinLock, 0x98, 0xa0>,

    /// Lock for dependent manager.
    /// Offset: `0xA0`
    pub dependentManagerLock: PhantomMember<BSSpinLock, 0xa0, 0xa8>,

    /// Active graph index.
    /// Offset: `0xA8`
    pub activeGraph: PhantomMember<u32, 0xa8, 0xb0>,

    /// Graph generation depth.
    /// Offset: `0xAC`
    pub generateDepth: PhantomMember<u32, 0xac, 0xb4>,
}
const _: () = {
    assert!(core::mem::offset_of!(BSAnimationGraphManager, __base) == 0x0);
    assert!(core::mem::offset_of!(BSAnimationGraphManager, __base1) == 0x8);
    assert!(core::mem::offset_of!(BSAnimationGraphManager, pad0C) == 0x0C);
    assert!(core::mem::offset_of!(BSAnimationGraphManager, boundChannels) == 0x10);
    assert!(core::mem::offset_of!(BSAnimationGraphManager, bumpedChannels) == 0x28);
    assert!(core::mem::offset_of!(BSAnimationGraphManager, graphs) == 0x40);
    assert!(core::mem::offset_of!(BSAnimationGraphManager, subManagers) == 0x58);
    assert!(core::mem::offset_of!(BSAnimationGraphManager, variableCache) == 0x70);
    assert!(core::mem::size_of::<BSAnimationGraphManager>() == 0x70);
};

impl BSAnimationGraphManager {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_BSAnimationGraphManager;

    /// Address & offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_BSAnimationGraphManager;
}

impl BSIntrusiveRefCountedTrait for BSAnimationGraphManager {
    #[inline]
    fn inc_ref(&self) -> u32 {
        self.__base1.inc_ref()
    }

    #[inline]
    fn dec_ref(&self) -> u32 {
        self.__base1.dec_ref()
    }
}

#[repr(C)]
// #[derive(Debug, Clone, Copy, PartialEq)]
pub union HkbVariableValue {
    pub b: bool,
    pub i: i32,
    pub f: f32,
}

const _: () = {
    assert!(std::mem::size_of::<HkbVariableValue>() == 0x4);
};

#[repr(C)]
#[derive(Debug)]
pub struct AnimVariableCacheInfo {
    /// Variable name.
    /// Offset: `0x00`
    pub variable_name: BSFixedString,

    /// Pointer to the variable value.
    /// Offset: `0x08`
    pub variable: *mut HkbVariableValue,
}

const _: () = {
    assert!(std::mem::size_of::<AnimVariableCacheInfo>() == 0x10);
};

#[repr(C)]
#[derive(Debug)]
pub struct BSAnimationGraphVariableCache {
    /// Cache of animation variables.
    /// Offset: `0x00`
    pub variable_cache: BSTArray<AnimVariableCacheInfo>,

    /// Lock for updating cache.
    /// Offset: `0x18`
    pub update_lock: BSSpinLock,

    /// Smart pointer to the animation graph (SE only).
    /// Offset: `0x20` (SE), `0x28` (AE)
    pub animation_graph: BSTSmartPointer<BShkbAnimationGraph>,
}

const _: () = {
    assert!(std::mem::size_of::<BSAnimationGraphVariableCache>() == 0x28);
};

impl BSAnimationGraphVariableCache {
    /// Gets the graph lock (Skyrim AE only).
    #[inline]
    pub fn get_graph_lock(&mut self) -> Option<&mut BSSpinLock> {
        let is_ae_1_6_629 =
            ModuleState::map_or_init(|module| module.version >= RUNTIME_SSE_1_6_629).ok()?;
        if is_ae_1_6_629 {
            return relocate_member_mut(self, 0x20, 0x20).ok();
        }

        None
    }

    /// Gets the animation graph (SE only).
    ///
    /// # Errors
    #[inline]
    pub fn get_animation_graph(
        &self,
    ) -> Result<&BSTSmartPointer<BShkbAnimationGraph>, RelocationError> {
        unsafe { relocate_member_if_newer(RUNTIME_SSE_1_6_629, self, 0x20, 0x28) }
    }
}

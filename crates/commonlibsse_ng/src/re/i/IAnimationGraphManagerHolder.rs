//! # IAnimationGraphManagerHolder
//!
//! Represents the interface for managing animation graphs.
//!
//! Provides virtual functions for interacting with animation graphs and related operations.
//!
//! # Memory Layout:
//! - `VTABLE`: Virtual function table pointer (0x0)

use crate::re::BSAnimationGraphChannel::BSAnimationGraphChannel;
use crate::re::BSAnimationGraphManager::BSAnimationGraphManager;
use crate::re::BSFixedString::BSFixedString;
use crate::re::BSTArray::BSScrapArray;
use crate::re::BSTSmartPointer::BSTSmartPointer;
use crate::re::BShkbAnimationGraph::BShkbAnimationGraph;
use crate::re::NiAVObject::NiAVObject;
use crate::re::offsets_rtti::RTTI_IAnimationGraphManagerHolder;
use crate::re::offsets_vtable::VTABLE_IAnimationGraphManagerHolder;
use crate::rel::id::VariantID;

#[repr(C)]
#[derive(Debug)]
pub struct IAnimationGraphManagerHolder {
    /// Virtual function table pointer.
    /// Offset: `0x0`
    pub vtable: *const IAnimationGraphManagerHolderVtbl,
}

const _: () = {
    assert!(core::mem::size_of::<IAnimationGraphManagerHolder>() == 0x8);
};

impl IAnimationGraphManagerHolder {
    /// Address & offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_IAnimationGraphManagerHolder;

    /// Address & offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_IAnimationGraphManagerHolder;
}

/// The virtual function table for `IAnimationGraphManagerHolder`.
///
/// This struct defines function pointers to simulate the C++ virtual functions.
#[repr(C)]
#[derive(Debug)]
pub struct IAnimationGraphManagerHolderVtbl {
    /// Destructor.
    pub CxxDrop: fn(this: &mut IAnimationGraphManagerHolder),

    /// Virtual functions corresponding to the C++ methods.
    pub NotifyAnimationGraph:
        fn(this: &IAnimationGraphManagerHolder, event_name: &BSFixedString) -> bool,
    pub GetAnimationGraphManagerImpl: fn(
        this: &IAnimationGraphManagerHolder,
        out: &mut BSTSmartPointer<BSAnimationGraphManager>,
    ) -> bool,
    pub SetAnimationGraphManagerImpl: fn(
        this: &mut IAnimationGraphManagerHolder,
        in_mgr: BSTSmartPointer<BSAnimationGraphManager>,
    ) -> bool,
    pub PopulateGraphNodesToTarget:
        fn(this: &IAnimationGraphManagerHolder, nodes: &mut BSScrapArray<*mut NiAVObject>) -> bool,
    pub ConstructAnimationGraph: fn(
        this: &IAnimationGraphManagerHolder,
        out: &mut BSTSmartPointer<BShkbAnimationGraph>,
    ) -> bool,
    pub Unk_06: fn(this: &IAnimationGraphManagerHolder),
    pub Unk_07: fn(this: &IAnimationGraphManagerHolder),
    pub SetupAnimEventSinks: fn(
        this: &IAnimationGraphManagerHolder,
        anim_graph: &BSTSmartPointer<BShkbAnimationGraph>,
    ) -> bool,
    pub Unk_09: fn(this: &IAnimationGraphManagerHolder),
    pub CreateAnimationChannels: fn(
        this: &IAnimationGraphManagerHolder,
        channels: &mut BSScrapArray<BSTSmartPointer<BSAnimationGraphChannel>>,
    ) -> bool,
    pub PostCreateAnimationGraphManager: fn(
        this: &IAnimationGraphManagerHolder,
        anim_graph_mgr: &mut BSTSmartPointer<BSAnimationGraphManager>,
    ),
    pub Unk_0C: fn(this: &IAnimationGraphManagerHolder),
    pub PostChangeAnimationManager: fn(
        this: &IAnimationGraphManagerHolder,
        arg1: &BSTSmartPointer<BShkbAnimationGraph>,
        arg2: &BSTSmartPointer<BShkbAnimationGraph>,
    ),
    pub Unk_0E: fn(this: &IAnimationGraphManagerHolder),
    pub GetGraphVariableCacheSize: fn(this: &IAnimationGraphManagerHolder) -> u32,
    pub GetGraphVariableImpl1:
        fn(this: &IAnimationGraphManagerHolder, name: &BSFixedString, out: &mut f32) -> bool,
    pub GetGraphVariableImpl2:
        fn(this: &IAnimationGraphManagerHolder, name: &BSFixedString, out: &mut i32) -> bool,
    pub GetGraphVariableImpl3:
        fn(this: &IAnimationGraphManagerHolder, name: &BSFixedString, out: &mut bool) -> bool,
}

impl Default for IAnimationGraphManagerHolderVtbl {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl IAnimationGraphManagerHolderVtbl {
    /// Creates a new default virtual table with stubbed functions.
    pub const fn new() -> Self {
        const fn CxxDrop(_this: &mut IAnimationGraphManagerHolder) {}

        const fn NotifyAnimationGraph(
            _this: &IAnimationGraphManagerHolder,
            _event_name: &BSFixedString,
        ) -> bool {
            false
        }

        const fn GetAnimationGraphManagerImpl(
            _this: &IAnimationGraphManagerHolder,
            _out: &mut BSTSmartPointer<BSAnimationGraphManager>,
        ) -> bool {
            false
        }

        fn SetAnimationGraphManagerImpl(
            _this: &mut IAnimationGraphManagerHolder,
            _in_mgr: BSTSmartPointer<BSAnimationGraphManager>,
        ) -> bool {
            false
        }

        const fn PopulateGraphNodesToTarget(
            _this: &IAnimationGraphManagerHolder,
            _nodes: &mut BSScrapArray<*mut NiAVObject>,
        ) -> bool {
            false
        }

        const fn ConstructAnimationGraph(
            _this: &IAnimationGraphManagerHolder,
            _out: &mut BSTSmartPointer<BShkbAnimationGraph>,
        ) -> bool {
            false
        }

        const fn Unk_06(_this: &IAnimationGraphManagerHolder) {}

        const fn Unk_07(_this: &IAnimationGraphManagerHolder) {}

        const fn SetupAnimEventSinks(
            _this: &IAnimationGraphManagerHolder,
            _anim_graph: &BSTSmartPointer<BShkbAnimationGraph>,
        ) -> bool {
            true
        }

        const fn Unk_09(_this: &IAnimationGraphManagerHolder) {}

        const fn CreateAnimationChannels(
            _this: &IAnimationGraphManagerHolder,
            _channels: &mut BSScrapArray<BSTSmartPointer<BSAnimationGraphChannel>>,
        ) -> bool {
            false
        }

        const fn PostCreateAnimationGraphManager(
            _this: &IAnimationGraphManagerHolder,
            _anim_graph_mgr: &mut BSTSmartPointer<BSAnimationGraphManager>,
        ) {
        }

        const fn Unk_0C(_this: &IAnimationGraphManagerHolder) {}

        const fn PostChangeAnimationManager(
            _this: &IAnimationGraphManagerHolder,
            _arg1: &BSTSmartPointer<BShkbAnimationGraph>,
            _arg2: &BSTSmartPointer<BShkbAnimationGraph>,
        ) {
        }

        const fn Unk_0E(_this: &IAnimationGraphManagerHolder) {}

        const fn GetGraphVariableCacheSize(_this: &IAnimationGraphManagerHolder) -> u32 {
            0
        }

        const fn GetGraphVariableImpl1(
            _this: &IAnimationGraphManagerHolder,
            _name: &BSFixedString,
            _out: &mut f32,
        ) -> bool {
            false
        }

        const fn GetGraphVariableImpl2(
            _this: &IAnimationGraphManagerHolder,
            _name: &BSFixedString,
            _out: &mut i32,
        ) -> bool {
            false
        }

        const fn GetGraphVariableImpl3(
            _this: &IAnimationGraphManagerHolder,
            _name: &BSFixedString,
            _out: &mut bool,
        ) -> bool {
            false
        }

        Self {
            CxxDrop,
            NotifyAnimationGraph,
            GetAnimationGraphManagerImpl,
            SetAnimationGraphManagerImpl,
            PopulateGraphNodesToTarget,
            ConstructAnimationGraph,
            Unk_06,
            Unk_07,
            SetupAnimEventSinks,
            Unk_09,
            CreateAnimationChannels,
            PostCreateAnimationGraphManager,
            Unk_0C,
            PostChangeAnimationManager,
            Unk_0E,
            GetGraphVariableCacheSize,
            GetGraphVariableImpl1,
            GetGraphVariableImpl2,
            GetGraphVariableImpl3,
        }
    }
}

//! # hkbCharacterSetup
//!
//! This module defines the `hkbCharacterSetup` struct, which inherits from `hkReferencedObject` and represents
//! the character setup in the game's engine. It includes a virtual table for C++ compatibility
//! and maintains the original memory layout.

use crate::re::hkArray::hkArray;
use crate::re::hkCriticalSection;
use crate::re::hkRefPtr::hkRefPtr;
use crate::re::hkReferencedObject::hkReferencedObject;
use crate::re::hkaMirroredSkeleton;
use crate::re::hkaSkeleton;
use crate::re::hkaSkeletonMapper;
use crate::re::hkbAnimationBindingSet;
use crate::re::hkbCharacterData;
use crate::re::hkbSymbolIdMap;
use crate::re::offsets_rtti::RTTI_hkbCharacterSetup;
use crate::re::offsets_vtable::VTABLE_hkbCharacterSetup;
use crate::rel::id::VariantID;

#[repr(C)]
pub struct hkbCharacterSetup {
    /// Base class `hkReferencedObject`.
    pub __base: hkReferencedObject,

    /// The array of retargeting skeleton mappers.
    /// - Offset: `0x10`
    pub retargetingSkeletonMappers: hkArray<hkRefPtr<hkaSkeletonMapper>>,

    /// The reference to the animation skeleton.
    /// - Offset: `0x20`
    pub animationSkeleton: hkRefPtr<hkaSkeleton>,

    /// The reference to the ragdoll-to-animation skeleton mapper.
    /// - Offset: `0x28`
    pub ragdollToAnimationSkeletonMapper: hkRefPtr<hkaSkeletonMapper>,

    /// The reference to the animation-to-ragdoll skeleton mapper.
    /// - Offset: `0x30`
    pub animationToRagdollSkeletonMapper: hkRefPtr<hkaSkeletonMapper>,

    /// The animation binding set.
    /// - Offset: `0x38`
    pub animationBindingSet: hkRefPtr<hkbAnimationBindingSet>,

    /// The character data.
    /// - Offset: `0x40`
    pub data: hkRefPtr<hkbCharacterData>,

    /// The unscaled animation skeleton.
    /// - Offset: `0x48`
    pub unscaledAnimationSkeleton: hkRefPtr<hkaSkeleton>,

    /// The mirrored skeleton.
    /// - Offset: `0x50`
    pub mirroredSkeleton: hkRefPtr<hkaMirroredSkeleton>,

    /// The character property ID map.
    /// - Offset: `0x58`
    pub characterPropertyIdMap: hkRefPtr<hkbSymbolIdMap>,

    /// The critical section.
    /// - Offset: `0x60`
    pub criticalSection: hkCriticalSection,

    // Padding and alignment.
    pub _pad: [u8; 8],
}

impl crate::re::hkRefPtr::hkRefPtrCounted for hkbCharacterSetup {}

const _: () = {
    assert!(core::mem::offset_of!(hkbCharacterSetup, __base) == 0x0);
    assert!(core::mem::offset_of!(hkbCharacterSetup, retargetingSkeletonMappers) == 0x10);
    assert!(core::mem::offset_of!(hkbCharacterSetup, animationSkeleton) == 0x20);
    assert!(core::mem::offset_of!(hkbCharacterSetup, ragdollToAnimationSkeletonMapper) == 0x28);
    assert!(core::mem::offset_of!(hkbCharacterSetup, animationToRagdollSkeletonMapper) == 0x30);
    assert!(core::mem::offset_of!(hkbCharacterSetup, animationBindingSet) == 0x38);
    assert!(core::mem::offset_of!(hkbCharacterSetup, data) == 0x40);
    assert!(core::mem::offset_of!(hkbCharacterSetup, unscaledAnimationSkeleton) == 0x48);
    assert!(core::mem::offset_of!(hkbCharacterSetup, mirroredSkeleton) == 0x50);
    assert!(core::mem::offset_of!(hkbCharacterSetup, characterPropertyIdMap) == 0x58);
    assert!(core::mem::offset_of!(hkbCharacterSetup, criticalSection) == 0x60);
    assert!(core::mem::size_of::<hkbCharacterSetup>() == 0x68);
};

impl Default for hkbCharacterSetup {
    fn default() -> Self {
        Self::new()
    }
}

impl hkbCharacterSetup {
    /// Address & Offset of the runtime type information (RTTI) identifier.
    pub const RTTI: VariantID = RTTI_hkbCharacterSetup;

    /// Address & Offset of the virtual function table.
    pub const VTABLE: [VariantID; 1] = VTABLE_hkbCharacterSetup;

    /// Creates a new `hkbCharacterSetup` instance with default values.
    ///
    /// - `__base`: Default `hkReferencedObject`
    /// - `retargetingSkeletonMappers`: Empty `hkArray`
    /// - `animationSkeleton`: Default `hkRefPtr`
    /// - Other fields initialized to their default values.
    #[inline]
    pub fn new() -> Self {
        Self {
            __base: hkReferencedObject::new(),
            retargetingSkeletonMappers: hkArray::new(),
            animationSkeleton: hkRefPtr::default(),
            ragdollToAnimationSkeletonMapper: hkRefPtr::default(),
            animationToRagdollSkeletonMapper: hkRefPtr::default(),
            animationBindingSet: hkRefPtr::default(),
            data: hkRefPtr::default(),
            unscaledAnimationSkeleton: hkRefPtr::default(),
            mirroredSkeleton: hkRefPtr::default(),
            characterPropertyIdMap: hkRefPtr::default(),
            criticalSection: hkCriticalSection,
            _pad: [0; 8], // Padding to align structure
        }
    }
}

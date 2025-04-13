/// NOTE: Entire enum needs more REing
#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
#[non_exhaustive]
pub enum INPUT_CONTEXT_ID_SE {
    Gameplay = 0,
    MenuMode,
    Console,
    ItemMenu,
    Inventory,
    DebugText,
    Favorites,
    Map,
    Stats,
    Cursor,
    Book,
    DebugOverlay,
    Journal,
    TFCMode,
    MapDebug,
    Lockpicking,
    Favor,
}
const _: () = assert!(INPUT_CONTEXT_ID_SE_CEnum::count() == 17);

/// NOTE: Entire enum needs more REing
#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
#[non_exhaustive]
pub enum INPUT_CONTEXT_ID_AE {
    Gameplay = 0,
    MenuMode,
    Console,
    ItemMenu,
    Inventory,
    DebugText,
    Favorites,
    Map,
    Stats,
    Cursor,
    Book,
    DebugOverlay,
    Journal,
    TFCMode,
    MapDebug,
    Lockpicking,
    Marketplace, // <- AE only
    Favor,
}
const _: () = assert!(INPUT_CONTEXT_ID_AE_CEnum::count() == 18);

/// NOTE: Entire enum needs more REing
#[commonlibsse_ng_derive_internal::ffi_enum]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
#[non_exhaustive]
pub enum INPUT_CONTEXT_ID_VR {
    Gameplay = 0,
    MenuMode,
    Console,
    ItemMenu,
    Inventory,
    DebugText,
    Favorites,
    Map,
    Stats,
    Cursor,
    Book,
    DebugOverlay,
    Journal,
    TFCMode,
    MapDebug,
    Lockpicking,
    Favor,
    // None = 22, // More input contexts might be available, needs REing
}
const _: () = assert!(INPUT_CONTEXT_ID_VR_CEnum::count() == 17);

/// `INPUT_CONTEXT_ID` to store the raw value.
///
/// This is a generic type that exists because the correct value varies depending on the SE, AE , VR environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct INPUT_CONTEXT_ID(pub u32);
const _: () = assert!(core::mem::size_of::<INPUT_CONTEXT_ID>() == 0x4);

impl INPUT_CONTEXT_ID {
    /// Try to cast valid enum for SE, AE.
    #[inline]
    pub const fn as_se(&self) -> Option<INPUT_CONTEXT_ID_SE> {
        INPUT_CONTEXT_ID_SE_CEnum(self.0).to_enum()
    }

    /// Try to cast valid enum for AE.
    #[inline]
    pub const fn as_ae(&self) -> Option<INPUT_CONTEXT_ID_AE> {
        INPUT_CONTEXT_ID_AE_CEnum(self.0).to_enum()
    }

    /// Try to cast valid enum for VR.
    #[inline]
    pub const fn as_vr(&self) -> Option<INPUT_CONTEXT_ID_VR> {
        INPUT_CONTEXT_ID_VR_CEnum(self.0).to_enum()
    }
}

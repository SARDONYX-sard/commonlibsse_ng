#[derive(Debug, darling::FromMeta)]
pub(crate) struct MacroArgs {
    pub cast_as: String,
    pub default: String,
    #[darling(default)]
    pub deref_once: Option<bool>,
    pub id: RelocationId,
}

#[derive(Debug, darling::FromMeta)]
pub(crate) struct RelocationId {
    /// Skyrim Special Edition ID
    pub se: u64,
    /// Skyrim Anniversary Edition ID
    pub ae: u64,
    /// Skyrim VR ID
    pub vr: Option<u64>,
}

#[derive(Debug, darling::FromMeta)]
pub(crate) struct MacroArgs {
    /// Skyrim Special Edition ID
    pub se_id: u64,
    /// Skyrim Anniversary Edition ID
    pub ae_id: u64,
    /// Skyrim VR ID
    pub vr_id: Option<u64>,
}

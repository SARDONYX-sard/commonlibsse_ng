#[derive(Debug, darling::FromMeta)]
pub(crate) struct MacroArgs {
    /// Custom bitflags name
    pub flag_name: Option<String>,
}

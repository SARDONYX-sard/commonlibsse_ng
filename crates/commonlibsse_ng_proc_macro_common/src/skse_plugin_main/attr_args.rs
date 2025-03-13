#[derive(Debug, darling::FromMeta)]
pub(crate) struct MacroArgs {
    pub plugin_name: Option<String>,
    pub plugin_author: Option<String>,
    pub plugin_version: Option<String>,

    #[cfg(feature = "tracing")]
    #[darling(default = "ret_true")]
    pub logger: bool,
    #[cfg(feature = "tracing")]
    #[darling(default)]
    pub log_level: super::logger::LogLevel,
}

#[cfg(feature = "tracing")]
fn ret_true() -> bool {
    true
}

use quote::quote;

#[derive(Debug, Default, darling::FromMeta, PartialEq, Eq)]
pub(crate) enum LogLevel {
    #[default]
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

pub struct GeneratedCode {
    pub init_logger: proc_macro2::TokenStream,
    pub is_editor_log: proc_macro2::TokenStream,
}

impl Default for GeneratedCode {
    fn default() -> Self {
        Self { init_logger: quote! {}, is_editor_log: quote! {} }
    }
}

pub(crate) fn gen_logger_code(
    enable_logger: bool,
    plugin_log_name: &str,
    log_level: LogLevel,
) -> GeneratedCode {
    let mut code = GeneratedCode::default();

    if enable_logger {
        let log_level = log_level.as_str();
        let err_title = format!("{plugin_log_name} Error");

        code.init_logger = quote! {
            if let Err(err) = commonlibsse_ng::skse::logger::init_with_log_dir(#plugin_log_name, #log_level.parse().unwrap()) {
                commonlibsse_ng::rex::win32::message_box(#err_title, &err.to_string());
                std::process::exit(1);
            };
            tracing::info!("Logger has been initialized.");
        };

        code.is_editor_log = quote! {
            tracing::error!("The use of the SKSE Plugin within Editor is not supported.");
        };
    };

    code
}

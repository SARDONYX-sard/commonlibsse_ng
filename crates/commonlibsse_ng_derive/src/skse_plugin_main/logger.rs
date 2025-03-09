use quote::quote;

#[cfg(feature = "tracing")]
#[derive(Debug, Default, darling::FromMeta, PartialEq, Eq)]
pub(crate) enum LogLevel {
    #[default]
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[cfg(feature = "tracing")]
impl LogLevel {
    fn token(&self) -> proc_macro2::TokenStream {
        let level = match self {
            LogLevel::Trace => quote! { TRACE },
            LogLevel::Debug => quote! { DEBUG },
            LogLevel::Info => quote! { INFO },
            LogLevel::Warn => quote! { WARN },
            LogLevel::Error => quote! { ERROR },
        };

        quote! { commonlibsse_ng::__private::tracing::level_filters::LevelFilter::#level }
    }
}

pub struct LoggerTokenStream {
    pub init_logger: proc_macro2::TokenStream,
    pub is_editor_log: proc_macro2::TokenStream,
}

impl Default for LoggerTokenStream {
    fn default() -> Self {
        Self { init_logger: quote! {}, is_editor_log: quote! {} }
    }
}

#[cfg(feature = "tracing")]
pub(crate) fn gen_logger_code(
    enable_logger: bool,
    plugin_name: Option<&str>,
    log_level: LogLevel,
) -> LoggerTokenStream {
    let mut code = LoggerTokenStream::default();

    if enable_logger {
        let plugin_log_name = if let Some(plugin_name) = plugin_name {
            quote! { concat!(#plugin_name, ".log") }
        } else {
            quote! { concat!(env!("CARGO_PKG_NAME"), ".log") }
        };
        let err_title = if let Some(plugin_name) = plugin_name {
            let title = format!("{plugin_name} Error");
            quote! { #title }
        } else {
            quote! {
                concat!(env!("CARGO_PKG_NAME"), " Error")
            }
        };
        let log_level = log_level.token();

        code.init_logger = quote! {
            if let Err(err) = commonlibsse_ng::skse::logger::init_with_log_dir(#plugin_log_name, #log_level) {
                commonlibsse_ng::rex::win32::message_box(#err_title, &err.to_string());
                std::process::exit(1);
            };
            commonlibsse_ng::__private::tracing::info!("Logger has been initialized.");
        };

        code.is_editor_log = quote! {
            commonlibsse_ng::__private::tracing::error!("The use of the SKSE Plugin within Editor is not supported.");
        };
    };

    code
}

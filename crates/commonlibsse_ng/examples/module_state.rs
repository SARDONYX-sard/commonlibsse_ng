use commonlibsse_ng::skse_plugin_main;

#[cfg_attr(feature = "tracing", skse_plugin_main(plugin_name = "module_state"))]
#[cfg_attr(
    not(feature = "tracing"),
    skse_plugin_main(plugin_name = "module_state", logger = false)
)]
// #[skse_plugin_main] // To avoid lsp compilation bug
fn plugin_main() {
    match commonlibsse_ng::skse::api::get_messaging_interface() {
        Ok(messaging) => {
            if let Err(err) = messaging.register_skse_listener(|message| {
                #[cfg(feature = "tracing")]
                tracing::info!("SKSE event: {message:#?}");
            }) {
                #[cfg(feature = "tracing")]
                tracing::error!("{err}");
            };
        }
        Err(err) => {
            #[cfg(feature = "tracing")]
            tracing::error!("Failed to skse::init: {err}");
        }
    }
}

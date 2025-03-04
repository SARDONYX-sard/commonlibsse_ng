use commonlibsse_ng::skse_plugin_main;

#[cfg_attr(feature = "tracing", skse_plugin_main(plugin_name = "module_state"))]
#[cfg_attr(
    not(feature = "tracing"),
    skse_plugin_main(plugin_name = "module_state", logger = false)
)]
fn plugin_main() {
    if let Some(messaging) = commonlibsse_ng::skse::api::get_messaging_interface() {
        messaging.register_listener(|message| {
            #[cfg(feature = "tracing")]
            tracing::info!("SKSE event: {message:?}");
        });
    }
}

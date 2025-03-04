// #[commonlibsse_ng::skse_plugin_main]
#[cfg_attr(feature = "tracing", commonlibsse_ng::skse_plugin_main)]
#[cfg_attr(not(feature = "tracing"), commonlibsse_ng::skse_plugin_main(logger = false))]
fn plugin_main() {
    if let Some(messaging) = commonlibsse_ng::skse::api::get_messaging_interface() {
        messaging.register_listener(|message| {
            #[cfg(feature = "tracing")]
            tracing::info!("SKSE event: {message:?}");
        });
    }
}

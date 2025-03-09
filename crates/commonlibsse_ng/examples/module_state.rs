#[commonlibsse_ng::skse_plugin_main]
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

#[commonlibsse_ng::relocate_fn(se_id = "100", ae_id = "100")]
fn init_extra_data_list(enable: bool) -> bool {}

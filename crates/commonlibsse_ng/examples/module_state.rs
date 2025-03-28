use commonlibsse_ng::skse::interfaces::messaging::MessageType;

#[commonlibsse_ng::skse_plugin_main]
fn plugin_main() {
    match commonlibsse_ng::skse::api::get_messaging_interface() {
        Ok(messaging) => {
            if let Err(err) = messaging.register_skse_listener(|message| {
                #[cfg(feature = "tracing")]
                tracing::info!("SKSE event: {message:#?}");

                if let Some(msg_type) = message.msg_type.to_enum() {
                    if msg_type == MessageType::PostLoadGame {
                        record_game_date();
                    }
                }
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

fn record_game_date() {
    use commonlibsse_ng::re::Calendar::Calendar;

    if let Some(calendar) = unsafe { Calendar::get_singleton().as_ref() } {
        #[cfg(feature = "tracing")]
        tracing::trace!("{calendar:#?}");
        if let Some(date) = calendar.get_time() {
            #[cfg(feature = "tracing")]
            tracing::trace!("{date}");
        };
    };
}

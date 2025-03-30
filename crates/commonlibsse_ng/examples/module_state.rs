use commonlibsse_ng::skse::{
    self,
    interfaces::messaging::{Message, MessageType},
};

#[commonlibsse_ng::skse_plugin_main(plugin_name = "module_state")]
fn plugin_main() {
    match skse::api::get_messaging_interface() {
        Ok(messaging) => {
            if let Err(err) = messaging.register_skse_listener(skse_event_listener) {
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

fn skse_event_listener(message: &Message) {
    #[cfg(feature = "tracing")]
    tracing::trace!("SKSE event: {message:#?}");

    if let Some(msg_type) = message.msg_type.to_enum() {
        if msg_type == MessageType::PostLoadGame {
            record_game_date();
        }
    }
}

fn record_game_date() {
    use commonlibsse_ng::re::Calendar::Calendar;
    if let Some(calendar) = Calendar::get_singleton() {
        #[cfg(feature = "tracing")]
        tracing::trace!("{calendar:#?}");
        if let Some(_date) = calendar.get_time() {
            #[cfg(feature = "tracing")]
            tracing::trace!("{_date}");
        };
    };
}

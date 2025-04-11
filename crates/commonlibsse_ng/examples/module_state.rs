use commonlibsse_ng::skse;
use commonlibsse_ng::skse::interfaces::messaging::{Message, MessageType};

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
    if let Some(msg_type) = message.msg_type.to_enum() {
        if msg_type == MessageType::PostLoadGame {
            record_game_date();
            record_player_character();
            record_game_ini();
        }
    }
}

#[allow(unused)]
fn record_game_date() {
    use commonlibsse_ng::re::Calendar::Calendar;

    if let Some(calendar) = Calendar::get_singleton() {
        #[cfg(feature = "tracing")]
        tracing::trace!("{calendar:#?}");
        if let Some(date) = calendar.get_time() {
            #[cfg(feature = "tracing")]
            tracing::trace!("{date}");
        };
    };
}

#[allow(unused)]
fn record_player_character() {
    use commonlibsse_ng::re::PlayerCharacter::PlayerCharacter;

    #[cfg(feature = "tracing")]
    tracing::trace!("is_god_mode = {}", PlayerCharacter::is_god_mode());

    if let Some(player) = PlayerCharacter::get_singleton() {
        #[cfg(feature = "tracing")]
        {
            tracing::trace!("player addr = {player:p}");
            commonlibsse_ng::console_println!("player addr = {player:p}");

            let is_valid_range = {
                let player_ptr = (player as *const PlayerCharacter).cast();
                const PLAYER_LEN: usize = core::mem::size_of::<PlayerCharacter>();
                commonlibsse_ng::rex::win32::is_valid_range(player_ptr, PLAYER_LEN)
            };
            tracing::trace!("player.is_valid_range() = {is_valid_range}");

            if is_valid_range {
                let refr = &player.__base.__base.__base;
                tracing::trace!("player_refr = {refr:#?}");
            }
        }
    };
}

#[allow(unused)]
fn record_game_ini() {
    use commonlibsse_ng::re::GameSettingCollection::GameSettingCollection;

    if let Some(game_setting) = GameSettingCollection::get_singleton() {
        #[cfg(feature = "tracing")]
        {
            tracing::trace!("game_setting addr = {:p}", game_setting);
            tracing::trace!("game_setting = {:#?}", game_setting.to_hashmap());
        }
    };
}

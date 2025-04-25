#![allow(unused)]
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
            record_story_teller();

            record_ui();

            // record_game_date();
            // record_player_character();
            // record_game_ini();
        }
    }
}

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

fn record_player_character() {
    use commonlibsse_ng::re::PlayerCharacter::PlayerCharacter;

    let array = commonlibsse_ng::bst_array![0, 1];

    #[cfg(feature = "tracing")]
    tracing::trace!("is_god_mode = {}", PlayerCharacter::is_god_mode());

    if let Some(player) = PlayerCharacter::get_singleton() {
        #[cfg(feature = "tracing")]
        {
            tracing::trace!("player addr = {player:p}");
            // commonlibsse_ng::console_println!("player addr = {player:p}");

            let is_accessible_struct = commonlibsse_ng::rex::win32::is_accessible_struct(player);
            tracing::trace!("player.is_accessible_struct() = {is_accessible_struct}");

            if !is_accessible_struct {
                return;
            }

            let refr = &player.__base.__base.__base;
            tracing::trace!("player_refr = {refr:#?}");
            commonlibsse_ng::debug_message_box!("player_refr = {refr:#?}");
        }
    };
}

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

fn record_ui() {
    use commonlibsse_ng::re::UI::UI;

    if let Some(ui) = UI::get_singleton() {
        #[cfg(feature = "tracing")]
        {
            let is_accessible_struct = commonlibsse_ng::rex::win32::is_accessible_struct(ui);
            tracing::trace!("player.is_accessible_struct() = {is_accessible_struct}");

            tracing::trace!("ui_addr = {:p}", ui);

            match ui.menuMap.show_memory_layout() {
                Ok(mem_layout) => {
                    tracing::trace!("ui.menuMap = {mem_layout}");
                }
                Err(err) => {
                    tracing::error!("Failed to show memory layout: {err}");
                    tracing::trace!("ui.menuMap = {:#?}", ui.menuMap);
                }
            }
            tracing::trace!("ui = {:#?}", ui);
        }
    };
}

#[allow(clippy::cognitive_complexity)]
fn record_story_teller() {
    use commonlibsse_ng::re::BGSStoryTeller::BGSStoryTeller;

    if let Some(story_teller) = BGSStoryTeller::get_singleton() {
        #[cfg(feature = "tracing")]
        {
            tracing::trace!("story_teller addr = {:p}", story_teller);
            tracing::trace!("story_teller addr = {:#?}", story_teller);

            for (index, quest_ptr) in story_teller.queuedStartQuests.iter().enumerate() {
                if let Some(quest) = unsafe { quest_ptr.as_ref() } {
                    tracing::trace!("queuedStartQuests[{index}] = {:#?}", quest);
                }
            }

            for (index, quest_ptr) in story_teller.runningQuests.iter().enumerate() {
                if let Some(quest) = unsafe { quest_ptr.as_ref() } {
                    tracing::trace!("runningQuests[{index}] = {:#?}", quest);
                }
            }

            for (index, quest_ptr) in story_teller.queuedStopQuests.iter().enumerate() {
                if let Some(quest) = unsafe { quest_ptr.as_ref() } {
                    tracing::trace!("queuedStopQuests[{index}] = {:#?}", quest);
                }
            }

            for (index, quest_ptr) in story_teller.infoClearQuests.iter().enumerate() {
                if let Some(quest) = unsafe { quest_ptr.as_ref() } {
                    tracing::trace!("infoClearQuests[{index}] = {:#?}", quest);
                }
            }

            for (index, quest_ptr) in story_teller.helloTopicQuests.iter().enumerate() {
                if let Some(quest) = unsafe { quest_ptr.as_ref() } {
                    tracing::trace!("helloTopicQuests[{index}] = {:#?}", quest);
                }
            }

            for (index, quest_ptr) in story_teller.greetingTopicQuests.iter().enumerate() {
                if let Some(quest) = unsafe { quest_ptr.as_ref() } {
                    tracing::trace!("greetingTopicQuests[{index}] = {:#?}", quest);
                }
            }
        }
    };
}

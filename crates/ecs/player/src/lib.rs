use bevy_app::prelude::*;
use bevy_ecs::change_detection::ResMut;
use bevy_ecs::event::{EventReader, EventWriter};
use common::logging;
use ecs_database::SavePlayerDataEvent;
use ecs_message::event::ClientMessageEvent;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use proto::p11::{Cs11011, Cs11017, Cs11603, Sc11012, Sc11018, Sc11604};

pub mod plugin;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(plugin::PlayerDataNotifyPlugin)
            .add_plugins(plugin::PlayerBuffNotifyPlugin)
            .add_systems(PreUpdate, update_story)
            .add_systems(PreUpdate, fetch_secondary_password)
            .add_systems(PreUpdate, change_player_icon);
    }
}

fn update_story(
    mut events: EventReader<ClientMessageEvent>,
    mut player: ResMut<Player>,
    mut message_output: ResMut<MessageOutput>,
    mut save_player_data_event: EventWriter<SavePlayerDataEvent>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs11017>() {
            logging::debug!("UpdateStory: {:?}", req);

            player.info().story_list.push(req.story_id);

            message_output.send(Sc11018::default(), message.id());

            save_player_data_event.send(SavePlayerDataEvent(player.uid()));
        }
    }
}

pub fn fetch_secondary_password(
    mut events: EventReader<ClientMessageEvent>,
    mut message_output: ResMut<MessageOutput>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs11603>() {
            logging::debug!("FetchSecondaryPassword: {:?}", req);

            message_output.send(Sc11604::default(), message.id());
        }
    }
}

fn change_player_icon(
    mut events: EventReader<ClientMessageEvent>,
    mut player: ResMut<Player>,
    mut message_output: ResMut<MessageOutput>,
    mut save_player_data_event: EventWriter<SavePlayerDataEvent>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs11011>() {
            logging::debug!("ChangePlayerIcon: {:?}", req);

            player.info().character = req.character;

            message_output.send(Sc11012::default(), message.id());

            save_player_data_event.send(SavePlayerDataEvent(player.uid()));
        }
    }
}

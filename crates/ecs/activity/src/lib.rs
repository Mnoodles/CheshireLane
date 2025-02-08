pub mod activity;
pub mod reflux;
mod mini_game;
mod limit_challenge;
pub mod event;

use bevy_app::prelude::*;

pub struct ActivityPlugin;

impl Plugin for ActivityPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(reflux::RefluxDataNotifyPlugin)
            .add_plugins(activity::notify::ActivityDataNotifyPlugin)
            .add_plugins(event::EventInfoNotifyPlugin)
            .add_plugins(activity::notify::InstagramRegisterPlugin)
            .add_plugins(activity::permanent::ActivityPermanentRegisterPlugin)
            .add_systems(PreUpdate, mini_game::mini_game_request)
            .add_systems(PreUpdate, limit_challenge::limit_challenge_request)
            .add_systems(PreUpdate, activity::instagram_chat_get_data)
            .add_systems(PreUpdate, activity::instagram_chat_active_topic);
    }
}

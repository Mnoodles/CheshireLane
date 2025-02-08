use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;

pub struct PlayerDataNotifyPlugin;

impl Plugin for PlayerDataNotifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyPlayerDataEvent>()
            .add_systems(Update, notify_player_data);
    }
}

#[derive(Event)]
pub struct NotifyPlayerDataEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_player_data(
    mut events: EventReader<NotifyPlayerDataEvent>,
    mut player: ResMut<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                player.info().notify_player_data(),
                event.1,
                event.2,
                event.3);
        }
    }
}

pub struct PlayerBuffNotifyPlugin;

impl Plugin for PlayerBuffNotifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyPlayerBuffEvent>()
            .add_systems(Update, notify_player_buff);
    }
}

#[derive(Event)]
pub struct NotifyPlayerBuffEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_player_buff(
    mut events: EventReader<NotifyPlayerBuffEvent>,
    mut player: ResMut<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                player.info().notify_player_buff(),
                event.1,
                event.2,
                event.3);
        }
    }
}

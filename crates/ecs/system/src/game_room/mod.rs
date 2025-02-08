use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use proto::p26::Sc26120;

pub struct GameRoomNotifyPlugin;

impl Plugin for GameRoomNotifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyGameRoomEvent>()
            .add_systems(Update, notify_game_room_data);
    }
}

#[derive(Event)]
pub struct NotifyGameRoomEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_game_room_data(
    mut events: EventReader<NotifyGameRoomEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                Sc26120::default(),
                event.1,
                event.2,
                event.3);
        }
    }
}

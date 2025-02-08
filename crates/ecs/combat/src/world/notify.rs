use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;

pub struct WorldDataNotifyPlugin;

impl Plugin for WorldDataNotifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyWorldDataEvent>()
            .add_systems(Update, notify_world_data);
    }
}

#[derive(Event)]
pub struct NotifyWorldDataEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_world_data(
    mut events: EventReader<NotifyWorldDataEvent>,
    mut player: ResMut<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                player.info().notify_world_data(),
                event.1,
                event.2,
                event.3);
        }
    }
}

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use proto::p11::Sc11752;

pub struct RefluxDataNotifyPlugin;

impl Plugin for RefluxDataNotifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyRefluxDataEvent>()
            .add_systems(Update, notify_reflux_data);
    }
}

#[derive(Event)]
pub struct NotifyRefluxDataEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_reflux_data(
    mut events: EventReader<NotifyRefluxDataEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                Sc11752::default(),
                event.1,
                event.2,
                event.3);
        }
    }
}

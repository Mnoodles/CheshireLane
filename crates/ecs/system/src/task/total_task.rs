use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use proto::p20::Sc20201;

pub struct TotalTaskRegisterPlugin;

impl Plugin for TotalTaskRegisterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyTotalTaskRegisterEvent>()
            .add_systems(Update, notify_total_task_register);
    }
}

#[derive(Event)]
pub struct NotifyTotalTaskRegisterEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_total_task_register(
    mut events: EventReader<NotifyTotalTaskRegisterEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                // TODO: total task
                Sc20201::default(),
                event.1,
                event.2,
                event.3);
        }
    }
}

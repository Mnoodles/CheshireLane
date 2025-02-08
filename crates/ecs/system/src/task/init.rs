use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use proto::p20::Sc20001;

pub struct InitTaskInfoPlugin;

impl Plugin for InitTaskInfoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyInitTaskInfoEvent>()
            .add_systems(Update, notify_task_info);
    }
}

#[derive(Event)]
pub struct NotifyInitTaskInfoEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_task_info(
    mut events: EventReader<NotifyInitTaskInfoEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                // TODO: task
                Sc20001::default(),
                event.1,
                event.2,
                event.3);
        }
    }
}

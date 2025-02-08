use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use proto::p20::Sc20101;

pub struct WeekTaskProgressInfoPlugin;

impl Plugin for WeekTaskProgressInfoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyWeekTaskProgressInfoEvent>()
            .add_systems(Update, notify_week_task_progress_info);
    }
}

#[derive(Event)]
pub struct NotifyWeekTaskProgressInfoEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_week_task_progress_info(
    mut events: EventReader<NotifyWeekTaskProgressInfoEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                // TODO: week task
                Sc20101::default(),
                event.1,
                event.2,
                event.3);
        }
    }
}

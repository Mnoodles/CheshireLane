use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;

pub struct StatisticsNotifyPlugin;

impl Plugin for StatisticsNotifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyStatisticsEvent>()
            .add_systems(Update, notify_statistics);
    }
}

#[derive(Event)]
pub struct NotifyStatisticsEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_statistics(
    mut events: EventReader<NotifyStatisticsEvent>,
    mut player: ResMut<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                player.info().notify_statistics(),
                event.1,
                event.2,
                event.3);
        }
    }
}

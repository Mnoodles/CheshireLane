use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use proto::p13::Sc13201;

pub struct DailyLevelRegisterNotifyPlugin;

impl Plugin for DailyLevelRegisterNotifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyDailyLevelRegisterEvent>()
            .add_systems(Update, register_daily_level);
    }
}

#[derive(Event)]
pub struct NotifyDailyLevelRegisterEvent(pub u32, pub u16, pub u8, pub u8);

pub fn register_daily_level(
    mut events: EventReader<NotifyDailyLevelRegisterEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                Sc13201::default(),
                event.1,
                event.2,
                event.3);
        }
    }
}

use bevy_app::{App, Plugin, Update};
use bevy_ecs::change_detection::{Res, ResMut};
use bevy_ecs::event::{Event, EventReader};
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use proto::p50::Sc50000;

pub struct NotificationRegisterPlugin;

impl Plugin for NotificationRegisterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyNotificationRegisterEvent>()
            .add_systems(Update, notify_notification_register);
    }
}

#[derive(Event)]
pub struct NotifyNotificationRegisterEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_notification_register(
    mut events: EventReader<NotifyNotificationRegisterEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                Sc50000::default(),
                event.1,
                event.2,
                event.3);
        }
    }
}

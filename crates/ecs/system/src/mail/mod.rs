pub mod compensate;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use proto::p30::Sc30001;

pub struct MailRegisterPlugin;

impl Plugin for MailRegisterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyMailRegisterEvent>()
            .add_systems(Update, notify_mail_register);
    }
}

#[derive(Event)]
pub struct NotifyMailRegisterEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_mail_register(
    mut events: EventReader<NotifyMailRegisterEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                Sc30001::default(),
                event.1,
                event.2,
                event.3);
        }
    }
}

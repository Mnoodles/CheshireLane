use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use proto::p11::{Sc11200, Sc11700};

pub struct ActivityDataNotifyPlugin;

impl Plugin for ActivityDataNotifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyActivityDataEvent>()
            .add_systems(Update, notify_activity_data);
    }
}

#[derive(Event)]
pub struct NotifyActivityDataEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_activity_data(
    mut events: EventReader<NotifyActivityDataEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                Sc11200::default(),
                event.1,
                event.2,
                event.3);
        }
    }
}

pub struct InstagramRegisterPlugin;

impl Plugin for InstagramRegisterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyInstagramRegisterEvent>()
            .add_systems(Update, notify_instagram_register);
    }
}

#[derive(Event)]
pub struct NotifyInstagramRegisterEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_instagram_register(
    mut events: EventReader<NotifyInstagramRegisterEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                Sc11700::default(),
                event.1,
                event.2,
                event.3);
        }
    }
}

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use proto::p28::Sc28000;

pub struct DormDataNotifyPlugin;

impl Plugin for DormDataNotifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyDormDataEvent>()
            .add_systems(Update, notify_dorm_data);
    }
}

#[derive(Event)]
pub struct NotifyDormDataEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_dorm_data(
    mut events: EventReader<NotifyDormDataEvent>,
    mut player: ResMut<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                // TODO: make Dorm System an Entity
                player.info().notify_dorm_data(),
                event.1,
                event.2,
                event.3);
        }
    }
}

pub struct ApartmentRegisterPlugin;

impl Plugin for ApartmentRegisterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyApartmentRegisterEvent>()
            .add_systems(Update, notify_apartment_register);
    }
}

#[derive(Event)]
pub struct NotifyApartmentRegisterEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_apartment_register(
    mut events: EventReader<NotifyApartmentRegisterEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                Sc28000::default(),
                event.1,
                event.2,
                event.3);
        }
    }
}

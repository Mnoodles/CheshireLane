use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use proto::p12::Sc12024;

pub struct BuildShipNotifyPlugin;

impl Plugin for BuildShipNotifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyBuildShipEvent>()
            .add_systems(Update, notify_build_ship_data);
    }
}

#[derive(Event)]
pub struct NotifyBuildShipEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_build_ship_data(
    mut events: EventReader<NotifyBuildShipEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                // TODO: make Build System an Entity
                Sc12024::default(),
                event.1,
                event.2,
                event.3);
        }
    }
}

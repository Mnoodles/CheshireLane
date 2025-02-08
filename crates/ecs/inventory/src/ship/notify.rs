use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;

pub struct ShipDataNotifyPlugin;

impl Plugin for ShipDataNotifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyShipDataEvent>()
            .add_systems(Update, notify_ship_data);
    }
}

#[derive(Event)]
pub struct NotifyShipDataEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_ship_data(
    mut events: EventReader<NotifyShipDataEvent>,
    mut player: ResMut<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                player.info().notify_player_ships_data(),
                event.1,
                event.2,
                event.3);
        }
    }
}

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;

pub struct ShipSkinDataNotifyPlugin;

impl Plugin for ShipSkinDataNotifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyShipSkinDataEvent>()
            .add_systems(Update, notify_ship_skin_data);
    }
}

#[derive(Event)]
pub struct NotifyShipSkinDataEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_ship_skin_data(
    mut events: EventReader<NotifyShipSkinDataEvent>,
    mut player: ResMut<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                player.info().notify_player_ship_skins_data(),
                event.1,
                event.2,
                event.3);
        }
    }
}

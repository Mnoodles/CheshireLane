pub mod equip_skin;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;

pub struct EquipDataNotifyPlugin;

impl Plugin for EquipDataNotifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyEquipDataEvent>()
            .add_systems(Update, notify_equip_data);
    }
}

#[derive(Event)]
pub struct NotifyEquipDataEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_equip_data(
    mut events: EventReader<NotifyEquipDataEvent>,
    mut player: ResMut<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                player.info().notify_equip_data(),
                event.1,
                event.2,
                event.3);
        }
    }
}

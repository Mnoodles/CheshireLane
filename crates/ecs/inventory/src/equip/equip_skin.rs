use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;

pub struct EquipSkinDataNotifyPlugin;

impl Plugin for EquipSkinDataNotifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyEquipSkinDataEvent>()
            .add_systems(Update, notify_equip_skin_data);
    }
}

#[derive(Event)]
pub struct NotifyEquipSkinDataEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_equip_skin_data(
    mut events: EventReader<NotifyEquipSkinDataEvent>,
    mut player: ResMut<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                player.info().notify_equip_skin_data(),
                event.1,
                event.2,
                event.3);
        }
    }
}

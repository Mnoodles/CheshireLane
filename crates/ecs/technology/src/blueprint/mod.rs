use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use proto::p63::Sc63100;

pub struct BlueprintDataNotifyPlugin;

impl Plugin for BlueprintDataNotifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyBlueprintDataEvent>()
            .add_systems(Update, notify_blueprint_data);
    }
}

#[derive(Event)]
pub struct NotifyBlueprintDataEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_blueprint_data(
    mut events: EventReader<NotifyBlueprintDataEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                Sc63100::default(),
                event.1,
                event.2,
                event.3);
        }
    }
}

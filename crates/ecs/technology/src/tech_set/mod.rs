use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use proto::p64::Sc64000;

pub struct TechSetListsNotifyPlugin;

impl Plugin for TechSetListsNotifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyTechSetListsEvent>()
            .add_systems(Update, notify_tech_set_lists_data);
    }
}

#[derive(Event)]
pub struct NotifyTechSetListsEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_tech_set_lists_data(
    mut events: EventReader<NotifyTechSetListsEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                Sc64000::default(),
                event.1,
                event.2,
                event.3);
        }
    }
}

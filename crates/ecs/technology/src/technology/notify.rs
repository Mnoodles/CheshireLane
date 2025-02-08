use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use proto::p63::Sc63000;

pub struct TechnologyDataNotifyPlugin;

impl Plugin for TechnologyDataNotifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyTechnologyDataEvent>()
            .add_systems(Update, notify_technology_data);
    }
}

#[derive(Event)]
pub struct NotifyTechnologyDataEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_technology_data(
    mut events: EventReader<NotifyTechnologyDataEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                Sc63000::default(),
                event.1,
                event.2,
                event.3);
        }
    }
}

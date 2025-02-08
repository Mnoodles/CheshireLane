use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;

pub struct NavalAcademyNotifyPlugin;

impl Plugin for NavalAcademyNotifyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyNavalAcademyEvent>()
            .add_systems(Update, notify_naval_academy);
    }
}

#[derive(Event)]
pub struct NotifyNavalAcademyEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_naval_academy(
    mut events: EventReader<NotifyNavalAcademyEvent>,
    mut player: ResMut<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                // TODO: make Naval Academy System an Entity
                player.info().notify_naval_academy(),
                event.1,
                event.2,
                event.3);
        }
    }
}

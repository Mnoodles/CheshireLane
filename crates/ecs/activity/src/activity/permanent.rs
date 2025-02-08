use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use proto::p11::Sc11210;

pub struct ActivityPermanentRegisterPlugin;

impl Plugin for ActivityPermanentRegisterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyActivityPermanentRegisterEvent>()
            .add_systems(Update, notify_energy_recover_time_info);
    }
}

#[derive(Event)]
pub struct NotifyActivityPermanentRegisterEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_energy_recover_time_info(
    mut events: EventReader<NotifyActivityPermanentRegisterEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                Sc11210::default(),
                event.1,
                event.2,
                event.3);
        }
    }
}

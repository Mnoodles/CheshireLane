use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use proto::p12::Sc12031;

pub struct EnergyRecoverTimePlugin;

impl Plugin for EnergyRecoverTimePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyEnergyRecoverTimeEvent>()
            .add_systems(Update, notify_energy_recover_time_info);
    }
}

#[derive(Event)]
pub struct NotifyEnergyRecoverTimeEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_energy_recover_time_info(
    mut events: EventReader<NotifyEnergyRecoverTimeEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                Sc12031 {
                    energy_auto_increase_time: common::time::now_timestamp_s() as u32,
                },
                event.1,
                event.2,
                event.3);
        }
    }
}

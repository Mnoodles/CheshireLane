use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use proto::p30::Sc30101;

pub struct CompensateRegisterPlugin;

impl Plugin for CompensateRegisterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NotifyCompensateRegisterEvent>()
            .add_systems(Update, notify_compensate_register);
    }
}

#[derive(Event)]
pub struct NotifyCompensateRegisterEvent(pub u32, pub u16, pub u8, pub u8);

pub fn notify_compensate_register(
    mut events: EventReader<NotifyCompensateRegisterEvent>,
    player: Res<Player>,
    mut message_output: ResMut<MessageOutput>,
) {
    for event in events.read() {
        if event.0 == player.uid() {
            message_output.send_seq(
                Sc30101::default(),
                event.1,
                event.2,
                event.3);
        }
    }
}

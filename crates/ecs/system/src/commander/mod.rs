use bevy_ecs::change_detection::ResMut;
use bevy_ecs::event::EventReader;
use common::logging;
use ecs_message::event::ClientMessageEvent;
use ecs_message::output::MessageOutput;
use proto::p25::{Commanderhomeslot, Cs25026, Sc25027};

pub mod notify;

pub fn get_commander_home(
    mut events: EventReader<ClientMessageEvent>,
    mut message_output: ResMut<MessageOutput>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs25026>() {
            logging::debug!("FetchSecondaryPassword: {:?}", req);

            message_output.send(Sc25027 {
                slots: vec![Commanderhomeslot {
                    id: 1,
                    op_flag: 7,
                    style: 1,
                    ..Default::default()
                }],
                level: 1,
                ..Default::default()
            }, message.id());
        }
    }
}

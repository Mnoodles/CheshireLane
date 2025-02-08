use bevy_ecs::change_detection::ResMut;
use bevy_ecs::event::EventReader;
use common::logging;
use ecs_message::event::ClientMessageEvent;
use ecs_message::output::MessageOutput;
use proto::p13::{Cs13505, Sc13506};

pub fn remaster_info_request(
    mut events: EventReader<ClientMessageEvent>,
    mut message_output: ResMut<MessageOutput>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs13505>() {
            logging::debug!("RemasterInfoRequest: {:?}", req);

            message_output.send(Sc13506::default(), message.id());
        }
    }
}

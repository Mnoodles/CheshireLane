use bevy_ecs::change_detection::ResMut;
use bevy_ecs::event::EventReader;
use common::logging;
use ecs_message::event::ClientMessageEvent;
use ecs_message::output::MessageOutput;
use proto::p26::{Cs26101, Sc26102};

pub fn mini_game_request(
    mut events: EventReader<ClientMessageEvent>,
    mut message_output: ResMut<MessageOutput>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs26101>() {
            logging::debug!("MiniGameRequest: {:?}", req);

            message_output.send(Sc26102::default(), message.id());
        }
    }
}

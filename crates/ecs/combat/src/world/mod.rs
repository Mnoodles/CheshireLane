pub mod notify;

use bevy_ecs::prelude::*;
use common::logging;
use ecs_message::event::ClientMessageEvent;
use ecs_message::output::MessageOutput;
use proto::p34::{Cs34501, Sc34502};

pub fn get_world_boss(
    mut events: EventReader<ClientMessageEvent>,
    mut message_output: ResMut<MessageOutput>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs34501>() {
            logging::debug!("GetWorldBoss: {:?}", req);
            message_output.send(
                Sc34502::default(),
                message.id());
        }
    }
}

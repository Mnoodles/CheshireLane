pub mod notify;
pub mod permanent;

use bevy_ecs::change_detection::ResMut;
use bevy_ecs::event::EventReader;
use common::logging;
use ecs_message::event::ClientMessageEvent;
use ecs_message::output::MessageOutput;
use proto::p11::{Cs11710, Cs11722, Sc11711, Sc11723};

pub fn instagram_chat_get_data(
    mut events: EventReader<ClientMessageEvent>,
    mut message_output: ResMut<MessageOutput>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs11710>() {
            logging::debug!("InstagramChat: {:?}", req);

            message_output.send(Sc11711::default(), message.id());
        }
    }
}

pub fn instagram_chat_active_topic(
    mut events: EventReader<ClientMessageEvent>,
    mut message_output: ResMut<MessageOutput>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs11722>() {
            logging::debug!("InstagramChat: {:?}", req);

            message_output.send(Sc11723::default(), message.id());
        }
    }
}

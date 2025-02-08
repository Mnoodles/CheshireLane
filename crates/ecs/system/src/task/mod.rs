use bevy_ecs::change_detection::ResMut;
use bevy_ecs::event::EventReader;
use common::logging;
use ecs_message::event::ClientMessageEvent;
use ecs_message::output::MessageOutput;
use proto::p20::{Cs20007, Sc20008};

pub mod init;
pub mod week_progress;
pub mod total_task;

pub fn trigger_task(
    mut events: EventReader<ClientMessageEvent>,
    mut message_output: ResMut<MessageOutput>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs20007>() {
            logging::debug!("TriggerTask: {:?}", req);

            message_output.send(Sc20008::default(), message.id());
        }
    }
}

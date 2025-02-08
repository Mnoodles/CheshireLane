use bevy_ecs::change_detection::ResMut;
use bevy_ecs::event::EventReader;
use common::logging;
use ecs_message::event::ClientMessageEvent;
use ecs_message::output::MessageOutput;
use proto::p18::{Cs18001, Sc18002};

pub fn get_season_info(
    mut events: EventReader<ClientMessageEvent>,
    mut message_output: ResMut<MessageOutput>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs18001>() {
            logging::debug!("GetSeasonInfo: {:?}", req);

            message_output.send(Sc18002 {
                rank: 1,
                ..Default::default()
            }, message.id());
        }
    }
}

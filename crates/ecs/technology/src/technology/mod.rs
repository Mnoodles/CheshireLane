use bevy_ecs::change_detection::ResMut;
use bevy_ecs::event::EventReader;
use common::logging;
use ecs_message::event::ClientMessageEvent;
use ecs_message::output::MessageOutput;
use proto::p34::{Cs34001, MetaShipInfo, Sc34002};

pub mod notify;

pub fn metadata_request(
    mut events: EventReader<ClientMessageEvent>,
    mut message_output: ResMut<MessageOutput>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs34001>() {
            logging::debug!("MetadataRequest: {:?}", req);

            message_output.send(Sc34002 {
                meta_ship_list: req.group_id.iter()
                    .map(|id| MetaShipInfo {
                        group_id: *id,
                        ..Default::default()
                    })
                    .collect(),
            }, message.id());
        }
    }
}

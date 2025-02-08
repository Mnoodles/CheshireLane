pub mod notify;

use bevy_ecs::change_detection::ResMut;
use bevy_ecs::event::EventReader;
use common::logging;
use ecs_message::event::ClientMessageEvent;
use ecs_message::output::MessageOutput;
use proto::p15::{Cs15008, Sc15009};
use proto::p16::{Cs16104, Sc16105};

pub fn get_charge_list(
    mut events: EventReader<ClientMessageEvent>,
    mut message_output: ResMut<MessageOutput>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs16104>() {
            logging::debug!("GetChargeList: {:?}", req);

            message_output.send(Sc16105::default(), message.id());
        }
    }
}

pub fn sell_item(
    mut events: EventReader<ClientMessageEvent>,
    mut message_output: ResMut<MessageOutput>,
) {
    for message in events.read() {
        if message.cmd_id() == 15008 {
            logging::debug!("SellItem: BUG");

            message_output.send(Sc15009::default(), message.id());
        }
        // A bug! Could not decode proto
        if let Some(req) = message.decode::<Cs15008>() {
            logging::debug!("SellItem: {:?}", req);

            // message_output.send(Sc15009::default(), message.id());
        }
    }
}

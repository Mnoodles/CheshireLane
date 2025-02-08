use bevy_ecs::change_detection::ResMut;
use bevy_ecs::event::{EventReader, EventWriter};
use common::logging;
use ecs_database::SavePlayerDataEvent;
use ecs_message::event::ClientMessageEvent;
use ecs_message::output::MessageOutput;
use ecs_persistence::Player;
use proto::p12::{Cs12202, Sc12203};
use proto::p63::{Cs63317, Sc63318};

pub mod statistics;
pub mod notify;

pub fn meta_character_tactics_info_request(
    mut events: EventReader<ClientMessageEvent>,
    mut message_output: ResMut<MessageOutput>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs63317>() {
            logging::debug!("MetaCharacterTacticsInfoRequest: {:?}", req);

            message_output.send(Sc63318::default(), message.id());
        }
    }
}

pub fn set_ship_skin(
    mut events: EventReader<ClientMessageEvent>,
    mut player: ResMut<Player>,
    mut message_output: ResMut<MessageOutput>,
    mut save_player_data_event: EventWriter<SavePlayerDataEvent>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs12202>() {
            logging::debug!("UpdateStory: {:?}", req);

            if let Some(info) = player.info().ships.iter_mut()
                .find(|info| info.id == req.ship_id) {
                info.skin_id = req.skin_id;
            }

            message_output.send(Sc12203::default(), message.id());
            
            save_player_data_event.send(SavePlayerDataEvent(player.uid()));
        }
    }
}

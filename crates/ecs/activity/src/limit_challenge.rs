use bevy_ecs::change_detection::ResMut;
use bevy_ecs::event::EventReader;
use common::logging;
use ecs_message::event::ClientMessageEvent;
use ecs_message::output::MessageOutput;
use proto::common::Kvdata;
use proto::p24::{Cs24020, Sc24021};

pub fn limit_challenge_request(
    mut events: EventReader<ClientMessageEvent>,
    mut message_output: ResMut<MessageOutput>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs24020>() {
            logging::debug!("LimitChallengeRequest: {:?}", req);

            message_output.send(Sc24021 {
                awards: (10022..=10024).map(|key| Kvdata {
                    key,
                    value: 0,
                }).collect(),
                ..Default::default()
            }, message.id());
        }
    }
}

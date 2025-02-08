use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use common::logging;
use ecs_message::event::ClientMessageEvent;
use ecs_message::output::MessageOutput;
use proto::p10::{Cs10100, Sc10101};

pub struct ConnectionMgrPlugin;

impl Plugin for ConnectionMgrPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreUpdate, reset_hb_timer);
    }
}

fn reset_hb_timer(
    mut events: EventReader<ClientMessageEvent>,
    mut message_output: ResMut<MessageOutput>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs10100>() {
            logging::debug!("ConnectionMgrResetHBTimer: {:?}", req);

            if req.need_request == 1 {
                message_output.send(Sc10101 {
                    state: 0,
                },
                message.id());
            }
        }
    }
}

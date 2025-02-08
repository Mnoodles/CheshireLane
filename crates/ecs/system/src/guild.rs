use bevy_ecs::change_detection::ResMut;
use bevy_ecs::event::EventReader;
use common::logging;
use ecs_message::event::ClientMessageEvent;
use ecs_message::output::MessageOutput;
use proto::p60::{Cs60037, Cs60102, GuildBaseInfo, GuildInfo, Sc60000, Sc60103, UserGuildInfo};
use proto::p62::{Cs62100, Sc62101};

pub fn get_guild_info(
    mut events: EventReader<ClientMessageEvent>,
    mut message_output: ResMut<MessageOutput>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs60037>() {
            logging::debug!("GetGuildInfo: {:?}", req);

            message_output.send(Sc60000 {
                guild: GuildInfo {
                    member: vec![],
                    base: GuildBaseInfo {
                        faction: 1,
                        level: 1,
                        policy: 1,
                        ..Default::default()
                    },
                    log: vec![],
                    guild_ex: Default::default(),
                },
            }, message.id());
        }
    }
}

pub fn get_public_guild_user_data_create(
    mut events: EventReader<ClientMessageEvent>,
    mut message_output: ResMut<MessageOutput>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs62100>() {
            logging::debug!("GetPublicGuildUserData_Create: {:?}", req);

            message_output.send(Sc62101::default(), message.id());
        }
    }
}

pub fn get_user_info_create(
    mut events: EventReader<ClientMessageEvent>,
    mut message_output: ResMut<MessageOutput>,
) {
    for message in events.read() {
        if let Some(req) = message.decode::<Cs60102>() {
            logging::debug!("GetGuildUserInfo: {:?}", req);

            message_output.send(Sc60103 {
                user_info: UserGuildInfo {
                    donate_tasks: vec![1, 13, 2],
                    ..Default::default()
                },
            }, message.id());
        }
    }
}

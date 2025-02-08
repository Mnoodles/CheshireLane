use std::sync::{Arc, OnceLock};
use dashmap::mapref::one::RefMut;
use network::tcp_server::packet_handler::PacketHandlerItem;
use common::logging;
use config::CONFIG;
use database::schema::CheshireDBSchemaGetU32Id;
use network::inner_packet::InnerPacket;
use network::packet::Packet;
use proto::CmdID;
use proto::p10::{Cs10022, Sc10023};
use encryption::Md5;
use crate::AppState;
use crate::session::Session;

pub async fn on_message(state: &'static AppState, item: PacketHandlerItem) {
    match item {
        PacketHandlerItem::NewConnection(connect) => {
            logging::debug!("Create new session: ({}, {})",
                connect.conv, connect.token);
            state.sessions.insert(connect.conv, Arc::new(Session {
                connection: connect,
                uid: OnceLock::new(),
            }));
        }
        PacketHandlerItem::DropConnection(conv) => {
            logging::debug!("Delete session conv {}", conv);
            state.sessions.remove(&conv);
        }
        PacketHandlerItem::Packet(conv, data) => {
            match state.sessions.get_mut(&conv) {
                Some(session) => {
                    if let Ok(packet) = Packet::new(*data) {
                        match packet.cmd_id {
                            Cs10022::CMD_ID => {
                                // Cs10022:
                                //   check_key = md5(server_ticket + salt)
                                let req = packet.decode::<Cs10022>().unwrap();
                                logging::debug!("-> Conv {} Id {} Req: {:?}",
                                    conv, packet.id, req);

                                if let Err(err) = session.uid.set(req.account_id) {
                                    logging::warn!("Unable to set uid for session: {}", err);
                                }
                                logging::debug!("Set uid {:?} for Conv {}",
                                    session.uid.get(), conv);

                                let md5 = Md5::hash(
                                    &req.server_ticket,
                                    Some(&CONFIG.gate_config.salt));

                                let (player_id, is_banned) =
                                    if let Ok(Some(player)) = state.db.get_player_by_uid(req
                                        .account_id.to_string()).await { 
                                        (Some(player.uid()), Some(player.is_banned)) 
                                    } else { (None, None) };

                                let result = {
                                    if md5 == req.check_key {
                                        if let Ok(Some(_)) = state.db
                                            .get_account_by_uid(req.account_id.to_string())
                                            .await {
                                            // Check if a player has been banned
                                            if is_banned == Some(true) {
                                                17
                                            } else {
                                                0
                                            }
                                        } else { 1 }
                                    } else { 1 }
                                };

                                // Send to game-server for game creating if login successfully
                                if result == 0 {
                                    send_inner_packet_to_game_server(&session, &packet, state)
                                        .await;
                                } else {
                                    let user_id = match player_id {
                                        Some(Ok(id)) => id,
                                        _ => 0,
                                    };

                                    // Sc10023:
                                    //   result:
                                    //     0: Success
                                    //     6: login_game_login_full
                                    //     13: login_game_not_ready
                                    //     15: login_game_rigister_full
                                    //     17: SERVER_LOGIN_FAILED_USER_BANNED
                                    //     18: SERVER_LOGIN_WAIT
                                    //     _: SERVER_LOGIN_FAILED + result
                                    let rsp = Sc10023 {
                                        db_load: None,
                                        result,
                                        user_id,
                                        server_ticket: req.server_ticket,
                                        server_load: None,
                                    };
                                    logging::debug!("<- Conv {} ID {} Rsp: {:?}",
                                        conv, packet.id, rsp);

                                    let packet = Packet::encode(&rsp, packet.id);
                                    let data = packet.to_bytes().unwrap();
                                    if let Err(e) = session.connection.send(data).await {
                                        logging::error!("Conv {} Send packet error: {}", conv, e);
                                    }
                                }
                            },
                            cmd_id => {
                                logging::debug!("-> Conv {} CmdId: {}", conv, cmd_id);
                                send_inner_packet_to_game_server(&session, &packet, state).await;
                            }
                        }
                    } else {
                        logging::error!("Received invalid packet from session conv {}", conv);
                    }
                },
                None => logging::error!("Session {} not found", conv),
            }
        }
    }
}

async fn send_inner_packet_to_game_server(
    session: &RefMut<'static, u32, Arc<Session>>,
    packet: &Packet,
    state: &'static AppState,
) {
    logging::debug!("<-- Conv {} send packet {} to game-server",
        session.connection.conv, packet.cmd_id);
    let inner = InnerPacket::build(
        session.connection.conv,
        session.connection.token,
        session.uid.get().unwrap().clone(),
        packet.cmd_id,
        packet.id,
        packet.data.clone());
    state.game_server_socket.send(inner.to_raw()).await;
}

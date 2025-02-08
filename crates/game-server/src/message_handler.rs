use tokio::sync::mpsc;
use common::logging;
use ecs_message::output::ClientOutput;
use network::inner_packet::InnerPacket;
use proto::CmdID;
use crate::AppState;

pub async fn on_message(state: &'static AppState, data: Box<[u8]>) {
    if let Some(inner_packet) = InnerPacket::from_bytes((*data).to_vec()) {
        logging::debug!("--> Conv {} receive packet {} from gate-server",
            inner_packet.get_conv(), inner_packet.get_cmd_id());

        match inner_packet.get_cmd_id() {
            proto::p10::Cs10022::CMD_ID => {
                // Login proto, create new world
                player_login(
                    state,
                    inner_packet.get_uid(),
                    inner_packet.get_conv(),
                    inner_packet.get_token()
                ).await;

                state.simulator.add_client_packet(
                    inner_packet.get_uid(),
                    proto::p10::Cs10022::CMD_ID,
                    inner_packet.get_id(),
                    Box::from(inner_packet.get_data()),
                    true
                );
            },
            cmd_id => {
                // add client packet to world
                state.simulator.add_client_packet(
                    inner_packet.get_uid(),
                    cmd_id,
                    inner_packet.get_id(),
                    Box::from(inner_packet.get_data()),
                    true
                );
            },
        }
    } else {
        logging::error!("Received invalid inner packet");
    }
}

async fn player_login(
    state: &'static AppState,
    uid: u32,
    conv: u32,
    token: u32,
) {
    let (tx, rx) = mpsc::channel(32);

    let Some(player_data) = state.db.fetch(uid).await else {
        logging::error!("Failed to get player {} data", uid);
        return;
    };

    tokio::spawn(packet_sink(state, uid, conv, token, rx));

    state.simulator.create_world(uid, player_data, ClientOutput::new(tx));
}

async fn packet_sink(
    state: &'static AppState,
    uid: u32,
    conv: u32,
    token: u32,
    mut rx: mpsc::Receiver<(u16, u16, Box<[u8]>, Option<(u8,  u8)>)>,
) {
    while let Some((cmd_id, id, data, seq)) = rx.recv().await {
        logging::debug!("<-- Conv {} send packet {} to game-server", conv, cmd_id);
        let mut inner_packet = InnerPacket::build(
            conv,
            token,
            uid,
            cmd_id,
            id,
            data.to_vec(),
        );
        if let Some((seq_id, seq_num)) = seq {
            inner_packet.set_sequence(seq_id, seq_num);
        }
        state.gate_server_socket
            .send(inner_packet.to_raw())
            .await;
    }
}

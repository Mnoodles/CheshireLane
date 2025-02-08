use std::collections::{BTreeMap, HashMap};
use tokio::sync::mpsc;
use common::logging;
use network::inner_packet::InnerPacket;
use network::packet::Packet;
use crate::AppState;

pub async fn on_message(handler: MessageHandler, data: Box<[u8]>) {
    handler.0.send(data).await.unwrap();
}

#[derive(Clone)]
pub struct MessageHandler(mpsc::Sender<Box<[u8]>>);

impl MessageHandler {
    pub fn new(state: &'static AppState) -> Self {
        let (tx, rx) = mpsc::channel(32);
        tokio::spawn(handler_loop(state, rx));

        Self(tx)
    }
}

struct SequenceBuffer {
    expected: u8,
    fragments: BTreeMap<u8, Packet>,
}

async fn handler_loop(
    state: &'static AppState,
    mut rx: mpsc::Receiver<Box<[u8]>>,
) {
    let mut sequence_buffers: HashMap<(u32, u32, u16), SequenceBuffer> = HashMap::new();

    while let Some(data) = rx.recv().await {
        if let Some(inner_packet) = InnerPacket::from_bytes((*data).to_vec()) {
            logging::debug!("--> Conv {} receive packet {} id {} from game-server",
            inner_packet.get_conv(), inner_packet.get_cmd_id(), inner_packet.get_id());

            let data = inner_packet.get_data().to_vec();
            let packet = Packet::encode_raw(
                inner_packet.get_cmd_id(),
                inner_packet.get_id(),
                data);

            let (seq_id, seq_num) = inner_packet.get_sequence();

            if seq_id == 0 && seq_num == 0 {
                // Not a sequence packet
                send_packet(state, &packet, inner_packet.get_conv()).await;
            } else {
                // Sequence packet
                handle_sequence_packet(
                    state,
                    &mut sequence_buffers,
                    &packet,
                    inner_packet.get_conv(),
                    inner_packet.get_token(),
                    inner_packet.get_id(),
                    seq_id,
                    seq_num).await;
            }
        } else {
            logging::error!("Received invalid inner packet");
        }
    }
}

async fn send_packet(state: &'static AppState, packet: &Packet, conv: u32) {
    if let Some(session) = state.sessions.get(&conv) {
        logging::debug!("<- Conv {} CmdId: {} ID: {}",
                session.connection.conv, packet.cmd_id, packet.id);
        if let Err(e) = session.connection
            .send(packet.to_bytes().unwrap()).await {
            logging::error!("Conv {} Send packet error: {}", conv, e);
        }
    } else {
        logging::error!("Received inner packet include invalid session conv {}",
                conv);
    }
}

async fn handle_sequence_packet(
    state: &'static AppState,
    sequence_buffers: &mut HashMap<(u32, u32, u16), SequenceBuffer>,
    packet: &Packet,
    conv: u32,
    token: u32,
    id: u16,
    seq_id: u8,
    seq_num: u8,
) {
    let key = (conv, token, id);
    let buffer = sequence_buffers
        .entry(key)
        .or_insert(SequenceBuffer {
            expected: seq_num,
            fragments: BTreeMap::new(),
        });

    if buffer.expected == seq_num {
        if !buffer.fragments.contains_key(&seq_id) {
            buffer.fragments.insert(seq_id, packet.clone());

            if buffer.fragments.len() as u8 == buffer.expected {
                // Check if the seq_id is consecutive
                let expected_seq_ids: Vec<u8> = (1..=buffer.expected)
                    .collect();
                let received_seq_ids: Vec<u8> = buffer.fragments.keys()
                    .cloned()
                    .collect();

                if expected_seq_ids == received_seq_ids {
                    // Right packets, Send
                    for key in received_seq_ids {
                        let packet = buffer.fragments.get(&key).unwrap().clone();
                        send_packet(state, &packet, conv).await;
                    }
                    // Send all-in-one data
                    // let mut all_data = vec![];
                    // for key in received_seq_ids {
                    //     let packet = buffer.fragments.get(&key).unwrap();
                    //     all_data.extend_from_slice(&packet.to_bytes().unwrap());
                    // }
                    // if let Some(session) = state.sessions.get(&conv) {
                    //     let _ = session.connection.send(all_data).await;
                    // }
                } else {
                    logging::error!(
                        "Sequence fragments for conv {} id {} are not consecutive. \
                        Expected: {:?}, Received: {:?}",
                        conv, id, expected_seq_ids, received_seq_ids);
                }
                sequence_buffers.remove(&key);
            }
        } else {
            logging::warn!(
                "Duplicate sequence fragment: conv {} token {} id {} seq_id {}",
                conv, token, id, seq_id);
        }
    } else {
        logging::warn!(
            "Inconsistent seq_num for conv {} token {} id {}: expected {}, get {}",
            conv, token, id, buffer.expected, seq_num);
    }
}

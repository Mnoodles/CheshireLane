use tokio::sync::mpsc;
use bevy_ecs::system::Resource;
use proto::CheshireMessage;

#[derive(Clone)]
pub struct ClientOutput(mpsc::Sender<(u16, u16, Box<[u8]>, Option<(u8,  u8)>)>);

impl ClientOutput {
    pub fn new(tx: mpsc::Sender<(u16, u16, Box<[u8]>, Option<(u8,  u8)>)>) -> Self {
        Self(tx)
    }

    pub fn push(&mut self, message: impl CheshireMessage, id: u16, seq: Option<(u8,  u8)>) {
        self.0.blocking_send((
            message.get_cmd_id(),
            id,
            message.encode_to_vec().into_boxed_slice(),
            seq)).unwrap();
    }
}

#[derive(Resource)]
pub struct MessageOutput(#[allow(dead_code)] u32, ClientOutput);

impl MessageOutput {
    pub fn new(uid: u32, client_output: ClientOutput) -> Self {
        Self(uid, client_output)
    }

    pub fn send(&mut self, message: impl CheshireMessage, id: u16) {
        self.1.push(message, id, None);
    }
    
    pub fn send_seq(&mut self, message: impl CheshireMessage, id: u16, seq_id: u8, seq_num: u8) {
        self.1.push(message, id, Some((seq_id, seq_num)));
    }
}

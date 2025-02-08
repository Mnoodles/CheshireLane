use bevy_ecs::prelude::*;
use proto::CheshireMessage;

#[derive(Event)]
pub struct ClientMessageEvent{
    uid: u32,
    cmd_id: u16,
    id: u16,
    data: Box<[u8]>,
}

impl ClientMessageEvent {
    pub fn new(uid: u32, cmd_id: u16, id: u16, data: Box<[u8]>) -> Self {
        Self {
            uid,
            cmd_id,
            id,
            data,
        }
    }

    pub fn uid(&self) -> u32 {
        self.uid
    }

    pub fn cmd_id(&self) -> u16 {
        self.cmd_id
    }
    
    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn decode<T: CheshireMessage + Default>(&self) -> Option<T> {
        (self.cmd_id == T::CMD_ID)
            .then_some(T::decode(self.data.to_vec().as_slice()).ok())
            .flatten()
    }
}

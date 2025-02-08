use std::sync::OnceLock;
use network::tcp_server::connection::Connection;

pub struct Session {
    pub connection: Connection,
    pub uid: OnceLock<u32>,
}

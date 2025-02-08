use std::collections::HashMap;
use rand::RngCore;
use tokio::io::{AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use common::logging;

#[derive(Clone)]
pub struct Connection {
    pub conv: u32,
    pub token: u32,
    writer_tx: mpsc::UnboundedSender<Vec<u8>>,
}

impl Connection {
    pub fn new(conv: u32, token: u32, writer: WriteHalf<TcpStream>) -> Self {
        let (tx, mut rx) =
            mpsc::unbounded_channel::<Vec<u8>>();

        tokio::spawn(async move {
            let mut writer = writer;
            while let Some(data) = rx.recv().await {
                if let Err(e) = writer.write(&data).await {
                    logging::error!("Error writing to socket: {:?}", e);
                }
            }
        });

        Self {conv, token, writer_tx: tx }
    }

    pub async fn send(&self, data: Vec<u8>) -> anyhow::Result<()> {
        self.writer_tx.send(data)?;
        Ok(())
    }
}

#[derive(Default)]
pub struct ConnectionManager {
    connections: HashMap<u32, Connection>,
    connection_counter: u32,
}

impl ConnectionManager {
    pub fn create(&mut self, writer: WriteHalf<TcpStream>) -> &Connection {
        let (conv, token) = (self.connection_counter, rand::thread_rng().next_u32());
        self.connection_counter += 1;

        let conn = Connection::new(conv, token, writer);

        self.connections.entry(conn.conv).or_insert(conn)
    }

    pub fn get(&self, conv: u32, token: u32) -> Option<&Connection> {
        self.connections
            .get(&conv)
            .and_then(|c| (c.token == token).then_some(c))
    }

    pub fn remove(&mut self, conv: u32, token: u32) -> Option<Connection> {
        if let Some(id) = self.connections.get(&conv) {
            if id.token == token {
                return self.connections.remove(&conv);
            }
        }

        None
    }
}

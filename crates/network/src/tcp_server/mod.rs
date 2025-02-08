pub mod packet_handler;
pub mod connection;

use std::sync::Arc;
use tokio::io::{split, AsyncReadExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use common::logging;

use packet_handler::PacketHandler;
use packet_handler::PacketHandlerCallback;
use connection::ConnectionManager;
use crate::tcp_server::packet_handler::PacketHandlerItem;

pub struct TcpServer {
    listener: Arc<TcpListener>,
    packet_handler: PacketHandler,
    connection_mgr: Arc<Mutex<ConnectionManager>>,
}

impl TcpServer {
    pub async fn bind<S: Send + Sync + Clone + 'static>(
        addr: &'static str,
        state: S,
        callback: impl PacketHandlerCallback<S> + 'static,
    ) -> anyhow::Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        let packet_handler = PacketHandler::new(state, callback);

        Ok(Self {
            listener: Arc::new(listener),
            packet_handler,
            connection_mgr: Arc::new(Mutex::new(ConnectionManager::default())),
        })
    }

    pub async fn serve(self) -> anyhow::Result<()> {
        loop {
            let (socket, addr) = self.listener.accept().await?;
            logging::info!("New connection from: {}", addr);

            // Setting up TCP socket for game
            if let Err(e) = socket.set_nodelay(true) {
                logging::error!("Failed to disable Nagle: {}", e);
            }

            let handler = self.packet_handler.clone();
            let conn_mgr = self.connection_mgr.clone();
            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(socket, handler, conn_mgr).await {
                    logging::error!("Failed to handle connection: {}", e);
                }
            });
        }
    }

    async fn handle_connection(
        socket: TcpStream,
        handler: PacketHandler,
        connection_manager: Arc<Mutex<ConnectionManager>>
    ) -> anyhow::Result<()> {
        let peer_addr = socket.peer_addr()?;
        let (mut reader, writer) = split(socket);

        let conn = connection_manager.lock().await.create(writer).clone();
        handler.0.send(PacketHandlerItem::NewConnection(conn.clone()))?;

        let mut buf = [0u8; 4096];
        loop {
            match reader.read(&mut buf).await {
                Ok(0) => {
                    break;
                }
                Ok(n) => {
                    handler.0.send(PacketHandlerItem::Packet(
                        conn.conv,
                        Box::new(buf[..n].to_vec())))?;
                }
                Err(e) => {
                    logging::error!("Failed to read from socket: {}", e);
                    break;
                }
            }
        }

        handler.0.send(PacketHandlerItem::DropConnection(conn.conv))?;
        connection_manager.lock().await.remove(conn.conv, conn.token);
        logging::info!("Connection from {} closed", peer_addr);

        Ok(())
    }
}

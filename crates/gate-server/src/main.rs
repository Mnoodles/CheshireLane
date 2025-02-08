mod outer_handler;
mod inner_handler;
mod session;

use std::sync::{Arc, OnceLock};
use dashmap::DashMap;
use config::CONFIG;
use database::DbContext;
use network::server_socket::ServerSocket;
use common::{banner, show_info, logging};

use crate::session::Session;

struct AppState {
    sessions: DashMap<u32, Arc<Session>>,
    db: DbContext,
    game_server_socket: ServerSocket,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    banner();
    logging::init(logging::Level::DEBUG);
    show_info();

    static STATE: OnceLock<AppState> = OnceLock::new();

    let db = DbContext::connect().await?;
    let game_server_socket = ServerSocket::new(&CONFIG.gate_config.game_server_addr);

    let state = STATE.get_or_init(move || AppState {
        sessions: DashMap::new(),
        db,
        game_server_socket,
    });

    logging::info!("Gate server (inner) listening on {}", CONFIG.gate_config.inner_listen_addr);
    let inner_message_handler = inner_handler::MessageHandler::new(state);
    network::listener::listen(
        &CONFIG.gate_config.inner_listen_addr,
        inner_message_handler,
        inner_handler::on_message,
    ).await?;

    logging::info!("Gate server (outer) listening on {}", CONFIG.gate_config.outer_host);
    let tcp_server = network::tcp_server::TcpServer::bind(
        &CONFIG.gate_config.outer_host,
        state,
        outer_handler::on_message).await?;
    tcp_server.serve().await?;

    Ok(())
}

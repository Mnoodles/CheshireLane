mod command;
mod message_handler;
mod db_worker;

use std::sync::OnceLock;
use anyhow::Result;
use common::{banner, logging, show_info};
use common::logging::Level;
use common::command::CheshireCommandManager;

use command::GameServerCommand;
use config::CONFIG;
use database::DbContext;
use game_server_core::LogicSimulator;
use network::server_socket::ServerSocket;
use crate::db_worker::DbWorkerHandle;

#[derive(thiserror::Error, Debug)]
enum GameServerError {
    #[error("Failed to init logging")]
    InitLoggingFailure,
    #[error("Start server error: {0}")]
    StartServerFailure(#[from] std::io::Error),
}

struct AppState {
    pub db: DbWorkerHandle,
    pub simulator: LogicSimulator,
    pub gate_server_socket: ServerSocket,
}

#[tokio::main]
async fn main() -> Result<()> {
    banner();

    let rl = logging::init_with_readline(Level::DEBUG)
        .ok_or(GameServerError::InitLoggingFailure)?;
    let mut cmd_manager = CheshireCommandManager::new();
    cmd_manager.run(rl);

    show_info();

    data::load_all()?;

    static STATE: OnceLock<AppState> = OnceLock::new();

    let db = DbContext::connect().await?;
    let (db, save_data_tx) = db_worker::start(db);

    let simulator = LogicSimulator::spawn(save_data_tx);
    let gate_server_socket = ServerSocket::new(&CONFIG.game_config.gate_server_addr);

    let state = STATE.get_or_init(move || AppState {
        db,
        simulator,
        gate_server_socket,
    });

    cmd_manager.set_cmd(GameServerCommand::new(state)).await;

    logging::info!("Game server listening on {}", CONFIG.game_config.listen_addr);
    let _ = network::listener::listen(
        &CONFIG.game_config.listen_addr,
        state,
        message_handler::on_message,
    ).await?.await;

    Ok(())
}

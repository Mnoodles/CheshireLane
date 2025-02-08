mod handler;
mod dispatch_config;

use std::fs;
use std::sync::OnceLock;
use config::CONFIG;
use network::tcp_listener::listen;
use common::{banner, show_info, logging};

pub static VERSION: OnceLock<dispatch_config::Version> = OnceLock::new();
pub static SERVERS: OnceLock<dispatch_config::Servers> = OnceLock::new();

// Fake server for host `blhxusgate.yo-star.com`

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    banner();
    logging::init(logging::Level::DEBUG);
    show_info();

    let _ = VERSION.set(serde_json::from_str(
        &fs::read_to_string(&CONFIG.dispatch_config.version_path)?)?);
    let _ = SERVERS.set(serde_json::from_str(
        &fs::read_to_string(&CONFIG.dispatch_config.servers_path)?)?);


    let addr = format!("0.0.0.0:{}", CONFIG.dispatch_config.port);
    logging::info!("Dispatch server listening on {}", addr);
    listen(addr.as_str(), (), handler::handler)
        .await?
        .await?;

    Ok(())
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct GateConfig {
    pub outer_host: String,
    pub inner_listen_addr: String,
    pub game_server_addr: String,
    pub salt: String,
}

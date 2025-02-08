use serde::Deserialize;

#[derive(Deserialize)]
pub struct GameConfig {
    pub listen_addr: String,
    pub gate_server_addr: String,
}

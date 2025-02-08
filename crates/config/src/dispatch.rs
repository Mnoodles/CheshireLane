use serde::Deserialize;

#[derive(Deserialize)]
pub struct DispatchConfig {
    pub ip: String,
    pub port: u16,
    pub version_path: String,
    pub servers_path: String,
    pub salt: String,
}

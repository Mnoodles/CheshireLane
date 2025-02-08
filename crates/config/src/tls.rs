use serde::Deserialize;

#[derive(Deserialize)]
pub struct TLSConfig {
    pub cert_path: String,
    pub key_path: String,
}

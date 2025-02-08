use serde::Deserialize;

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub username: String,
    pub password: String,
}

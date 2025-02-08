mod database;
mod tls;
mod sdk;
mod dispatch;
mod resource;
mod gate;
mod game;

use std::sync::LazyLock;
use common::toml::TomlConfig;
use serde::Deserialize;

use database::DatabaseConfig;
use tls::TLSConfig;
use sdk::SDKConfig;
use dispatch::DispatchConfig;
use resource::ResourceConfig;
use gate::GateConfig;
use game::GameConfig;

#[derive(Deserialize)]
pub struct Config {
    pub database_config: DatabaseConfig,
    pub resource: ResourceConfig,
    pub tls_config: TLSConfig,
    pub sdk_config: SDKConfig,
    pub dispatch_config: DispatchConfig,
    pub gate_config: GateConfig,
    pub game_config: GameConfig,
}

impl TomlConfig for Config {
    const DEFAULT_TOML: &'static str = include_str!("config.default.toml");
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    Config::load_or_create("config.toml")
});

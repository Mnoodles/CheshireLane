use serde::Deserialize;

#[derive(Deserialize)]
pub struct ResourceConfig {
    pub game_cfg_dir: String,
    pub share_cfg_dir: String,
    pub share_cfg_data_dir: String,
}

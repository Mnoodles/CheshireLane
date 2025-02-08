use serde::Deserialize;

#[derive(Deserialize)]
pub struct SDKConfig {
    pub http_addr: String,
    pub https_addr: String,
    pub code_path: String,
    pub settings_path: String,
    pub error_code_path: String,
}

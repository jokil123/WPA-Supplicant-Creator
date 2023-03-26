use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct NetworkInfo {
    pub ssid: String,
    pub password: String,
}

#[derive(Debug, Clone, Default)]
pub struct ConfigData {
    pub network: NetworkInfo,
    pub country: String,
    pub path: PathBuf,
}

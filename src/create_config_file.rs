use std::fs;

use crate::{data_types::ConfigData, error::CreatorError};

static CONFIG_FILE_TEMPLATE_FALLBACK: &str = include_str!("../wpa_supplicant.conf");

pub fn create_config_file(data: &ConfigData) -> Result<(), CreatorError> {
    // let mut config_file = fs::read_to_string("wpa_supplicant.conf")
    //     .unwrap_or(CONFIG_FILE_TEMPLATE_FALLBACK.to_string());

    let mut config_file = CONFIG_FILE_TEMPLATE_FALLBACK.to_string();

    config_file = config_file.replace("<alpha_2_country>", &data.country);
    config_file = config_file.replace("<network_ssid>", &data.network.ssid);
    config_file = config_file.replace("<network_password>", &data.network.password);

    fs::write(&data.path, config_file)?;

    Ok(())
}

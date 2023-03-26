use std::fs;

use crate::{data_types::ConfigData, error::CreatorError};

pub fn create_config_file(data: &ConfigData) -> Result<(), CreatorError> {
    let mut config_file = fs::read_to_string("wpa_supplicant.conf")
        .map_err(|_| CreatorError::ConfigFileTemplateNotFound)?;

    config_file = config_file.replace("<alpha_2_country>", &data.country);
    config_file = config_file.replace("<network_ssid>", &data.network.ssid);
    config_file = config_file.replace("<network_password>", &data.network.password);

    fs::write(&data.path, config_file)?;

    Ok(())
}

use std::{os::windows::process::CommandExt, process::Command};

use fancy_regex::Regex;
use sysinfo::{DiskExt, RefreshKind, SystemExt};
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

use crate::{
    data_types::{ConfigData, NetworkInfo},
    error::CreatorError,
};

const CREATE_NO_WINDOW: u32 = 0x08000000;
const DETACHED_PROCESS: u32 = 0x00000008;

pub fn get_config_data() -> Result<ConfigData, CreatorError> {
    let sys =
        sysinfo::System::new_with_specifics(RefreshKind::new().with_disks().with_disks_list());
    let disk = sys
        .disks()
        .into_iter()
        .filter(|disk| disk.is_removable() && disk.name().to_ascii_lowercase() == "boot")
        .next()
        .ok_or(CreatorError::NoMatchingDisksFound)?;

    let path = disk.mount_point().to_owned().join("wpa_supplicant.conf");

    let network = get_network_info()?;

    let region: String = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey("Control Panel\\International\\Geo")?
        .get_value("Name")?;

    Ok(ConfigData {
        network,
        country: region,
        path,
    })
}

fn get_network_info() -> Result<NetworkInfo, CreatorError> {
    let output_current = String::from_utf8(
        Command::new("netsh")
            .args(&["wlan", "show", "interfaces"])
            .creation_flags(CREATE_NO_WINDOW)
            .output()?
            .stdout,
    )?;

    let network_state_line = Regex::new(r" +State +: .*\n")?
        .find(&output_current)?
        .ok_or(CreatorError::NoMatchFound)?
        .as_str();

    let network_state = Regex::new(r"(?<=: ).*\n")?
        .find(&network_state_line)?
        .ok_or(CreatorError::NoMatchFound)?
        .as_str()
        .trim();

    if network_state != "connected" {
        return Err(CreatorError::NotConnected);
    }

    let network_ssid_line = Regex::new(r" +SSID +: .*\n")?
        .find(&output_current)?
        .ok_or(CreatorError::NoMatchFound)?
        .as_str();

    let network_ssid = Regex::new(r"(?<=: ).*\n")?
        .find(&network_ssid_line)?
        .ok_or(CreatorError::NoMatchFound)?
        .as_str()
        .trim();

    let output_password = String::from_utf8(
        Command::new("netsh")
            .args(&["wlan", "show", "profile", "FRITZ!Box 7530 CO", "key=clear"])
            .creation_flags(CREATE_NO_WINDOW)
            .output()?
            .stdout,
    )?;

    let network_password_line = Regex::new(r" +Key Content +: .*\n")?
        .find(&output_password)?
        .ok_or(CreatorError::NoMatchFound)?
        .as_str();

    let network_password = Regex::new(r"(?<=: ).*\n")?
        .find(&network_password_line)?
        .ok_or(CreatorError::NoMatchFound)?
        .as_str()
        .trim();

    Ok(NetworkInfo {
        ssid: network_ssid.to_owned(),
        password: network_password.to_owned(),
    })
}

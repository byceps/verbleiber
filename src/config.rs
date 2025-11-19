/*
 * Copyright 2022-2025 Jochen Kupperschmidt
 * License: MIT
 */

use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::Deserialize;

use crate::buttons::Button;
use crate::devices::DeviceName;
use crate::keycodenames::KeyName;
use crate::model::{PartyId, UserId, UserMode};

#[derive(Deserialize)]
pub(crate) struct Config {
    pub reader_input_device: Option<DeviceName>,
    pub button_input_device: DeviceName,

    #[serde(rename = "buttons_to_key_codes")]
    pub buttons_to_key_code_names: HashMap<Button, KeyName>,

    pub sounds_path: PathBuf,
    pub api: ApiConfig,
    pub party: PartyConfig,
    pub single_user: Option<SingleUserConfig>,
}

impl Config {
    pub fn get_user_mode(&self) -> UserMode {
        self.single_user
            .as_ref()
            .and_then(|single_user_config| single_user_config.id.clone())
            .map(UserMode::SingleUser)
            .unwrap_or(UserMode::MultiUser)
    }
}

#[derive(Deserialize)]
pub(crate) struct ApiConfig {
    pub base_url: String,
    pub client_token: String,
    pub tls_verify: bool,
    pub timeout_in_seconds: u64,
}

#[derive(Deserialize)]
pub(crate) struct PartyConfig {
    pub party_id: PartyId,
    pub buttons_to_whereabouts: HashMap<Button, String>,
    pub whereabouts_sounds: HashMap<String, Vec<String>>,
}

#[derive(Deserialize)]
pub(crate) struct SingleUserConfig {
    pub id: Option<UserId>,
}

pub(crate) fn load_config(path: &Path) -> Result<Config> {
    let text = read_to_string(path)?;
    let config: Config = toml::from_str(&text)?;
    Ok(config)
}

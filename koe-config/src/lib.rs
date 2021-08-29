use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    pub google_application_credentials: PathBuf,

    pub discord_client_id: u64,
    pub discord_bot_token: String,
}

pub fn load() -> Result<Config> {
    let config =
        envy::from_env::<Config>().context("Failed to read config from environment variables")?;

    Ok(config)
}

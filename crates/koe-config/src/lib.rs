use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub discord_client_id: u64,
    pub discord_bot_token: String,

    pub voicevox_api_base: String,
    pub redis_url: String,
}

pub fn load() -> Result<Config> {
    let config =
        envy::from_env::<Config>().context("Failed to read config from environment variables")?;

    Ok(config)
}

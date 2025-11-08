use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub discord: DiscordConfig,
    pub voicevox: VoicevoxConfig,
    pub redis: RedisConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DiscordConfig {
    pub client_id: u64,
    pub bot_token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VoicevoxConfig {
    pub api_base: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}

pub async fn load() -> Result<Config> {
    let config_path = std::env::var("KOE_CONFIG").unwrap_or_else(|_| "/etc/koe.yaml".to_string());

    let yaml = tokio::fs::read_to_string(&config_path)
        .await
        .with_context(|| format!("Failed to load config file from {config_path}"))?;

    let config = serde_yaml::from_str::<Config>(&yaml).context("Failed to parse config file")?;

    Ok(config)
}

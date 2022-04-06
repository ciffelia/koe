use anyhow::{Context, Result};
use dashmap::DashMap;
use koe_db::redis;
use koe_speech::SpeechProvider;
use log::info;
use sentry::integrations::anyhow::capture_anyhow;
use serenity::Client;
use songbird::SerenityInit;

mod app_state;
mod audio_queue;
mod command_setup;
mod error;
mod handler;
mod regex;
mod sanitize;
mod songbird_util;
mod voice_preset;

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = sentry::init(());

    run().await.map_err(|err| {
        capture_anyhow(&err);
        err
    })
}

async fn run() -> Result<()> {
    ecs_logger::init();

    let config = koe_config::load()?;
    info!("Config loaded");

    let mut client = Client::builder(config.discord_bot_token)
        .event_handler(handler::Handler)
        .application_id(config.discord_client_id)
        .register_songbird()
        .await
        .context("Failed to build serenity client")?;

    app_state::initialize(
        &client,
        app_state::AppState {
            redis_client: redis::Client::open(config.redis_url)?,
            speech_provider: SpeechProvider::new(config.voicevox_api_base),
            connected_guild_states: DashMap::new(),
        },
    )
    .await;

    info!("Starting client...");
    client.start().await.context("Client error occurred")?;

    Ok(())
}

use crate::connection_status::VoiceConnectionStatusMap;
use crate::voice_client::VoiceClient;
use anyhow::{Context, Result};
use koe_db::redis;
use koe_speech::SpeechProvider;
use log::info;
use sentry::integrations::anyhow::capture_anyhow;
use serenity::Client;
use songbird::SerenityInit;

mod command_setup;
mod connection_status;
mod context_store;
mod error;
mod handler;
mod regex;
mod sanitize;
mod speech;
mod voice_client;

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

    let redis_client = redis::Client::open(config.redis_url)?;
    let speech_provider = SpeechProvider::new(config.google_application_credentials).await?;
    let voice_client = VoiceClient::new();
    let status_map = VoiceConnectionStatusMap::new();

    let mut client = Client::builder(config.discord_bot_token)
        .event_handler(handler::Handler)
        .application_id(config.discord_client_id)
        .register_songbird()
        .await
        .context("Failed to build serenity client")?;

    context_store::insert(&client, redis_client).await;
    context_store::insert(&client, speech_provider).await;
    context_store::insert(&client, voice_client).await;
    context_store::insert(&client, status_map).await;

    info!("Starting client...");
    client.start().await.context("Client error occurred")?;

    Ok(())
}

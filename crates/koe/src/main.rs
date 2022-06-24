use crate::error::report_error;
use anyhow::{Context, Result};
use dashmap::DashMap;
use koe_db::redis;
use koe_speech::{speech::initialize_speakers, voicevox::VoicevoxClient};
use log::info;
use sentry::integrations::anyhow::capture_anyhow;
use serenity::Client;
use songbird::SerenityInit;
use tokio::time::Duration;

mod app_state;
mod command;
mod component_interaction;
mod error;
mod event_handler;
mod message;
mod regex;
mod voice_state;

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
        .event_handler(event_handler::Handler)
        .application_id(config.discord_client_id)
        .register_songbird()
        .await
        .context("Failed to build serenity client")?;

    app_state::initialize(
        &client,
        app_state::AppState {
            redis_client: redis::Client::open(config.redis_url)?,
            voicevox_client: VoicevoxClient::new(config.voicevox_api_base),
            connected_guild_states: DashMap::new(),
        },
    )
    .await;

    {
        let d = client.data.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(3)).await;
            info!("Initializing speakers...");

            let data = d.read().await;
            let state = data.get::<app_state::AppState>().unwrap();

            if let Err(err) = initialize_speakers(&state.voicevox_client).await {
                report_error(err);
            }
        });
    }

    info!("Starting client...");
    client.start().await.context("Client error occurred")?;

    Ok(())
}

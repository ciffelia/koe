use anyhow::{Context, Result};
use dashmap::DashMap;
use log::{error, info};
use serenity::{
    Client,
    model::{gateway::GatewayIntents, id::ApplicationId},
};
use songbird::SerenityInit;
use tokio::time::Duration;
use tts::{speech::initialize_speakers, voicevox::VoicevoxClient};

mod app_state;
mod commands;
mod components;
mod config;
mod db;
mod event_handler;
mod message;
mod tts;
mod voice_call;
mod voice_state;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let config = config::load().await?;
    info!("Config loaded");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(config.discord.bot_token, intents)
        .event_handler(event_handler::Handler)
        .application_id(ApplicationId::new(config.discord.client_id))
        .register_songbird()
        .await
        .context("Failed to build serenity client")?;

    app_state::initialize(
        &client,
        app_state::AppState {
            redis_client: redis::Client::open(config.redis.url)?,
            voicevox_client: VoicevoxClient::new(config.voicevox.api_base),
            connected_guild_states: DashMap::new(),
        },
    )
    .await;

    let d = client.data.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(3)).await;
        info!("Initializing speakers...");

        let data = d.read().await;
        let state = data.get::<app_state::AppState>().unwrap();

        if let Err(err) = initialize_speakers(&state.voicevox_client)
            .await
            .context("Failed to initialize speakers")
        {
            error!("{err:?}");
        }
    });

    info!("Starting client...");
    client.start().await.context("Client error occurred")?;

    Ok(())
}

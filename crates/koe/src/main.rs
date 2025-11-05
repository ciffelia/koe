use anyhow::{Context, Result};
use dashmap::DashMap;
use koe_db::redis;
use koe_speech::{speech::initialize_speakers, voicevox::VoicevoxClient};
use log::{error, info};
use poise::serenity_prelude as serenity;
use serenity::model::{gateway::GatewayIntents, id::ApplicationId};
use songbird::SerenityInit;
use tokio::time::Duration;

mod app_state;
mod commands;
mod component_interaction;
mod event_handler;
mod message;
mod regex;
mod voice_state;

#[tokio::main]
async fn main() -> Result<()> {
    ecs_logger::init();

    let config = koe_config::load().await?;
    info!("Config loaded");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    // Clone voicevox API base before moving config
    let voicevox_api_base = config.voicevox.api_base.clone();

    let state = app_state::AppState {
        redis_client: redis::Client::open(config.redis.url)?,
        voicevox_client: VoicevoxClient::new(config.voicevox.api_base),
        connected_guild_states: DashMap::new(),
    };

    // Initialize speakers asynchronously after a delay
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(3)).await;
        info!("Initializing speakers...");

        let voicevox_client = VoicevoxClient::new(voicevox_api_base);
        if let Err(err) = initialize_speakers(&voicevox_client)
            .await
            .context("Failed to initialize speakers")
        {
            error!("{:?}", err);
        }
    });

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::commands(),
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler::event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(state)
            })
        })
        .build();

    let token = config.discord.bot_token.clone();
    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .application_id(ApplicationId::new(config.discord.client_id))
        .register_songbird()
        .await
        .context("Failed to build client")?;

    info!("Starting client...");
    client.start().await.context("Client error occurred")?;

    Ok(())
}

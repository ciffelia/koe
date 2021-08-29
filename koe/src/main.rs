use crate::voice_client::VoiceClient;
use anyhow::{Context, Result};
use dashmap::DashMap;
use koe_speech::SpeechProvider;
use serenity::model::id::{ChannelId, GuildId};
use serenity::Client;
use songbird::SerenityInit;

mod command;
mod context_store;
mod handler;
mod voice_client;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let config = koe_config::load()?;

    let mut client = Client::builder(config.discord_bot_token)
        .event_handler(handler::Handler)
        .application_id(config.discord_client_id)
        .register_songbird()
        .await
        .context("Failed to build serenity client")?;

    let speech_provider = SpeechProvider::new(config.google_application_credentials).await?;
    context_store::insert(&client, speech_provider).await;

    let voice_client = VoiceClient::new();
    context_store::insert(&client, voice_client).await;

    let bound_text_channel_map = DashMap::<GuildId, ChannelId>::new();
    context_store::insert(&client, bound_text_channel_map).await;

    client.start().await.context("Client error occurred")?;

    Ok(())
}

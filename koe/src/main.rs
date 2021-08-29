use crate::context_store::{BoundTextChannelMapStore, SpeechProviderStore, VoiceClientStore};
use crate::voice_client::VoiceClient;
use anyhow::{Context, Result};
use dashmap::DashMap;
use koe_speech::SpeechProvider;
use serenity::Client;
use songbird::SerenityInit;
use std::sync::Arc;

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

    {
        let mut data = client.data.write().await;

        let speech_provider = SpeechProvider::new(config.google_application_credentials).await?;
        data.insert::<SpeechProviderStore>(Arc::new(speech_provider));

        let voice_client = VoiceClient::new();
        data.insert::<VoiceClientStore>(Arc::new(voice_client));

        let bound_text_channel_map = DashMap::new();
        data.insert::<BoundTextChannelMapStore>(Arc::new(bound_text_channel_map));
    }

    client.start().await.context("Client error occurred")?;

    Ok(())
}

use crate::voice_client::VoiceClient;
use anyhow::{anyhow, Result};
use dashmap::DashMap;
use koe_speech::SpeechProvider;
use serenity::client::Context;
use serenity::model::id::{ChannelId, GuildId};
use serenity::prelude::TypeMapKey;
use songbird::Songbird;
use std::sync::Arc;

pub struct SpeechProviderStore;

impl TypeMapKey for SpeechProviderStore {
    type Value = Arc<SpeechProvider>;
}

pub struct BoundTextChannelMapStore;

impl TypeMapKey for BoundTextChannelMapStore {
    type Value = Arc<DashMap<GuildId, ChannelId>>;
}

pub struct VoiceClientStore;

impl TypeMapKey for VoiceClientStore {
    type Value = Arc<VoiceClient>;
}

pub async fn extract_songbird(ctx: &Context) -> Result<Arc<Songbird>> {
    let songbird = songbird::get(ctx)
        .await
        .ok_or_else(|| anyhow!("Songbird voice client is not initialized"))?;

    Ok(songbird)
}

pub async fn extract_speech_provider(ctx: &Context) -> Result<Arc<SpeechProvider>> {
    let data = ctx.data.read().await;

    let speech_provider = data
        .get::<SpeechProviderStore>()
        .ok_or_else(|| anyhow!("SpeechProviderStore is not initialized"))?;

    Ok(speech_provider.clone())
}

pub async fn extract_voice_client(ctx: &Context) -> Result<Arc<VoiceClient>> {
    let data = ctx.data.read().await;

    let voice_client = data
        .get::<VoiceClientStore>()
        .ok_or_else(|| anyhow!("VoiceClientStore is not initialized"))?;

    Ok(voice_client.clone())
}

pub async fn extract_bound_text_channel_map(
    ctx: &Context,
) -> Result<Arc<DashMap<GuildId, ChannelId>>> {
    let data = ctx.data.read().await;

    let bound_text_channel_map = data
        .get::<BoundTextChannelMapStore>()
        .ok_or_else(|| anyhow!("BoundTextChannelMapStore is not initialized"))?;

    Ok(bound_text_channel_map.clone())
}

use std::sync::Arc;

use anyhow::{Context as _, Result};
use dashmap::DashMap;
use koe_db::redis;
use koe_speech::voicevox::VoicevoxClient;
use serenity::{
    client::{Client, Context},
    model::{
        channel::Message,
        id::{ChannelId, GuildId},
    },
    prelude::TypeMapKey,
};

pub struct AppState {
    pub redis_client: redis::Client,
    pub voicevox_client: VoicevoxClient,
    pub connected_guild_states: DashMap<GuildId, ConnectedGuildState>,
}

pub struct ConnectedGuildState {
    pub bound_text_channel: ChannelId,
    pub last_message_read: Option<Message>,
}

impl TypeMapKey for AppState {
    type Value = Arc<AppState>;
}

pub async fn initialize(client: &Client, state: AppState) {
    let mut data = client.data.write().await;
    data.insert::<AppState>(Arc::new(state));
}

pub async fn get(ctx: &Context) -> Result<Arc<AppState>> {
    let data = ctx.data.read().await;

    let state_ref = data
        .get::<AppState>()
        .context("AppState is not initialized")?;

    Ok(state_ref.clone())
}

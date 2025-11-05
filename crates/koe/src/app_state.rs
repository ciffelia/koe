use dashmap::DashMap;
use koe_db::redis;
use koe_speech::voicevox::VoicevoxClient;
use serenity::model::{
    channel::Message,
    id::{ChannelId, GuildId},
};

/// Application state shared across the bot
pub struct AppState {
    pub redis_client: redis::Client,
    pub voicevox_client: VoicevoxClient,
    pub connected_guild_states: DashMap<GuildId, ConnectedGuildState>,
}

/// State for guilds where the bot is connected to a voice channel
pub struct ConnectedGuildState {
    pub bound_text_channel: ChannelId,
    pub last_message_read: Option<Message>,
}

/// Type alias for Poise context
pub type Context<'a> = poise::Context<'a, AppState, anyhow::Error>;

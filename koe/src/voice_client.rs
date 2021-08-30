use crate::context_store;
use anyhow::Result;
use serenity::client::Context;
use songbird::id::{ChannelId, GuildId};
use songbird::join::Join;
use songbird::Call;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct VoiceClient;

impl VoiceClient {
    pub fn new() -> Self {
        Self
    }

    pub async fn join(
        &self,
        ctx: &Context,
        guild_id: impl Into<GuildId>,
        channel_id: impl Into<ChannelId>,
    ) -> Result<Arc<Mutex<Call>>> {
        let manager = context_store::extract_songbird(ctx).await?;
        let guild_id = guild_id.into();
        let channel_id = channel_id.into();

        let call = manager.get_or_insert(guild_id);

        // See https://docs.rs/songbird/latest/songbird/struct.Call.html#method.join
        let join_res: Result<Join> = {
            let mut handler = call.lock().await;
            handler.deafen(true).await?;

            let join = handler.join(channel_id).await?;

            Ok(join)
        };
        let join = join_res?;
        join.await?;

        Ok(call)
    }

    pub async fn leave(&self, ctx: &Context, guild_id: impl Into<GuildId>) -> Result<()> {
        let manager = context_store::extract_songbird(ctx).await?;
        let guild_id = guild_id.into();

        manager.remove(guild_id).await?;

        Ok(())
    }

    pub async fn is_connected(&self, ctx: &Context, guild_id: impl Into<GuildId>) -> Result<bool> {
        let manager = context_store::extract_songbird(ctx).await?;
        let guild_id = guild_id.into();

        let is_connected = manager.get(guild_id).is_some();

        Ok(is_connected)
    }
}

use std::sync::Arc;

use anyhow::{Context as _, Result, anyhow};
use serenity::client::Context;
use songbird::{
    Call, Songbird,
    id::{ChannelId, GuildId},
};
use tokio::sync::Mutex;

pub async fn join_deaf(
    ctx: &Context,
    guild_id: impl Into<GuildId>,
    channel_id: impl Into<ChannelId>,
) -> Result<()> {
    let manager = extract_songbird(ctx).await?;
    let guild_id = guild_id.into();
    let channel_id = channel_id.into();

    let call = manager.get_or_insert(guild_id);

    // To join a voice channel in a deafened state, you need to use `Call::join` instead of `Songbird::join`.
    // `Call::join` requires two stages of await, and the Mutex needs to be released before the second await.
    // For more details, see https://docs.rs/songbird/latest/songbird/struct.Call.html#method.join
    let stage_1 = {
        let mut handler = call.lock().await;
        handler.deafen(true).await?;
        handler.join(channel_id).await?
    };
    stage_1.await?;

    Ok(())
}

pub async fn leave(ctx: &Context, guild_id: impl Into<GuildId>) -> Result<()> {
    let manager = extract_songbird(ctx).await?;
    let guild_id = guild_id.into();

    manager.remove(guild_id).await?;

    Ok(())
}

pub async fn is_connected(ctx: &Context, guild_id: impl Into<GuildId>) -> Result<bool> {
    let manager = extract_songbird(ctx).await?;
    let guild_id = guild_id.into();

    let is_connected = manager.get(guild_id).is_some();

    Ok(is_connected)
}

pub async fn enqueue(ctx: &Context, guild_id: impl Into<GuildId>, audio: Vec<u8>) -> Result<()> {
    let manager = extract_songbird(ctx).await?;
    let call = get_call(manager, guild_id).await?;

    let mut handler = call.lock().await;
    handler.enqueue_input(audio.into()).await;

    Ok(())
}

pub async fn skip(ctx: &Context, guild_id: impl Into<GuildId>) -> Result<()> {
    let manager = extract_songbird(ctx).await?;
    let call = get_call(manager, guild_id).await?;

    let handler = call.lock().await;
    let current_track = handler.queue().current();

    if let Some(track) = current_track {
        track.stop().context("Failed to stop current track")?;
    }

    Ok(())
}

async fn extract_songbird(ctx: &Context) -> Result<Arc<Songbird>> {
    let songbird = songbird::get(ctx)
        .await
        .ok_or_else(|| anyhow!("Songbird voice client is not initialized"))?;

    Ok(songbird)
}

async fn get_call(
    manager: Arc<Songbird>,
    guild_id: impl Into<GuildId>,
) -> Result<Arc<Mutex<Call>>> {
    let guild_id = guild_id.into();

    let call = manager
        .get(guild_id)
        .ok_or_else(|| anyhow!("Failed to retrieve call for guild {}", guild_id))?;

    Ok(call)
}

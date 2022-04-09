use anyhow::{anyhow, Context as _, Result};
use serenity::client::Context;
use songbird::{
    id::{ChannelId, GuildId},
    input::{Codec, Container, Input, Reader},
    join::Join,
    Call, Songbird,
};
use std::sync::Arc;
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

    // Call::joinを実行するには、2段階のawaitが必要
    // 詳細は https://docs.rs/songbird/latest/songbird/struct.Call.html#method.join
    let join_res: Result<Join> = {
        let mut handler = call.lock().await;
        handler.deafen(true).await?;

        let join = handler.join(channel_id).await?;

        Ok(join)
    };
    let join = join_res?;
    join.await?;

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

pub async fn enqueue(
    ctx: &Context,
    guild_id: impl Into<GuildId>,
    raw_audio: Vec<u8>,
) -> Result<()> {
    let manager = extract_songbird(ctx).await?;
    let call = get_call(manager, guild_id).await?;

    let mut handler = call.lock().await;
    handler.enqueue_source(Input::new(
        false,
        Reader::from_memory(raw_audio),
        Codec::Pcm,
        Container::Raw,
        None,
    ));

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

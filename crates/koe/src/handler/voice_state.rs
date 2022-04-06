use crate::{app_state, songbird_util};
use anyhow::{Context as _, Result};
use log::debug;
use serenity::{
    client::Context,
    model::{
        id::{ChannelId, GuildId, UserId},
        voice::VoiceState,
    },
};

pub async fn handle_voice_state_update(
    ctx: &Context,
    guild_id: Option<GuildId>,
    old_voice_state: Option<VoiceState>,
    new_voice_state: VoiceState,
) -> Result<()> {
    let guild_id = match guild_id {
        Some(id) => id,
        None => return Ok(()),
    };

    let current_voice_channel_id = match get_current_voice_channel_id(ctx, guild_id).await? {
        Some(id) => id,
        None => return Ok(()),
    };

    if old_voice_state.as_ref().and_then(|state| state.channel_id) != Some(current_voice_channel_id)
        && new_voice_state.channel_id == Some(current_voice_channel_id)
    {
        handle_user_join(ctx, guild_id, new_voice_state.user_id).await?;
    }

    if old_voice_state.and_then(|state| state.channel_id) == Some(current_voice_channel_id)
        && new_voice_state.channel_id != Some(current_voice_channel_id)
    {
        handle_user_leave(ctx, guild_id, new_voice_state.user_id).await?;
    }

    let current_channel_user_list =
        list_users_in_voice_channel(ctx, guild_id, current_voice_channel_id)
            .await
            .context("Failed to count the number of users in the bot's channel")?;

    // VCのメンバーがKoe自身のみになった場合は抜ける
    if current_channel_user_list.len() == 1 {
        songbird_util::leave(ctx, guild_id)
            .await
            .context("Failed to leave voice channel")?;

        let state = app_state::get(ctx).await?;
        state.connected_guild_states.remove(&guild_id);

        debug!("Automatically disconnected in guild {}", guild_id.as_u64());
    }

    Ok(())
}

async fn handle_user_join(ctx: &Context, guild_id: GuildId, user_id: UserId) -> Result<()> {
    let state = app_state::get(ctx).await?;
    let mut guild_state = match state.connected_guild_states.get_mut(&guild_id) {
        Some(status) => status,
        None => return Ok(()),
    };

    let available_preset_ids = state.speech_provider.list_preset_ids().await?;
    let preset_id = guild_state
        .voice_preset_registry
        .pick_least_used_preset(&available_preset_ids)
        .await?;

    guild_state
        .voice_preset_registry
        .insert(user_id, preset_id)?;

    Ok(())
}

async fn handle_user_leave(ctx: &Context, guild_id: GuildId, user_id: UserId) -> Result<()> {
    let state = app_state::get(ctx).await?;
    let mut guild_state = match state.connected_guild_states.get_mut(&guild_id) {
        Some(status) => status,
        None => return Ok(()),
    };

    guild_state.voice_preset_registry.remove(user_id)?;

    Ok(())
}

async fn get_current_voice_channel_id(
    ctx: &Context,
    guild_id: GuildId,
) -> Result<Option<ChannelId>> {
    let current_user_id = ctx.cache.current_user_id().await;

    let voice_state_map = guild_id
        .to_guild_cached(&ctx.cache)
        .await
        .context("Failed to find guild in the cache")?
        .voice_states;

    let current_voice_state = match voice_state_map.get(&current_user_id) {
        Some(state) => state,
        None => return Ok(None),
    };

    Ok(current_voice_state.channel_id)
}

async fn list_users_in_voice_channel(
    ctx: &Context,
    guild_id: GuildId,
    channel_id: ChannelId,
) -> Result<Vec<UserId>> {
    let voice_state_map = guild_id
        .to_guild_cached(&ctx.cache)
        .await
        .context("Failed to find guild in the cache")?
        .voice_states;

    let list = voice_state_map
        .into_iter()
        .filter(|(_, state)| state.channel_id == Some(channel_id))
        .map(|(_, state)| state.user_id)
        .collect();

    Ok(list)
}

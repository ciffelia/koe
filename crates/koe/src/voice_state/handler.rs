use crate::app_state;
use anyhow::{Context as _, Result};
use log::debug;
use serenity::{
    client::Context,
    model::id::{ChannelId, GuildId, UserId},
};

pub async fn handle_update(ctx: &Context, guild_id: Option<GuildId>) -> Result<()> {
    let guild_id = match guild_id {
        Some(id) => id,
        None => return Ok(()),
    };

    let current_voice_channel_id = match get_current_voice_channel_id(ctx, guild_id)? {
        Some(id) => id,
        None => return Ok(()),
    };

    let current_channel_user_list =
        list_users_in_voice_channel(ctx, guild_id, current_voice_channel_id)
            .context("Failed to count the number of users in the bot's channel")?;

    // VCのメンバーがKoe自身のみになった場合は抜ける
    if current_channel_user_list.len() == 1 {
        koe_call::leave(ctx, guild_id)
            .await
            .context("Failed to leave voice channel")?;

        let state = app_state::get(ctx).await?;
        state.connected_guild_states.remove(&guild_id);

        debug!("Automatically disconnected in guild {}", guild_id.as_u64());
    }

    Ok(())
}

fn get_current_voice_channel_id(ctx: &Context, guild_id: GuildId) -> Result<Option<ChannelId>> {
    let current_user_id = ctx.cache.current_user_id();

    let voice_state_map = guild_id
        .to_guild_cached(&ctx.cache)
        .context("Failed to find guild in the cache")?
        .voice_states;

    let current_voice_state = match voice_state_map.get(&current_user_id) {
        Some(state) => state,
        None => return Ok(None),
    };

    Ok(current_voice_state.channel_id)
}

fn list_users_in_voice_channel(
    ctx: &Context,
    guild_id: GuildId,
    channel_id: ChannelId,
) -> Result<Vec<UserId>> {
    let voice_state_map = guild_id
        .to_guild_cached(&ctx.cache)
        .context("Failed to find guild in the cache")?
        .voice_states;

    let list = voice_state_map
        .into_iter()
        .filter(|(_, state)| state.channel_id == Some(channel_id))
        .map(|(_, state)| state.user_id)
        .collect();

    Ok(list)
}

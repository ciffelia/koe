use crate::{app_state, songbird_util};
use anyhow::{Context as _, Result};
use log::debug;
use serenity::{
    client::Context,
    model::{id::GuildId, voice::VoiceState},
};

pub async fn handle_voice_state_update(ctx: &Context, guild_id: Option<GuildId>) -> Result<()> {
    let guild_id = match guild_id {
        Some(id) => id,
        None => return Ok(()),
    };

    let voice_state_list = list_current_channel_voice_state(ctx, guild_id)
        .await
        .context("Failed to count the number of users in the bot's channel")?;

    // VCのメンバーがKoe自身のみになった場合は抜ける
    if voice_state_list.len() == 1 {
        songbird_util::leave(ctx, guild_id)
            .await
            .context("Failed to leave voice channel")?;

        let state = app_state::get(ctx).await?;
        state.connected_guild_states.remove(&guild_id);

        debug!("Automatically disconnected in guild {}", guild_id.as_u64());
    }

    Ok(())
}

/// Koeが参加しているVCの[`VoiceState`]を返す
/// KoeがVCに参加していない場合は空の[`Vec`]を返す
pub async fn list_current_channel_voice_state(
    ctx: &Context,
    guild_id: GuildId,
) -> Result<Vec<VoiceState>> {
    let current_user_id = ctx.cache.current_user_id().await;

    let voice_state_map = guild_id
        .to_guild_cached(&ctx.cache)
        .await
        .context("Failed to find guild in the cache")?
        .voice_states;

    let current_voice_state = match voice_state_map.get(&current_user_id) {
        Some(state) => state,
        None => return Ok(vec![]),
    };
    let current_voice_channel_id = match current_voice_state.channel_id {
        Some(id) => id,
        None => return Ok(vec![]),
    };

    let list = voice_state_map
        .into_iter()
        .filter(|(_, state)| state.channel_id == Some(current_voice_channel_id))
        .map(|(_, state)| state)
        .collect();

    Ok(list)
}

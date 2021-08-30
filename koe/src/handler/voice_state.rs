use crate::context_store;
use crate::status::VoiceConnectionStatusMap;
use crate::voice_client::VoiceClient;
use anyhow::{Context as _, Result};
use log::debug;
use serenity::{client::Context, model::id::GuildId};

pub async fn handle_voice_state_update(ctx: &Context, guild_id: Option<GuildId>) -> Result<()> {
    let guild_id = match guild_id {
        Some(id) => id,
        None => return Ok(()),
    };

    let count = count_user_in_current_channel(ctx, guild_id)
        .await
        .context("Failed to count the number of users in the bot's channel")?;

    if count == 1 {
        let voice_client = context_store::extract::<VoiceClient>(ctx).await?;
        voice_client
            .leave(ctx, guild_id)
            .await
            .context("Failed to leave voice channel")?;

        let status_map = context_store::extract::<VoiceConnectionStatusMap>(ctx).await?;
        status_map.remove(&guild_id);

        debug!("Automatically disconnected in guild {}", guild_id.as_u64());
    }

    Ok(())
}

/// Koeが入っているVCに接続しているメンバーの数を返す（Koe自身を含む）。
/// KoeがVCに接続していない場合は、`0`を返す。
async fn count_user_in_current_channel(ctx: &Context, guild_id: GuildId) -> Result<usize> {
    let current_user_id = ctx.cache.current_user_id().await;

    let voice_state_map = guild_id
        .to_guild_cached(&ctx.cache)
        .await
        .context("Failed to find guild in the cache")?
        .voice_states;

    let current_voice_state = match voice_state_map.get(&current_user_id) {
        Some(state) => state,
        None => return Ok(0),
    };
    let current_voice_channel_id = match current_voice_state.channel_id {
        Some(id) => id,
        None => return Ok(0),
    };

    let count = voice_state_map
        .iter()
        .filter(|(_, state)| state.channel_id == Some(current_voice_channel_id))
        .count();

    Ok(count)
}

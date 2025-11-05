use anyhow::{Context as _, Result};
use serenity::model::id::{ChannelId, GuildId, UserId};

use crate::app_state::{self, ConnectedGuildState};

/// ボイスチャンネルに接続し、読み上げを開始
#[poise::command(slash_command, aliases("kjoin"), guild_only)]
pub async fn join(ctx: app_state::Context<'_>) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();
    let user_id = ctx.author().id;
    let text_channel_id = ctx.channel_id();

    let voice_channel_id = match get_user_voice_channel(&ctx, &guild_id, &user_id)? {
        Some(channel) => channel,
        None => {
            ctx.say("ボイスチャンネルに接続してから `/join` を送信してください。")
                .await?;
            return Ok(());
        }
    };

    koe_call::join_deaf(ctx.serenity_context(), guild_id, voice_channel_id).await?;

    let state = ctx.data();
    state.connected_guild_states.insert(
        guild_id,
        ConnectedGuildState {
            bound_text_channel: text_channel_id,
            last_message_read: None,
        },
    );

    ctx.say("接続しました。").await?;
    Ok(())
}

fn get_user_voice_channel(
    ctx: &app_state::Context<'_>,
    guild_id: &GuildId,
    user_id: &UserId,
) -> Result<Option<ChannelId>> {
    let guild = guild_id
        .to_guild_cached(&ctx.serenity_context().cache)
        .context("Failed to find guild in the cache")?;

    let channel_id = guild
        .voice_states
        .get(user_id)
        .and_then(|voice_state| voice_state.channel_id);

    Ok(channel_id)
}

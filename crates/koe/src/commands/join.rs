use anyhow::{Context as _, Result};
use serenity::{
    builder::CreateCommand,
    client::Context,
    model::{
        application::CommandInteraction,
        id::{ChannelId, GuildId, UserId},
    },
};

use super::respond_text;
use crate::app_state;

pub fn commands() -> Vec<CreateCommand> {
    vec![
        CreateCommand::new("join").description("ボイスチャンネルに接続し、読み上げを開始"),
        CreateCommand::new("kjoin").description("ボイスチャンネルに接続し、読み上げを開始"),
    ]
}

pub fn matches(cmd: &CommandInteraction) -> bool {
    matches!(cmd.data.name.as_str(), "join" | "kjoin")
}

pub async fn handle(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    let Some(guild_id) = cmd.guild_id else {
        respond_text(ctx, cmd, "`/join`, `/kjoin` はサーバー内でのみ使えます。").await?;
        return Ok(());
    };
    let user_id = cmd.user.id;
    let text_channel_id = cmd.channel_id;

    let Some(voice_channel_id) = get_user_voice_channel(ctx, guild_id, user_id)? else {
        respond_text(
            ctx,
            cmd,
            "ボイスチャンネルに接続してから `/join` を送信してください。",
        )
        .await?;
        return Ok(());
    };

    koe_call::join_deaf(ctx, guild_id, voice_channel_id).await?;

    let state = app_state::get(ctx).await?;
    state.connected_guild_states.insert(
        guild_id,
        app_state::ConnectedGuildState {
            bound_text_channel: text_channel_id,
            last_message_read: None,
        },
    );

    respond_text(ctx, cmd, "接続しました。").await?;
    Ok(())
}

/// Helper function to get the voice channel a user is currently in
fn get_user_voice_channel(
    ctx: &Context,
    guild_id: GuildId,
    user_id: UserId,
) -> Result<Option<ChannelId>> {
    let guild = guild_id
        .to_guild_cached(&ctx.cache)
        .context("Failed to find guild in the cache")?;

    let channel_id = guild
        .voice_states
        .get(&user_id)
        .and_then(|voice_state| voice_state.channel_id);

    Ok(channel_id)
}

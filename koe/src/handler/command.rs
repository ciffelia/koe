use crate::context_store;
use crate::status::{VoiceConnectionStatus, VoiceConnectionStatusMap};
use crate::voice_client::VoiceClient;
use anyhow::{Context as _, Result};
use log::error;
use serenity::client::Context;
use serenity::model::{
    id::{ChannelId, GuildId, UserId},
    interactions::{application_command::ApplicationCommandInteraction, InteractionResponseType},
};

pub async fn handle_command(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<()> {
    let response_text = execute_command(ctx, command).await;

    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(response_text))
        })
        .await
        .context("Failed to create interaction response")?;

    Ok(())
}

async fn execute_command(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
    let res = match command.data.name.as_str() {
        "join" => handle_join(ctx, command).await,
        "leave" => handle_leave(ctx, command).await,
        _ => Ok("Error: unknown command".to_string()),
    };

    match res {
        Ok(message) => message,
        Err(err) => {
            error!("Error while executing command: {}", err);
            "内部エラーが発生しました。".to_string()
        }
    }
}

async fn handle_join(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<String> {
    let guild_id = match command.guild_id {
        Some(id) => id,
        None => return Ok("`/join` はサーバー内でのみ使えます。".to_string()),
    };
    let user_id = command.user.id;
    let text_channel_id = command.channel_id;

    let voice_channel_id = match get_user_voice_channel(ctx, &guild_id, &user_id).await? {
        Some(channel) => channel,
        None => {
            return Ok("あなたはボイスチャンネルに接続していません。".to_string());
        }
    };

    let voice_client = context_store::extract::<VoiceClient>(ctx).await?;
    voice_client.join(ctx, guild_id, voice_channel_id).await?;

    let status_map = context_store::extract::<VoiceConnectionStatusMap>(ctx).await?;
    status_map.insert(
        guild_id,
        VoiceConnectionStatus {
            bound_text_channel: text_channel_id,
        },
    );

    Ok("接続しました。".to_string())
}

async fn handle_leave(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<String> {
    let guild_id = match command.guild_id {
        Some(id) => id,
        None => return Ok("`/leave` はサーバー内でのみ使えます。".to_string()),
    };

    let voice_client = context_store::extract::<VoiceClient>(ctx).await?;

    if !voice_client.is_connected(ctx, guild_id).await? {
        return Ok("どのボイスチャンネルにも接続していません。".to_string());
    }

    voice_client.leave(ctx, guild_id).await?;

    let status_map = context_store::extract::<VoiceConnectionStatusMap>(ctx).await?;
    status_map.remove(&guild_id);

    Ok("切断しました。".to_string())
}

pub async fn get_user_voice_channel(
    ctx: &Context,
    guild_id: &GuildId,
    user_id: &UserId,
) -> Result<Option<ChannelId>> {
    let guild = guild_id
        .to_guild_cached(&ctx.cache)
        .await
        .context("Failed to find guild in the cache")?;

    let channel_id = guild
        .voice_states
        .get(user_id)
        .and_then(|voice_state| voice_state.channel_id);

    Ok(channel_id)
}

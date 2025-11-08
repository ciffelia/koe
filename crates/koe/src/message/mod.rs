mod read;

use anyhow::{Context as _, Result, anyhow};
use koe_db::voice::GetOption;
use koe_speech::speech::{SpeechRequest, list_preset_ids, make_speech};
use log::trace;
use rand::seq::IndexedRandom;
use serenity::{client::Context, model::channel::Message};

use crate::app_state;

pub async fn handle(ctx: &Context, msg: Message) -> Result<()> {
    let Some(guild_id) = msg.guild_id else {
        return Ok(());
    };

    if !koe_call::is_connected(ctx, guild_id).await? {
        return Ok(());
    }

    let state = app_state::get(ctx).await?;
    let mut guild_state = state
        .connected_guild_states
        .get_mut(&guild_id)
        .with_context(|| format!("Guild state not found for guild {guild_id}"))?;

    if guild_state.bound_text_channel != msg.channel_id {
        return Ok(());
    }

    // Skip message from Koe itself
    if msg.author.id == ctx.cache.current_user().id {
        return Ok(());
    }

    // Skip message that starts with semicolon
    if msg.content.starts_with(';') {
        return Ok(());
    }

    let mut conn = state
        .redis_client
        .get_multiplexed_async_connection()
        .await?;

    let text = read::build_read_text(
        ctx,
        &mut conn,
        guild_id,
        &msg,
        guild_state.last_message_read.as_ref(),
    )
    .await?;
    trace!("Built text: {:?}", &text);

    if text.is_empty() {
        trace!("Text is empty");
        return Ok(());
    }

    let available_preset_ids = list_preset_ids(&state.voicevox_client).await?;
    let fallback_preset_id = available_preset_ids
        .choose(&mut rand::rng())
        .ok_or_else(|| anyhow!("No presets available"))?
        .into();
    let preset_id = koe_db::voice::get(
        &mut conn,
        GetOption {
            guild_id: guild_id.into(),
            user_id: msg.author.id.into(),
            fallback: fallback_preset_id,
        },
    )
    .await?
    .into();

    let audio = make_speech(&state.voicevox_client, SpeechRequest { text, preset_id })
        .await
        .context("Failed to execute Text-to-Speech")?;

    koe_call::enqueue(ctx, guild_id, audio).await?;

    guild_state.last_message_read = Some(msg);

    Ok(())
}

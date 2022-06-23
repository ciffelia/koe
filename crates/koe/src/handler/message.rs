use crate::app_state;
use crate::regex::{custom_emoji_regex, url_regex};
use aho_corasick::{AhoCorasickBuilder, MatchKind};
use anyhow::{anyhow, Context as _, Result};
use chrono::Duration;
use discord_md::generate::{ToMarkdownString, ToMarkdownStringOption};
use koe_db::{dict::GetAllOption, redis, voice::GetOption};
use koe_speech::SpeechRequest;
use log::trace;
use rand::seq::SliceRandom;
use serenity::{
    client::Context,
    model::{channel::Message, id::GuildId},
    utils::ContentSafeOptions,
};

pub async fn handle_message(ctx: &Context, msg: Message) -> Result<()> {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Ok(()),
    };

    if !koe_call::is_connected(ctx, guild_id).await? {
        return Ok(());
    }

    let state = app_state::get(ctx).await?;
    let mut guild_state = match state.connected_guild_states.get_mut(&guild_id) {
        Some(status) => status,
        None => return Ok(()),
    };

    if guild_state.bound_text_channel != msg.channel_id {
        return Ok(());
    }

    // Skip message from Koe itself
    if msg.author.id == ctx.cache.current_user_id().await {
        return Ok(());
    }

    // Skip message that starts with semicolon
    if msg.content.starts_with(';') {
        return Ok(());
    }

    let mut conn = state.redis_client.get_async_connection().await?;

    let text = build_read_text(
        ctx,
        &mut conn,
        guild_id,
        &msg,
        &guild_state.last_message_read,
    )
    .await?;
    trace!("Built text: {:?}", &text);

    if text.is_empty() {
        trace!("Text is empty");
        return Ok(());
    }

    let available_preset_ids = state.speech_provider.list_preset_ids().await?;
    let fallback_preset_id = available_preset_ids
        .choose(&mut rand::thread_rng())
        .ok_or_else(|| anyhow!("No presets available"))?
        .into();
    let preset_id = koe_db::voice::get(
        &mut conn,
        GetOption {
            guild_id: guild_id.to_string(),
            user_id: msg.author.id.to_string(),
            fallback: fallback_preset_id,
        },
    )
    .await?
    .into();

    let encoded_audio = state
        .speech_provider
        .make_speech(SpeechRequest { text, preset_id })
        .await
        .context("Failed to execute Text-to-Speech")?;
    let raw_audio = encoded_audio.decode().await?.into();

    koe_call::enqueue(ctx, guild_id, raw_audio).await?;

    guild_state.last_message_read = Some(msg);

    Ok(())
}

async fn build_read_text(
    ctx: &Context,
    conn: &mut redis::aio::Connection,
    guild_id: GuildId,
    msg: &Message,
    last_msg: &Option<Message>,
) -> Result<String> {
    let author_name = build_author_name(ctx, msg).await;

    let content = replace_entities(ctx, guild_id, &msg.content).await;
    let content = replace_custom_emojis(&content);
    let content = discord_md::parse(&content).to_markdown_string(&ToMarkdownStringOption {
        omit_format: true,
        omit_spoiler: true,
    });
    let content = remove_url(&content);

    let text = if should_read_author_name(msg, last_msg) {
        format!("{}。{}", author_name, content)
    } else {
        content
    };

    let text = replace_words_on_dict(conn, guild_id, &text).await?;

    // 文字数を60文字に制限
    if text.chars().count() > 60 {
        Ok(text.chars().take(60 - 5).collect::<String>() + "、以下 略")
    } else {
        Ok(text)
    }
}

fn should_read_author_name(msg: &Message, last_msg: &Option<Message>) -> bool {
    let last_msg = match last_msg {
        Some(msg) => msg,
        None => return true,
    };

    msg.author != last_msg.author || (msg.timestamp - last_msg.timestamp) > Duration::seconds(10)
}

async fn build_author_name(ctx: &Context, msg: &Message) -> String {
    msg.author_nick(&ctx.http)
        .await
        .unwrap_or_else(|| msg.author.name.clone())
}

/// ID表記されたメンションやチャンネル名を読める形に書き換える
async fn replace_entities(ctx: &Context, guild_id: GuildId, text: &str) -> String {
    let options = ContentSafeOptions::new()
        .clean_channel(true)
        .clean_role(true)
        .clean_user(true)
        .show_discriminator(false)
        .display_as_member_from(guild_id)
        .clean_here(false)
        .clean_everyone(false);

    serenity::utils::content_safe(&ctx.cache, &text, &options).await
}

/// カスタム絵文字を読める形に置き換える
fn replace_custom_emojis(text: &str) -> String {
    custom_emoji_regex().replace_all(text, "$1").into()
}

async fn replace_words_on_dict(
    conn: &mut redis::aio::Connection,
    guild_id: GuildId,
    text: &str,
) -> Result<String> {
    let dict = koe_db::dict::get_all(
        conn,
        GetAllOption {
            guild_id: guild_id.to_string(),
        },
    )
    .await?;

    let dict_list = dict.into_iter().collect::<Vec<_>>();
    let word_list = dict_list.iter().map(|(word, _)| word).collect::<Vec<_>>();
    let read_as_list = dict_list
        .iter()
        .map(|(_, read_as)| read_as)
        .collect::<Vec<_>>();

    let ac = AhoCorasickBuilder::new()
        .match_kind(MatchKind::LeftmostLongest)
        .build(word_list);

    Ok(ac.replace_all(text, &read_as_list))
}

/// メッセージのURLを除去
fn remove_url(text: &str) -> String {
    url_regex().replace_all(text, "、").into()
}

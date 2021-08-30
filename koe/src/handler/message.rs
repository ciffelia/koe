use crate::context_store;
use crate::regex::url_regex;
use crate::status::VoiceConnectionStatusMap;
use crate::voice_client::VoiceClient;
use anyhow::Result;
use chrono::Duration;
use serenity::{client::Context, model::channel::Message};

pub async fn handle_message(ctx: &Context, msg: Message) -> Result<()> {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Ok(()),
    };

    let voice_client = context_store::extract::<VoiceClient>(ctx).await?;
    if !voice_client.is_connected(ctx, guild_id).await? {
        return Ok(());
    }

    let status_map = context_store::extract::<VoiceConnectionStatusMap>(ctx).await?;
    let mut status = match status_map.get_mut(&guild_id) {
        Some(status) => status,
        None => return Ok(()),
    };

    if status.bound_text_channel == msg.channel_id {
        let text = build_read_text(ctx, &msg, &status.last_message_read).await;
        status.speech_queue.push(text)?;
        status.last_message_read = Some(msg);
    }

    Ok(())
}

async fn build_read_text(ctx: &Context, msg: &Message, last_msg: &Option<Message>) -> String {
    let mut text = String::new();

    if should_read_author_name(msg, last_msg) {
        let author_name = msg
            .author_nick(&ctx.http)
            .await
            .unwrap_or_else(|| msg.author.name.clone());

        text.push_str(&author_name);
        text.push('。');
    }
    text.push_str(&msg.content);

    text = url_regex().replace_all(&text, "、").into();

    // 文字数を60文字に制限
    if text.chars().count() > 60 {
        text.chars().take(60 - 5).collect::<String>() + "、以下 略"
    } else {
        text
    }
}

fn should_read_author_name(msg: &Message, last_msg: &Option<Message>) -> bool {
    let last_msg = match last_msg {
        Some(msg) => msg,
        None => return true,
    };

    msg.author != last_msg.author || (msg.timestamp - last_msg.timestamp) > Duration::seconds(10)
}

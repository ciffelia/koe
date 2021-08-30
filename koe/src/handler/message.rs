use crate::context_store;
use crate::status::VoiceConnectionStatusMap;
use crate::voice_client::VoiceClient;
use anyhow::Result;
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
        let author_name = msg.author_nick(&ctx.http).await.unwrap_or(msg.author.name);
        let text = author_name + "。" + &msg.content;
        status.speech_queue.push(text)?;
    }

    Ok(())
}

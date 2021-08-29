use crate::context_store;
use crate::voice_client::VoiceClient;
use anyhow::Result;
use dashmap::DashMap;
use koe_speech::SpeechProvider;
use serenity::{
    client::Context,
    model::{
        channel::Message,
        id::{ChannelId, GuildId},
    },
};

pub async fn handle_message(ctx: &Context, msg: Message) -> Result<()> {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Ok(()),
    };

    let voice_client = context_store::extract::<VoiceClient>(ctx).await?;
    if !voice_client.is_connected(ctx, guild_id).await? {
        return Ok(());
    }

    let bound_text_channel_map = context_store::extract::<DashMap<GuildId, ChannelId>>(ctx).await?;
    let bound_text_channel = bound_text_channel_map.get(&guild_id);
    if bound_text_channel.as_deref() != Some(&msg.channel_id) {
        return Ok(());
    }

    let author_name = msg.author_nick(&ctx.http).await.unwrap_or(msg.author.name);
    let text = author_name + "。" + &msg.content;

    let speech_provider = context_store::extract::<SpeechProvider>(ctx).await?;
    let speech_audio = speech_provider.make_speech(text).await?;
    voice_client.play(ctx, guild_id, speech_audio).await?;

    Ok(())
}

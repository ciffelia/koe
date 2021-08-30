use crate::context_store;
use anyhow::{anyhow, Context as _, Result};
use koe_audio::EncodedAudio;
use serenity::client::Context;
use songbird::id::{ChannelId, GuildId};
use songbird::input::reader::Reader;
use songbird::input::{Codec, Container, Input};
use songbird::join::Join;

pub struct VoiceClient;

impl VoiceClient {
    pub fn new() -> Self {
        Self
    }

    pub async fn join(
        &self,
        ctx: &Context,
        guild_id: impl Into<GuildId>,
        channel_id: impl Into<ChannelId>,
    ) -> Result<()> {
        let manager = context_store::extract_songbird(ctx).await?;
        let guild_id = guild_id.into();
        let channel_id = channel_id.into();

        let call = manager.get_or_insert(guild_id);

        // See https://docs.rs/songbird/latest/songbird/struct.Call.html#method.join
        let join_res: Result<Join> = {
            let mut handler = call.lock().await;
            handler.deafen(true).await?;

            let join = handler.join(channel_id).await?;

            Ok(join)
        };
        let join = join_res?;
        join.await?;

        Ok(())
    }

    pub async fn leave(&self, ctx: &Context, guild_id: impl Into<GuildId>) -> Result<()> {
        let manager = context_store::extract_songbird(ctx).await?;
        let guild_id = guild_id.into();

        manager.remove(guild_id).await?;

        Ok(())
    }

    pub async fn is_connected(&self, ctx: &Context, guild_id: impl Into<GuildId>) -> Result<bool> {
        let manager = context_store::extract_songbird(ctx).await?;
        let guild_id = guild_id.into();

        let is_connected = manager.get(guild_id).is_some();

        Ok(is_connected)
    }

    pub async fn play(
        &self,
        ctx: &Context,
        guild_id: impl Into<GuildId>,
        audio: EncodedAudio,
    ) -> Result<()> {
        let manager = context_store::extract_songbird(ctx).await?;
        let guild_id = guild_id.into();

        let decoded_audio = audio
            .into_decoded()
            .await
            .context(anyhow!("Failed to decode audio"))?;

        let call = manager
            .get(guild_id)
            .ok_or_else(|| anyhow!("Not in voice channel"))?;
        let mut handler = call.lock().await;

        handler.play_source(Input::new(
            false,
            Reader::from_memory(decoded_audio.data()),
            Codec::Pcm,
            Container::Raw,
            None,
        ));

        Ok(())
    }
}

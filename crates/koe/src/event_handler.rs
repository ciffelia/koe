use anyhow::Context as _;
use log::{error, info};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    gateway::ActivityData,
    model::{
        application::Interaction, channel::Message, gateway::Ready, guild::Guild, voice::VoiceState,
    },
};

use crate::{commands, components, message, voice_state};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);

        ctx.set_activity(Some(ActivityData::playing("テキストチャット 読み上げBot")));

        for guild in &ready.guilds {
            if let Err(err) = guild
                .id
                .set_commands(&ctx.http, commands::commands())
                .await
                .context("Failed to set guild application commands")
            {
                error!("{err:?}");
            }
        }
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: Option<bool>) {
        if let Err(err) = guild
            .id
            .set_commands(&ctx.http, commands::commands())
            .await
            .context("Failed to set guild application commands")
        {
            error!("{err:?}");
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(command) => {
                if let Err(err) = commands::handle_interaction(&ctx, &command)
                    .await
                    .context("Failed to respond to slash command")
                {
                    error!("{err:?}");
                }
            }
            Interaction::Component(component_interaction) => {
                if let Err(err) = components::handle_interaction(&ctx, &component_interaction)
                    .await
                    .context("Failed to respond to message components interaction")
                {
                    error!("{err:?}");
                }
            }
            _ => {}
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(err) = message::handle(&ctx, msg)
            .await
            .context("Failed to handle message")
        {
            error!("{err:?}");
        }
    }

    async fn voice_state_update(
        &self,
        ctx: Context,
        _old_voice_state: Option<VoiceState>,
        new_voice_state: VoiceState,
    ) {
        if let Err(err) = voice_state::handler::handle_update(&ctx, new_voice_state.guild_id)
            .await
            .context("Failed to handle voice state update")
        {
            error!("{err:?}");
        }
    }
}

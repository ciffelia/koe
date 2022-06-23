mod command;
mod message;
mod voice_state;

use crate::command_setup::setup_guild_commands;
use crate::error::report_error;
use crate::handler::voice_state::handle_voice_state_update;
use anyhow::Context as _;
use command::handle_command;
use log::info;
use message::handle_message;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::Message, gateway::Ready, guild::Guild, id::GuildId, interactions::Interaction,
        voice::VoiceState,
    },
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);

        for guild in &ready.guilds {
            if let Err(err) = setup_guild_commands(&ctx, guild.id())
                .await
                .context("Failed to set guild application commands")
            {
                report_error(err);
            }
        }
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: bool) {
        if let Err(err) = setup_guild_commands(&ctx, guild.id)
            .await
            .context("Failed to set guild application commands")
        {
            report_error(err);
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let command = match interaction {
            Interaction::ApplicationCommand(command) => command,
            _ => return,
        };

        if let Err(err) = handle_command(&ctx, &command)
            .await
            .context("Failed to respond to slash command")
        {
            report_error(err);
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(err) = handle_message(&ctx, msg)
            .await
            .context("Failed to handle message")
        {
            report_error(err);
        }
    }

    async fn voice_state_update(
        &self,
        ctx: Context,
        guild_id: Option<GuildId>,
        _old_voice_state: Option<VoiceState>,
        _new_voice_state: VoiceState,
    ) {
        if let Err(err) = handle_voice_state_update(&ctx, guild_id)
            .await
            .context("Failed to handle voice state update")
        {
            report_error(err);
        }
    }
}

use anyhow::Context as _;
use log::{error, info};
use poise::serenity_prelude as serenity;
use serenity::{
    FullEvent,
    gateway::ActivityData,
    model::{application::Interaction, channel::Message, gateway::Ready, voice::VoiceState},
};

use crate::{component_interaction, message, voice_state};

/// Event handler for the bot
pub async fn event_handler(
    ctx: &serenity::Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, crate::app_state::AppState, anyhow::Error>,
    _data: &crate::app_state::AppState,
) -> Result<(), anyhow::Error> {
    match event {
        FullEvent::Ready { data_about_bot } => {
            handle_ready(ctx, data_about_bot).await;
        }
        FullEvent::InteractionCreate { interaction } => {
            handle_interaction_create(ctx, interaction, _data).await;
        }
        FullEvent::Message { new_message } => {
            handle_message(ctx, new_message.clone(), _data).await;
        }
        FullEvent::VoiceStateUpdate { old, new } => {
            handle_voice_state_update(ctx, old.as_ref(), new, _data).await;
        }
        _ => {}
    }

    Ok(())
}

async fn handle_ready(ctx: &serenity::Context, ready: &Ready) {
    info!("Connected as {}", ready.user.name);
    ctx.set_activity(Some(ActivityData::playing("テキストチャット 読み上げBot")));
}

async fn handle_interaction_create(
    ctx: &serenity::Context,
    interaction: &Interaction,
    data: &crate::app_state::AppState,
) {
    // Handle component interactions (Poise handles command interactions automatically)
    if let Interaction::Component(component_interaction) = interaction
        && let Err(err) = component_interaction::handler::handle(ctx, component_interaction, data)
            .await
            .context("Failed to respond to message components interaction")
    {
        error!("{:?}", err);
    }
}

async fn handle_message(ctx: &serenity::Context, msg: Message, data: &crate::app_state::AppState) {
    if let Err(err) = message::handler::handle(ctx, msg, data)
        .await
        .context("Failed to handle message")
    {
        error!("{:?}", err);
    }
}

async fn handle_voice_state_update(
    ctx: &serenity::Context,
    _old_voice_state: Option<&VoiceState>,
    new_voice_state: &VoiceState,
    data: &crate::app_state::AppState,
) {
    if let Err(err) = voice_state::handler::handle_update(ctx, new_voice_state.guild_id, data)
        .await
        .context("Failed to handle voice state update")
    {
        error!("{:?}", err);
    }
}

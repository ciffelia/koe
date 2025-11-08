use anyhow::{Context as _, Result, anyhow, bail};
use koe_db::voice::SetOption;
use serenity::{
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
    model::application::{ComponentInteraction, ComponentInteractionDataKind},
};

use super::custom_id;
use crate::app_state;

pub async fn handle(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    if interaction.data.custom_id == custom_id::CUSTOM_ID_VOICE {
        handle_voice(ctx, interaction)
            .await
            .context(r#"Failed to handle "voice" message component interaction"#)?;
    } else {
        bail!(
            "Unknown message component interaction custom_id: {}",
            interaction.data.custom_id
        );
    }

    Ok(())
}

async fn handle_voice(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    let guild_id = interaction
        .guild_id
        .ok_or_else(|| anyhow!("Failed to get guild ID"))?;

    let ComponentInteractionDataKind::StringSelect { values } = &interaction.data.kind else {
        bail!("Expected string select interaction")
    };

    let selected_preset_id = values
        .first()
        .ok_or_else(|| anyhow!("Value not available in message component interaction"))?
        .parse::<i64>()?;

    let state = app_state::get(ctx).await?;

    let available_presets = state.voicevox_client.presets().await?;
    let selected_preset = available_presets
        .into_iter()
        .find(|p| p.id == selected_preset_id)
        .ok_or_else(|| anyhow!("Preset {selected_preset_id} not available"))?;

    let mut conn = state
        .redis_client
        .get_multiplexed_async_connection()
        .await?;
    koe_db::voice::set(
        &mut conn,
        SetOption {
            guild_id: guild_id.into(),
            user_id: interaction.user.id.into(),
            value: selected_preset_id,
        },
    )
    .await?;

    r(
        ctx,
        interaction,
        format!(
            "<@{}>さんの声を`{}`に変更しました。",
            interaction.user.id, selected_preset.name
        ),
    )
    .await?;
    Ok(())
}

// Helper function to create text message response
async fn r(ctx: &Context, interaction: &ComponentInteraction, text: impl ToString) -> Result<()> {
    let message = CreateInteractionResponseMessage::new().content(text.to_string());

    interaction
        .create_response(&ctx.http, CreateInteractionResponse::Message(message))
        .await
        .context("Failed to create interaction response")?;

    Ok(())
}

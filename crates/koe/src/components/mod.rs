pub mod voice_select;

use anyhow::{Context as _, Result, bail};
use serenity::{
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
    model::application::ComponentInteraction,
};

pub async fn handle_interaction(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    if voice_select::custom_id_matches(&interaction.data.custom_id) {
        voice_select::handle_interaction(ctx, interaction)
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

/// Helper function to create text message response
async fn respond_text(
    ctx: &Context,
    interaction: &ComponentInteraction,
    text: impl ToString,
) -> Result<()> {
    let message = CreateInteractionResponseMessage::new().content(text.to_string());

    interaction
        .create_response(&ctx.http, CreateInteractionResponse::Message(message))
        .await
        .context("Failed to create interaction response")?;

    Ok(())
}

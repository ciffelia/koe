use anyhow::{Context as _, Result};
use serenity::{
    all::CreateButton,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
    model::application::ComponentInteraction,
};

const CUSTOM_ID_PREFIX_PAGINATION_BUTTON: &str = "voice_pagination_button:";

pub fn custom_id_matches(custom_id: &str) -> bool {
    custom_id.starts_with(CUSTOM_ID_PREFIX_PAGINATION_BUTTON)
}

pub fn component(page_idx: usize, enabled: bool) -> CreateButton {
    CreateButton::new(format!("{CUSTOM_ID_PREFIX_PAGINATION_BUTTON}{page_idx}"))
        .label((page_idx + 1).to_string())
        .disabled(!enabled)
}

pub async fn handle_interaction(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    let guild_id = interaction
        .guild_id
        .context("Guild ID not available in interaction")?;

    let target_page = interaction
        .data
        .custom_id
        .strip_prefix(CUSTOM_ID_PREFIX_PAGINATION_BUTTON)
        .and_then(|s| s.parse().ok())
        .with_context(|| {
            format!(
                "Invalid pagination button custom_id format: {}",
                interaction.data.custom_id
            )
        })?;

    let components =
        super::components(ctx, guild_id, interaction.user.id, Some(target_page)).await?;

    let message = CreateInteractionResponseMessage::new()
        .ephemeral(true)
        .components(components);

    interaction
        .create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(message))
        .await
        .context("Failed to create interaction response")?;

    Ok(())
}

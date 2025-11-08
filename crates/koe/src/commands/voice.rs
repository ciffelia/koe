use anyhow::{Context as _, Result};
use serenity::{
    builder::{
        CreateActionRow, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage,
    },
    client::Context,
    model::application::{CommandInteraction, InteractionContext},
};

use crate::components;

pub fn commands() -> Vec<CreateCommand> {
    vec![
        CreateCommand::new("voice")
            .description("話者の設定")
            .contexts(vec![InteractionContext::Guild]),
    ]
}

pub fn matches(cmd: &CommandInteraction) -> bool {
    cmd.data.name == "voice"
}

pub async fn handle(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    let guild_id = cmd
        .guild_id
        .context("Guild ID not available in interaction")?;

    let select_menu = components::voice_select::component(ctx, guild_id, cmd.user.id).await?;

    let action_row = CreateActionRow::SelectMenu(select_menu);

    let message = CreateInteractionResponseMessage::new()
        .ephemeral(true)
        .components(vec![action_row]);

    cmd.create_response(&ctx.http, CreateInteractionResponse::Message(message))
        .await
        .context("Failed to create interaction response")?;

    Ok(())
}

mod dict;
mod help;
mod join;
mod leave;
mod skip;
mod voice;

use anyhow::{Context as _, Result, bail};
use serenity::{
    builder::{CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
    model::application::CommandInteraction,
};

pub fn commands() -> Vec<CreateCommand> {
    let mut commands = Vec::new();

    commands.extend(dict::commands());
    commands.extend(help::commands());
    commands.extend(join::commands());
    commands.extend(leave::commands());
    commands.extend(skip::commands());
    commands.extend(voice::commands());

    commands
}

pub async fn handle_interaction(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    if dict::matches(cmd) {
        dict::handle(ctx, cmd)
            .await
            .context("Failed to execute /dict")?;
        return Ok(());
    }

    if help::matches(cmd) {
        help::handle(ctx, cmd)
            .await
            .context("Failed to execute /help")?;
        return Ok(());
    }

    if join::matches(cmd) {
        join::handle(ctx, cmd)
            .await
            .context("Failed to execute /join")?;
        return Ok(());
    }

    if leave::matches(cmd) {
        leave::handle(ctx, cmd)
            .await
            .context("Failed to execute /leave")?;
        return Ok(());
    }

    if skip::matches(cmd) {
        skip::handle(ctx, cmd)
            .await
            .context("Failed to execute /skip")?;
        return Ok(());
    }

    if voice::matches(cmd) {
        voice::handle(ctx, cmd)
            .await
            .context("Failed to execute /voice")?;
        return Ok(());
    }

    bail!("Unknown command: {:?}", cmd.data.name);
}

/// Helper function to create text message response
async fn respond_text(ctx: &Context, cmd: &CommandInteraction, text: impl ToString) -> Result<()> {
    let message = CreateInteractionResponseMessage::new().content(text.to_string());

    cmd.create_response(&ctx.http, CreateInteractionResponse::Message(message))
        .await
        .context("Failed to create interaction response")?;

    Ok(())
}

/// Helper function to sanitize text for Discord responses
fn sanitize_response(text: &str) -> String {
    format!("`{}`", text.replace('`', ""))
}

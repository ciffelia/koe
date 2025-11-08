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
    } else if help::matches(cmd) {
        help::handle(ctx, cmd)
            .await
            .context("Failed to execute /help")?;
    } else if join::matches(cmd) {
        join::handle(ctx, cmd)
            .await
            .context("Failed to execute /join")?;
    } else if leave::matches(cmd) {
        leave::handle(ctx, cmd)
            .await
            .context("Failed to execute /leave")?;
    } else if skip::matches(cmd) {
        skip::handle(ctx, cmd)
            .await
            .context("Failed to execute /skip")?;
    } else if voice::matches(cmd) {
        voice::handle(ctx, cmd)
            .await
            .context("Failed to execute /voice")?;
    } else {
        bail!("Unknown command: {:?}", cmd.data.name);
    }

    Ok(())
}

/// Helper function to create text message response
async fn respond_text(
    ctx: &Context,
    cmd: &CommandInteraction,
    text: impl Into<String>,
) -> Result<()> {
    let message = CreateInteractionResponseMessage::new().content(text);

    cmd.create_response(&ctx.http, CreateInteractionResponse::Message(message))
        .await
        .context("Failed to create interaction response")?;

    Ok(())
}

/// Helper function to sanitize text for Discord responses
fn sanitize_response(text: &str) -> String {
    format!("`{}`", text.replace('`', ""))
}

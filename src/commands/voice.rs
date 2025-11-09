use anyhow::{Context as _, Result};
use serenity::{
    builder::{CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
    model::application::{CommandInteraction, InteractionContext},
};

use crate::components;

const COMMAND_NAME: &str = "voice";

pub fn commands() -> Vec<CreateCommand> {
    vec![
        CreateCommand::new(COMMAND_NAME)
            .description("話者の設定")
            .contexts(vec![InteractionContext::Guild]),
    ]
}

pub fn matches(cmd: &CommandInteraction) -> bool {
    cmd.data.name == COMMAND_NAME
}

pub async fn handle(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    let guild_id = cmd
        .guild_id
        .context("Guild ID not available in interaction")?;

    let components = components::voice_select::components(ctx, guild_id, cmd.user.id, None).await?;

    let message = CreateInteractionResponseMessage::new()
        .ephemeral(true)
        .content(
            "-# [VOICEVOXウェブサイト](<https://voicevox.hiroshiba.jp/#characters>)で音声サンプルを試聴できます。",
        )
        .components(components);

    cmd.create_response(&ctx.http, CreateInteractionResponse::Message(message))
        .await
        .context("Failed to create interaction response")?;

    Ok(())
}

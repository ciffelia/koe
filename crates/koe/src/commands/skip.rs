use anyhow::{Context as _, Result};
use serenity::{
    builder::CreateCommand,
    client::Context,
    model::application::{CommandInteraction, InteractionContext},
};

use super::respond_text;

const COMMAND_NAME: &str = "skip";
const ALIAS_COMMAND_NAME: &str = "kskip";

pub fn commands() -> Vec<CreateCommand> {
    vec![
        CreateCommand::new(COMMAND_NAME)
            .description("読み上げ中のメッセージをスキップ")
            .contexts(vec![InteractionContext::Guild]),
        CreateCommand::new(ALIAS_COMMAND_NAME)
            .description("読み上げ中のメッセージをスキップ")
            .contexts(vec![InteractionContext::Guild]),
    ]
}

pub fn matches(cmd: &CommandInteraction) -> bool {
    matches!(cmd.data.name.as_str(), COMMAND_NAME | ALIAS_COMMAND_NAME)
}

pub async fn handle(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    let guild_id = cmd
        .guild_id
        .context("Guild ID not available in interaction")?;

    if !koe_call::is_connected(ctx, guild_id).await? {
        {
            respond_text(ctx, cmd, "どのボイスチャンネルにも接続していません。").await?;
            return Ok(());
        };
    }

    koe_call::skip(ctx, guild_id).await?;

    respond_text(ctx, cmd, "読み上げ中のメッセージをスキップしました。").await?;
    Ok(())
}

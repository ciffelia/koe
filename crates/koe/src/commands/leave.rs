use anyhow::{Context as _, Result};
use serenity::{
    builder::CreateCommand,
    client::Context,
    model::application::{CommandInteraction, InteractionContext},
};

use super::respond_text;
use crate::app_state;

const COMMAND_NAME: &str = "leave";
const ALIAS_COMMAND_NAME: &str = "kleave";

pub fn commands() -> Vec<CreateCommand> {
    vec![
        CreateCommand::new(COMMAND_NAME)
            .description("ボイスチャンネルから退出")
            .contexts(vec![InteractionContext::Guild]),
        CreateCommand::new(ALIAS_COMMAND_NAME)
            .description("ボイスチャンネルから退出")
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

    koe_call::leave(ctx, guild_id).await?;

    let state = app_state::get(ctx).await?;
    state.connected_guild_states.remove(&guild_id);

    respond_text(ctx, cmd, "切断しました。").await?;
    Ok(())
}

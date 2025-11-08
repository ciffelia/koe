use anyhow::Result;
use serenity::{builder::CreateCommand, client::Context, model::application::CommandInteraction};

use super::respond_text;
use crate::app_state;

pub fn commands() -> Vec<CreateCommand> {
    vec![
        CreateCommand::new("leave").description("ボイスチャンネルから退出"),
        CreateCommand::new("kleave").description("ボイスチャンネルから退出"),
    ]
}

pub fn matches(cmd: &CommandInteraction) -> bool {
    matches!(cmd.data.name.as_str(), "leave" | "kleave")
}

pub async fn handle(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    let guild_id = match cmd.guild_id {
        Some(id) => id,
        None => {
            respond_text(ctx, cmd, "`/leave`, `/kleave` はサーバー内でのみ使えます。").await?;
            return Ok(());
        }
    };

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

use anyhow::Result;
use serenity::{builder::CreateCommand, client::Context, model::application::CommandInteraction};

use super::respond_text;

pub fn commands() -> Vec<CreateCommand> {
    vec![
        CreateCommand::new("skip").description("読み上げ中のメッセージをスキップ"),
        CreateCommand::new("kskip").description("読み上げ中のメッセージをスキップ"),
    ]
}

pub fn matches(cmd: &CommandInteraction) -> bool {
    matches!(cmd.data.name.as_str(), "skip" | "kskip")
}

pub async fn handle(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    let Some(guild_id) = cmd.guild_id else {
        respond_text(ctx, cmd, "`/skip`, `/kskip` はサーバー内でのみ使えます。").await?;
        return Ok(());
    };

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

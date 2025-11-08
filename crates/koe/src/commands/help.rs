use anyhow::Result;
use serenity::{builder::CreateCommand, client::Context, model::application::CommandInteraction};

use super::respond_text;

pub fn commands() -> Vec<CreateCommand> {
    vec![CreateCommand::new("help").description("使い方を表示")]
}

pub fn matches(cmd: &CommandInteraction) -> bool {
    cmd.data.name == "help"
}

pub async fn handle(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    respond_text(
        ctx,
        cmd,
        "使い方はこちらをご覧ください:\nhttps://github.com/ciffelia/koe/blob/main/docs/user_guide.md",
    )
    .await?;
    Ok(())
}

use anyhow::Result;
use serenity::{
    builder::CreateCommand,
    client::Context,
    model::application::{CommandInteraction, InteractionContext},
};

use super::respond_text;

const COMMAND_NAME: &str = "help";

pub fn commands() -> Vec<CreateCommand> {
    vec![
        CreateCommand::new(COMMAND_NAME)
            .description("使い方を表示")
            .contexts(vec![InteractionContext::Guild]),
    ]
}

pub fn matches(cmd: &CommandInteraction) -> bool {
    cmd.data.name == COMMAND_NAME
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

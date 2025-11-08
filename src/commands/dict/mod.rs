pub mod add;
pub mod remove;
pub mod view;

use anyhow::{Context as _, Ok, Result, bail};
use serenity::{
    builder::CreateCommand,
    client::Context as SerenityContext,
    model::application::{CommandInteraction, InteractionContext},
};

const COMMAND_NAME: &str = "dict";

pub fn commands() -> Vec<CreateCommand> {
    vec![
        CreateCommand::new(COMMAND_NAME)
            .description("読み上げ辞書の閲覧と編集")
            .contexts(vec![InteractionContext::Guild])
            .add_option(add::subcommand())
            .add_option(remove::subcommand())
            .add_option(view::subcommand()),
    ]
}

pub fn matches(cmd: &CommandInteraction) -> bool {
    cmd.data.name == COMMAND_NAME
}

pub async fn handle(ctx: &SerenityContext, cmd: &CommandInteraction) -> Result<()> {
    let options = cmd.data.options();
    let Some(option) = options.first() else {
        bail!("No subcommand provided for /dict");
    };

    if add::matches(option) {
        add::handle(ctx, cmd, option)
            .await
            .context("Failed to execute /dict add")?;
    } else if remove::matches(option) {
        remove::handle(ctx, cmd, option)
            .await
            .context("Failed to execute /dict remove")?;
    } else if view::matches(option) {
        view::handle(ctx, cmd)
            .await
            .context("Failed to execute /dict view")?;
    } else {
        bail!("Unknown subcommand for /dict: {}", option.name);
    }

    Ok(())
}

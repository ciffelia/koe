pub mod add;
pub mod remove;
pub mod view;

use anyhow::{Context as _, Ok, Result, bail};
use serenity::{
    builder::CreateCommand,
    client::Context as SerenityContext,
    model::application::{CommandInteraction, ResolvedValue},
};

pub fn commands() -> Vec<CreateCommand> {
    vec![
        CreateCommand::new("dict")
            .description("読み上げ辞書の閲覧と編集")
            .add_option(add::subcommand())
            .add_option(remove::subcommand())
            .add_option(view::subcommand()),
    ]
}

pub fn matches(cmd: &CommandInteraction) -> bool {
    cmd.data.name == "dict"
}

pub async fn handle(ctx: &SerenityContext, cmd: &CommandInteraction) -> Result<()> {
    let options = cmd.data.options();
    let option = match options.first() {
        Some(option) => option,
        None => bail!("No subcommand provided for /dict"),
    };

    match option.name {
        "add" => {
            let ResolvedValue::SubCommand(suboptions) = &option.value else {
                bail!("Invalid subcommand value for /dict add");
            };

            add::handle(ctx, cmd, suboptions)
                .await
                .context("Failed to execute /dict add")?;
        }
        "remove" => {
            let ResolvedValue::SubCommand(suboptions) = &option.value else {
                bail!("Invalid subcommand value for /dict remove");
            };

            remove::handle(ctx, cmd, suboptions)
                .await
                .context("Failed to execute /dict remove")?;
        }
        "view" => {
            view::handle(ctx, cmd)
                .await
                .context("Failed to execute /dict view")?;
        }
        _ => bail!("Unknown subcommand for /dict: {}", option.name),
    };

    Ok(())
}

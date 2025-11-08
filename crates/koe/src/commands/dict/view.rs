use anyhow::{Context as _, Result};
use koe_db::dict::GetAllOption;
use serenity::{
    builder::{
        CreateCommandOption, CreateEmbed, CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
    client::Context,
    model::application::{CommandInteraction, CommandOptionType, ResolvedOption},
};

use super::super::sanitize_response;
use crate::app_state;

const SUBCOMMAND_NAME: &str = "view";

pub fn subcommand() -> CreateCommandOption {
    CreateCommandOption::new(CommandOptionType::SubCommand, SUBCOMMAND_NAME, "辞書を表示")
}

pub fn matches(option: &ResolvedOption<'_>) -> bool {
    option.name == SUBCOMMAND_NAME
}

pub async fn handle(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    let guild_id = cmd
        .guild_id
        .context("Guild ID not available in interaction")?;

    let state = app_state::get(ctx).await?;
    let mut conn = state
        .redis_client
        .get_multiplexed_async_connection()
        .await?;

    let dict = koe_db::dict::get_all(
        &mut conn,
        GetAllOption {
            guild_id: guild_id.into(),
        },
    )
    .await?;

    {
        let mut embed = CreateEmbed::default();

        let guild_name = guild_id
            .name(&ctx.cache)
            .unwrap_or_else(|| "サーバー".to_string());

        embed = embed.title(format!("📕 {guild_name}の辞書"));

        embed = embed.fields(
            dict.into_iter()
                .map(|(word, read_as)| (word, sanitize_response(&read_as), false)),
        );

        let message = CreateInteractionResponseMessage::new().embed(embed);

        cmd.create_response(&ctx.http, CreateInteractionResponse::Message(message))
            .await
            .context("Failed to create interaction response")?;
    };

    Ok(())
}

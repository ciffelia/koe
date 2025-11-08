use anyhow::{Context as _, Result};
use koe_db::dict::GetAllOption;
use serenity::{
    builder::{
        CreateCommandOption, CreateEmbed, CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
    client::Context,
    model::application::{CommandInteraction, CommandOptionType},
};

use super::super::{respond_text, sanitize_response};
use crate::app_state;

pub fn subcommand() -> CreateCommandOption {
    CreateCommandOption::new(CommandOptionType::SubCommand, "view", "辞書を表示")
}

pub async fn handle(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    let guild_id = match cmd.guild_id {
        Some(id) => id,
        None => {
            respond_text(ctx, cmd, "`/dict view` はサーバー内でのみ使えます。").await?;
            return Ok(());
        }
    };

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

        embed = embed.title(format!("📕 {}の辞書", guild_name));

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

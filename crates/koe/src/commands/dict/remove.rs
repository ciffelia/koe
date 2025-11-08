use anyhow::{Result, bail};
use koe_db::dict::RemoveResponse;
use serenity::{
    builder::CreateCommandOption,
    client::Context,
    model::application::{CommandInteraction, CommandOptionType, ResolvedOption, ResolvedValue},
};

use super::super::{respond_text, sanitize_response};
use crate::app_state;

pub fn subcommand() -> CreateCommandOption {
    CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "remove",
        "辞書から項目を削除",
    )
    .add_sub_option(
        CreateCommandOption::new(CommandOptionType::String, "word", "削除したい語句")
            .required(true),
    )
}

pub async fn handle(
    ctx: &Context,
    cmd: &CommandInteraction,
    suboptions: &[ResolvedOption<'_>],
) -> Result<()> {
    let guild_id = match cmd.guild_id {
        Some(id) => id,
        None => {
            respond_text(ctx, cmd, "`/dict remove` はサーバー内でのみ使えます。").await?;
            return Ok(());
        }
    };

    let [
        ResolvedOption {
            name: "word",
            value: ResolvedValue::String(word),
            ..
        },
    ] = suboptions
    else {
        bail!("Failed to parse /dict remove options");
    };

    let state = app_state::get(ctx).await?;
    let mut conn = state
        .redis_client
        .get_multiplexed_async_connection()
        .await?;

    let resp = koe_db::dict::remove(
        &mut conn,
        koe_db::dict::RemoveOption {
            guild_id: guild_id.into(),
            word: word.to_string(),
        },
    )
    .await?;

    let msg = match resp {
        RemoveResponse::Success => format!("辞書から{}を削除しました。", sanitize_response(word)),
        RemoveResponse::WordDoesNotExist => {
            format!("{}は辞書に登録されていません。", sanitize_response(word))
        }
    };
    respond_text(ctx, cmd, msg).await?;
    Ok(())
}

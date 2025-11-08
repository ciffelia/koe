use anyhow::{Context as _, Result, bail};
use koe_db::dict::RemoveResponse;
use serenity::{
    builder::CreateCommandOption,
    client::Context,
    model::application::{CommandInteraction, CommandOptionType, ResolvedOption, ResolvedValue},
};

use super::super::{respond_text, sanitize_response};
use crate::app_state;

const SUBCOMMAND_NAME: &str = "remove";
const WORD_OPTION_NAME: &str = "word";

pub fn subcommand() -> CreateCommandOption {
    CreateCommandOption::new(
        CommandOptionType::SubCommand,
        SUBCOMMAND_NAME,
        "辞書から項目を削除",
    )
    .add_sub_option(
        CreateCommandOption::new(
            CommandOptionType::String,
            WORD_OPTION_NAME,
            "削除したい語句",
        )
        .required(true),
    )
}

pub async fn handle(
    ctx: &Context,
    cmd: &CommandInteraction,
    suboptions: &[ResolvedOption<'_>],
) -> Result<()> {
    let guild_id = cmd
        .guild_id
        .context("Guild ID not available in interaction")?;

    let [
        ResolvedOption {
            name: WORD_OPTION_NAME,
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

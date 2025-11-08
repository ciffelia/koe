use anyhow::{Context as _, Result, bail};
use koe_db::dict::{InsertOption, InsertResponse};
use serenity::{
    builder::CreateCommandOption,
    client::Context,
    model::application::{CommandInteraction, CommandOptionType, ResolvedOption, ResolvedValue},
};

use super::super::{respond_text, sanitize_response};
use crate::app_state;

pub fn subcommand() -> CreateCommandOption {
    CreateCommandOption::new(CommandOptionType::SubCommand, "add", "辞書に項目を追加")
        .add_sub_option(
            CreateCommandOption::new(CommandOptionType::String, "word", "読み方を指定したい語句")
                .required(true),
        )
        .add_sub_option(
            CreateCommandOption::new(CommandOptionType::String, "read-as", "語句の読み方")
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
            name: "word",
            value: ResolvedValue::String(word),
            ..
        },
        ResolvedOption {
            name: "read-as",
            value: ResolvedValue::String(read_as),
            ..
        },
    ] = suboptions
    else {
        bail!("Failed to parse /dict add options");
    };

    let state = app_state::get(ctx).await?;
    let mut conn = state
        .redis_client
        .get_multiplexed_async_connection()
        .await?;

    let resp = koe_db::dict::insert(
        &mut conn,
        InsertOption {
            guild_id: guild_id.into(),
            word: word.to_string(),
            read_as: read_as.to_string(),
        },
    )
    .await?;

    let msg = match resp {
        InsertResponse::Success => format!(
            "{}の読み方を{}として辞書に登録しました。",
            sanitize_response(word),
            sanitize_response(read_as)
        ),
        InsertResponse::WordAlreadyExists => format!(
            "すでに{}は辞書に登録されています。",
            sanitize_response(word)
        ),
    };
    respond_text(ctx, cmd, msg).await?;
    Ok(())
}

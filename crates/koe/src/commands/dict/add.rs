use anyhow::{Context as _, Result, bail};
use koe_db::dict::{InsertOption, InsertResponse};
use serenity::{
    builder::CreateCommandOption,
    client::Context,
    model::application::{CommandInteraction, CommandOptionType, ResolvedOption, ResolvedValue},
};

use super::super::{respond_text, sanitize_response};
use crate::app_state;

const SUBCOMMAND_NAME: &str = "add";
const WORD_OPTION_NAME: &str = "word";
const READ_AS_OPTION_NAME: &str = "read_as";

pub fn subcommand() -> CreateCommandOption {
    CreateCommandOption::new(
        CommandOptionType::SubCommand,
        SUBCOMMAND_NAME,
        "辞書に項目を追加",
    )
    .add_sub_option(
        CreateCommandOption::new(
            CommandOptionType::String,
            WORD_OPTION_NAME,
            "読み方を指定したい語句",
        )
        .required(true),
    )
    .add_sub_option(
        CreateCommandOption::new(
            CommandOptionType::String,
            READ_AS_OPTION_NAME,
            "語句の読み方",
        )
        .required(true),
    )
}

pub fn matches(option: &ResolvedOption<'_>) -> bool {
    option.name == SUBCOMMAND_NAME
}

pub async fn handle(
    ctx: &Context,
    cmd: &CommandInteraction,
    option: &ResolvedOption<'_>,
) -> Result<()> {
    let guild_id = cmd
        .guild_id
        .context("Guild ID not available in interaction")?;
    let ResolvedValue::SubCommand(suboptions) = &option.value else {
        bail!("Invalid subcommand value for /dict add");
    };

    let [
        ResolvedOption {
            name: WORD_OPTION_NAME,
            value: ResolvedValue::String(word),
            ..
        },
        ResolvedOption {
            name: READ_AS_OPTION_NAME,
            value: ResolvedValue::String(read_as),
            ..
        },
    ] = &suboptions[..]
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

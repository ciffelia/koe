use anyhow::Result;
use koe_db::dict::{GetAllOption, InsertOption, InsertResponse, RemoveOption, RemoveResponse};
use poise::{CreateReply, serenity_prelude as serenity};
use serenity::builder::CreateEmbed;

use crate::app_state;

/// 読み上げ辞書の閲覧と編集
#[poise::command(slash_command, subcommands("add", "remove", "view"), guild_only)]
pub async fn dict(_ctx: app_state::Context<'_>) -> Result<()> {
    Ok(())
}

/// 辞書に項目を追加
#[poise::command(slash_command, guild_only)]
pub async fn add(
    ctx: app_state::Context<'_>,
    #[description = "読み方を指定したい語句"] word: String,
    #[description = "読み方"] read_as: String,
) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();
    let state = ctx.data();

    let mut conn = state
        .redis_client
        .get_multiplexed_async_connection()
        .await?;

    let resp = koe_db::dict::insert(
        &mut conn,
        InsertOption {
            guild_id: guild_id.into(),
            word: word.clone(),
            read_as: read_as.clone(),
        },
    )
    .await?;

    let msg = match resp {
        InsertResponse::Success => format!(
            "{}の読み方を{}として辞書に登録しました。",
            sanitize_response(&word),
            sanitize_response(&read_as)
        ),
        InsertResponse::WordAlreadyExists => {
            format!(
                "すでに{}は辞書に登録されています。",
                sanitize_response(&word)
            )
        }
    };

    ctx.say(msg).await?;
    Ok(())
}

/// 辞書から項目を削除
#[poise::command(slash_command, guild_only)]
pub async fn remove(
    ctx: app_state::Context<'_>,
    #[description = "削除したい語句"] word: String,
) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();
    let state = ctx.data();

    let mut conn = state
        .redis_client
        .get_multiplexed_async_connection()
        .await?;

    let resp = koe_db::dict::remove(
        &mut conn,
        RemoveOption {
            guild_id: guild_id.into(),
            word: word.clone(),
        },
    )
    .await?;

    let msg = match resp {
        RemoveResponse::Success => {
            format!("辞書から{}を削除しました。", sanitize_response(&word))
        }
        RemoveResponse::WordDoesNotExist => {
            format!("{}は辞書に登録されていません。", sanitize_response(&word))
        }
    };

    ctx.say(msg).await?;
    Ok(())
}

/// 辞書を閲覧
#[poise::command(slash_command, guild_only)]
pub async fn view(ctx: app_state::Context<'_>) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();
    let state = ctx.data();

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

    let mut embed = CreateEmbed::default();

    let guild_name = guild_id
        .name(&ctx.serenity_context().cache)
        .unwrap_or_else(|| "サーバー".to_string());

    embed = embed.title(format!("📕 {}の辞書", guild_name));

    embed = embed.fields(
        dict.into_iter()
            .map(|(word, read_as)| (word, sanitize_response(&read_as), false)),
    );

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}

fn sanitize_response(text: &str) -> String {
    format!("`{}`", text.replace('`', ""))
}

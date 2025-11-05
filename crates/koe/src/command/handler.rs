use anyhow::{Context as _, Result, anyhow, bail};
use koe_db::{
    dict::{GetAllOption, InsertOption, InsertResponse, RemoveOption, RemoveResponse},
    voice::GetOption,
};
use rand::seq::IndexedRandom;
use serenity::{
    builder::{
        CreateActionRow, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
        CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption,
    },
    client::Context,
    model::{
        application::CommandInteraction,
        id::{ChannelId, GuildId, UserId},
    },
};

use super::{
    model::{Command, DictAddOption, DictRemoveOption},
    parser::parse,
};
use crate::{app_state, component_interaction::custom_id};

pub async fn handle(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    match parse(cmd) {
        Command::Join => handle_join(ctx, cmd)
            .await
            .context("Failed to execute /join")?,
        Command::Leave => handle_leave(ctx, cmd)
            .await
            .context("Failed to execute /leave")?,
        Command::Skip => handle_skip(ctx, cmd)
            .await
            .context("Failed to execute /skip")?,
        Command::Voice => handle_voice(ctx, cmd)
            .await
            .context("Failed to execute /voice")?,
        Command::DictAdd(option) => handle_dict_add(ctx, cmd, option)
            .await
            .context("Failed to execute /dict add")?,
        Command::DictRemove(option) => handle_dict_remove(ctx, cmd, option)
            .await
            .context("Failed to execute /dict remove")?,
        Command::DictView => handle_dict_view(ctx, cmd)
            .await
            .context("Failed to execute /dict view")?,
        Command::Help => handle_help(ctx, cmd)
            .await
            .context("Failed to execute /help")?,
        Command::Unknown => {
            bail!("Unknown command: {:?}", cmd);
        }
    };

    Ok(())
}

async fn handle_join(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    let guild_id = match cmd.guild_id {
        Some(id) => id,
        None => {
            r(ctx, cmd, "`/join`, `/kjoin` はサーバー内でのみ使えます。").await?;
            return Ok(());
        }
    };
    let user_id = cmd.user.id;
    let text_channel_id = cmd.channel_id;

    let voice_channel_id = match get_user_voice_channel(ctx, &guild_id, &user_id)? {
        Some(channel) => channel,
        None => {
            r(
                ctx,
                cmd,
                "ボイスチャンネルに接続してから `/join` を送信してください。",
            )
            .await?;
            return Ok(());
        }
    };

    koe_call::join_deaf(ctx, guild_id, voice_channel_id).await?;

    let state = app_state::get(ctx).await?;
    state.connected_guild_states.insert(
        guild_id,
        app_state::ConnectedGuildState {
            bound_text_channel: text_channel_id,
            last_message_read: None,
        },
    );

    r(ctx, cmd, "接続しました。").await?;
    Ok(())
}

async fn handle_leave(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    let guild_id = match cmd.guild_id {
        Some(id) => id,
        None => {
            r(ctx, cmd, "`/leave`, `/kleave` はサーバー内でのみ使えます。").await?;
            return Ok(());
        }
    };

    if !koe_call::is_connected(ctx, guild_id).await? {
        {
            r(ctx, cmd, "どのボイスチャンネルにも接続していません。").await?;
            return Ok(());
        };
    }

    koe_call::leave(ctx, guild_id).await?;

    let state = app_state::get(ctx).await?;
    state.connected_guild_states.remove(&guild_id);

    r(ctx, cmd, "切断しました。").await?;
    Ok(())
}

async fn handle_skip(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    let guild_id = match cmd.guild_id {
        Some(id) => id,
        None => {
            r(ctx, cmd, "`/skip`, `/kskip` はサーバー内でのみ使えます。").await?;
            return Ok(());
        }
    };

    if !koe_call::is_connected(ctx, guild_id).await? {
        {
            r(ctx, cmd, "どのボイスチャンネルにも接続していません。").await?;
            return Ok(());
        };
    }

    koe_call::skip(ctx, guild_id).await?;

    r(ctx, cmd, "読み上げ中のメッセージをスキップしました。").await?;
    Ok(())
}

async fn handle_voice(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    let guild_id = match cmd.guild_id {
        Some(id) => id,
        None => {
            r(ctx, cmd, "`/voice` はサーバー内でのみ使えます。").await?;
            return Ok(());
        }
    };

    let state = app_state::get(ctx).await?;

    let available_presets = state.voicevox_client.presets().await?;
    let fallback_preset_id = available_presets
        .choose(&mut rand::rng())
        .map(|p| p.id)
        .ok_or_else(|| anyhow!("No presets available"))?;

    let mut conn = state
        .redis_client
        .get_multiplexed_async_connection()
        .await?;
    let current_preset = koe_db::voice::get(
        &mut conn,
        GetOption {
            guild_id: guild_id.into(),
            user_id: cmd.user.id.into(),
            fallback: fallback_preset_id,
        },
    )
    .await?;

    {
        let option_list = available_presets
            .iter()
            .map(|p| {
                CreateSelectMenuOption::new(&p.name, p.id.to_string())
                    .default_selection(p.id == current_preset)
            })
            .collect::<Vec<_>>();

        let select_menu = CreateSelectMenu::new(
            custom_id::CUSTOM_ID_VOICE,
            CreateSelectMenuKind::String {
                options: option_list,
            },
        );

        let action_row = CreateActionRow::SelectMenu(select_menu);

        let message = CreateInteractionResponseMessage::new()
            .ephemeral(true)
            .components(vec![action_row]);

        cmd.create_response(&ctx.http, CreateInteractionResponse::Message(message))
            .await
            .context("Failed to create interaction response")?;
    };

    Ok(())
}

async fn handle_dict_add(
    ctx: &Context,
    cmd: &CommandInteraction,
    option: DictAddOption,
) -> Result<()> {
    let guild_id = match cmd.guild_id {
        Some(id) => id,
        None => {
            r(ctx, cmd, "`/dict add` はサーバー内でのみ使えます。").await?;
            return Ok(());
        }
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
            word: option.word.clone(),
            read_as: option.read_as.clone(),
        },
    )
    .await?;

    let msg = match resp {
        InsertResponse::Success => format!(
            "{}の読み方を{}として辞書に登録しました。",
            sanitize_response(&option.word),
            sanitize_response(&option.read_as)
        ),
        InsertResponse::WordAlreadyExists => format!(
            "すでに{}は辞書に登録されています。",
            sanitize_response(&option.word)
        ),
    };
    r(ctx, cmd, msg).await?;
    Ok(())
}

async fn handle_dict_remove(
    ctx: &Context,
    cmd: &CommandInteraction,
    option: DictRemoveOption,
) -> Result<()> {
    let guild_id = match cmd.guild_id {
        Some(id) => id,
        None => {
            r(ctx, cmd, "`/dict remove` はサーバー内でのみ使えます。").await?;
            return Ok(());
        }
    };

    let state = app_state::get(ctx).await?;
    let mut conn = state
        .redis_client
        .get_multiplexed_async_connection()
        .await?;

    let resp = koe_db::dict::remove(
        &mut conn,
        RemoveOption {
            guild_id: guild_id.into(),
            word: option.word.clone(),
        },
    )
    .await?;

    let msg = match resp {
        RemoveResponse::Success => format!(
            "辞書から{}を削除しました。",
            sanitize_response(&option.word)
        ),
        RemoveResponse::WordDoesNotExist => format!(
            "{}は辞書に登録されていません。",
            sanitize_response(&option.word)
        ),
    };
    r(ctx, cmd, msg).await?;
    Ok(())
}

async fn handle_dict_view(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    let guild_id = match cmd.guild_id {
        Some(id) => id,
        None => {
            r(ctx, cmd, "`/dict view` はサーバー内でのみ使えます。").await?;
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

async fn handle_help(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    r(
        ctx,
        cmd,
        "使い方はこちらをご覧ください:\nhttps://github.com/ciffelia/koe/blob/main/docs/user_guide.md",
    )
    .await?;
    Ok(())
}

fn get_user_voice_channel(
    ctx: &Context,
    guild_id: &GuildId,
    user_id: &UserId,
) -> Result<Option<ChannelId>> {
    let guild = guild_id
        .to_guild_cached(&ctx.cache)
        .context("Failed to find guild in the cache")?;

    let channel_id = guild
        .voice_states
        .get(user_id)
        .and_then(|voice_state| voice_state.channel_id);

    Ok(channel_id)
}

// Helper function to create text message response
async fn r(ctx: &Context, cmd: &CommandInteraction, text: impl ToString) -> Result<()> {
    let message = CreateInteractionResponseMessage::new().content(text.to_string());

    cmd.create_response(&ctx.http, CreateInteractionResponse::Message(message))
        .await
        .context("Failed to create interaction response")?;

    Ok(())
}

fn sanitize_response(text: &str) -> String {
    format!("`{}`", text.replace('`', ""))
}

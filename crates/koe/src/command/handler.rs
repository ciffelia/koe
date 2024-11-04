use super::{
    model::{Command, DictAddOption, DictRemoveOption},
    parser::parse,
};
use crate::{app_state, component_interaction::custom_id};
use anyhow::{anyhow, bail, Context as _, Result};
use koe_db::{
    dict::{GetAllOption, InsertOption, InsertResponse, RemoveOption, RemoveResponse},
    voice::GetOption,
};
use rand::seq::SliceRandom;
use serenity::{
    builder::{
        CreateActionRow, CreateComponents, CreateEmbed, CreateSelectMenu, CreateSelectMenuOption,
    },
    client::Context,
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
            MessageFlags,
        },
        id::{ChannelId, GuildId, UserId},
    },
};

pub async fn handle(ctx: &Context, cmd: &ApplicationCommandInteraction) -> Result<()> {
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

async fn handle_join(ctx: &Context, cmd: &ApplicationCommandInteraction) -> Result<()> {
    let guild_id = match cmd.guild_id {
        Some(id) => id,
        None => {
            r(ctx, cmd, "`/join`, `/pjoin` はサーバー内でのみ使えます。").await?;
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

async fn handle_leave(ctx: &Context, cmd: &ApplicationCommandInteraction) -> Result<()> {
    let guild_id = match cmd.guild_id {
        Some(id) => id,
        None => {
            r(ctx, cmd, "`/leave`, `/pleave` はサーバー内でのみ使えます。").await?;
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

async fn handle_skip(ctx: &Context, cmd: &ApplicationCommandInteraction) -> Result<()> {
    let guild_id = match cmd.guild_id {
        Some(id) => id,
        None => {
            r(ctx, cmd, "`/skip`, `/pskip` はサーバー内でのみ使えます。").await?;
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

async fn handle_voice(ctx: &Context, cmd: &ApplicationCommandInteraction) -> Result<()> {
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
        .choose(&mut rand::thread_rng())
        .map(|p| p.id)
        .ok_or_else(|| anyhow!("No presets available"))?;

    let mut conn = state.redis_client.get_async_connection().await?;
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
                let mut option = CreateSelectMenuOption::default();
                option
                    .label(&p.name)
                    .value(p.id)
                    .default_selection(p.id == current_preset);
                option
            })
            .collect::<Vec<_>>();

        let mut select = CreateSelectMenu::default();
        select.custom_id(custom_id::CUSTOM_ID_VOICE);
        select.options(|create_options| create_options.set_options(option_list));

        let mut action_row = CreateActionRow::default();
        action_row.add_select_menu(select);

        let mut components = CreateComponents::default();
        components.add_action_row(action_row);

        cmd.create_interaction_response(&ctx.http, |create_response| {
            create_response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|create_message| {
                    create_message
                        .flags(MessageFlags::EPHEMERAL)
                        .set_components(components)
                })
        })
        .await
        .context("Failed to create interaction response")?;
    };

    Ok(())
}

async fn handle_dict_add(
    ctx: &Context,
    cmd: &ApplicationCommandInteraction,
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
    let mut conn = state.redis_client.get_async_connection().await?;

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
    cmd: &ApplicationCommandInteraction,
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
    let mut conn = state.redis_client.get_async_connection().await?;

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

async fn handle_dict_view(ctx: &Context, cmd: &ApplicationCommandInteraction) -> Result<()> {
    let guild_id = match cmd.guild_id {
        Some(id) => id,
        None => {
            r(ctx, cmd, "`/dict view` はサーバー内でのみ使えます。").await?;
            return Ok(());
        }
    };

    let state = app_state::get(ctx).await?;
    let mut conn = state.redis_client.get_async_connection().await?;

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
        embed.title(format!("📕 {}の辞書", guild_name));

        embed.fields(
            dict.into_iter()
                .map(|(word, read_as)| (word, sanitize_response(&read_as), false)),
        );

        cmd.create_interaction_response(&ctx.http, |create_response| {
            create_response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|create_message| create_message.add_embed(embed))
        })
        .await
        .context("Failed to create interaction response")?;
    };

    Ok(())
}

async fn handle_help(ctx: &Context, cmd: &ApplicationCommandInteraction) -> Result<()> {
    r(
        ctx,
        cmd,
        "使い方はこちらをご覧ください:\nhttps://github.com/eraiza0816/koe/blob/main/docs/user_guide.md",
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
async fn r(ctx: &Context, cmd: &ApplicationCommandInteraction, text: impl ToString) -> Result<()> {
    cmd.create_interaction_response(&ctx.http, |create_response| {
        create_response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|create_message| create_message.content(text))
    })
    .await
    .context("Failed to create interaction response")?;

    Ok(())
}

fn sanitize_response(text: &str) -> String {
    format!("`{}`", text.replace('`', ""))
}

use super::{
    model::{Command, CommandResponse, DictAddOption, DictRemoveOption},
    parser::parse,
};
use crate::{app_state, error::report_error};
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
        id::{ChannelId, GuildId, UserId},
        interactions::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
    },
};

pub async fn handle(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<()> {
    let response = match execute(ctx, command)
        .await
        .context("Failed to execute command")
    {
        Ok(resp) => resp,
        Err(err) => {
            report_error(err);
            CommandResponse::Text("内部エラーが発生しました。".to_string())
        }
    };

    command
        .create_interaction_response(&ctx.http, |create_response| {
            create_response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|create_message| match response {
                    CommandResponse::Text(text) => create_message.content(text),
                    CommandResponse::Embed(embed) => create_message.add_embed(embed),
                    CommandResponse::Components(components) => {
                        create_message.set_components(components)
                    }
                })
        })
        .await
        .context("Failed to create interaction response")?;

    Ok(())
}

async fn execute(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<CommandResponse> {
    let command_kind = parse(command);

    match command_kind {
        Command::Join => handle_join(ctx, command)
            .await
            .context("Failed to execute /join"),
        Command::Leave => handle_leave(ctx, command)
            .await
            .context("Failed to execute /leave"),
        Command::Skip => handle_skip(ctx, command)
            .await
            .context("Failed to execute /skip"),
        Command::Voice => handle_voice(ctx, command)
            .await
            .context("Failed to execute /voice"),
        Command::DictAdd(option) => handle_dict_add(ctx, command, option)
            .await
            .context("Failed to execute /dict add"),
        Command::DictRemove(option) => handle_dict_remove(ctx, command, option)
            .await
            .context("Failed to execute /dict remove"),
        Command::DictView => handle_dict_view(ctx, command)
            .await
            .context("Failed to execute /dict view"),
        Command::Help => handle_help(ctx, command)
            .await
            .context("Failed to execute /help"),
        Command::Unknown => {
            bail!("Unknown command: {:?}", command);
        }
    }
}

async fn handle_join(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<CommandResponse> {
    let guild_id = match command.guild_id {
        Some(id) => id,
        None => return Ok("`/join`, `/kjoin` はサーバー内でのみ使えます。".into()),
    };
    let user_id = command.user.id;
    let text_channel_id = command.channel_id;

    let voice_channel_id = match get_user_voice_channel(ctx, &guild_id, &user_id).await? {
        Some(channel) => channel,
        None => {
            return Ok("ボイスチャンネルに接続してから `/join` を送信してください。".into());
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

    Ok("接続しました。".into())
}

async fn handle_leave(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<CommandResponse> {
    let guild_id = match command.guild_id {
        Some(id) => id,
        None => return Ok("`/leave`, `/kleave` はサーバー内でのみ使えます。".into()),
    };

    if !koe_call::is_connected(ctx, guild_id).await? {
        return Ok("どのボイスチャンネルにも接続していません。".into());
    }

    koe_call::leave(ctx, guild_id).await?;

    let state = app_state::get(ctx).await?;
    state.connected_guild_states.remove(&guild_id);

    Ok("切断しました。".into())
}

async fn handle_skip(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<CommandResponse> {
    let guild_id = match command.guild_id {
        Some(id) => id,
        None => return Ok("`/skip`, `/kskip` はサーバー内でのみ使えます。".into()),
    };

    if !koe_call::is_connected(ctx, guild_id).await? {
        return Ok("どのボイスチャンネルにも接続していません。".into());
    }

    koe_call::skip(ctx, guild_id).await?;

    Ok("読み上げ中のメッセージをスキップしました。".into())
}

async fn handle_voice(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<CommandResponse> {
    let guild_id = match command.guild_id {
        Some(id) => id,
        None => return Ok("`/voice` はサーバー内でのみ使えます。".into()),
    };

    let state = app_state::get(ctx).await?;
    let mut conn = state.redis_client.get_async_connection().await?;

    let available_presets = state.voicevox_client.presets().await?;

    let fallback_preset_id = available_presets
        .choose(&mut rand::thread_rng())
        .map(|p| p.id)
        .ok_or_else(|| anyhow!("No presets available"))?;
    let current_preset = koe_db::voice::get(
        &mut conn,
        GetOption {
            guild_id: guild_id.to_string(),
            user_id: command.user.id.to_string(),
            fallback: fallback_preset_id,
        },
    )
    .await?;

    let components = {
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
        select.custom_id("voice");
        select.options(|create_options| create_options.set_options(option_list));

        let mut action_row = CreateActionRow::default();
        action_row.add_select_menu(select);

        let mut components = CreateComponents::default();
        components.add_action_row(action_row);
        components
    };

    Ok(CommandResponse::Components(components))
}

async fn handle_dict_add(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    option: DictAddOption,
) -> Result<CommandResponse> {
    let guild_id = match command.guild_id {
        Some(id) => id,
        None => return Ok("`/dict add` はサーバー内でのみ使えます。".into()),
    };

    let state = app_state::get(ctx).await?;
    let mut conn = state.redis_client.get_async_connection().await?;

    let resp = koe_db::dict::insert(
        &mut conn,
        InsertOption {
            guild_id: guild_id.to_string(),
            word: option.word.clone(),
            read_as: option.read_as.clone(),
        },
    )
    .await?;

    match resp {
        InsertResponse::Success => Ok(format!(
            "{}の読み方を{}として辞書に登録しました。",
            sanitize_response(&option.word),
            sanitize_response(&option.read_as)
        )
        .into()),
        InsertResponse::WordAlreadyExists => Ok(format!(
            "すでに{}は辞書に登録されています。",
            sanitize_response(&option.word)
        )
        .into()),
    }
}

async fn handle_dict_remove(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    option: DictRemoveOption,
) -> Result<CommandResponse> {
    let guild_id = match command.guild_id {
        Some(id) => id,
        None => return Ok("`/dict remove` はサーバー内でのみ使えます。".into()),
    };

    let state = app_state::get(ctx).await?;
    let mut conn = state.redis_client.get_async_connection().await?;

    let resp = koe_db::dict::remove(
        &mut conn,
        RemoveOption {
            guild_id: guild_id.to_string(),
            word: option.word.clone(),
        },
    )
    .await?;

    match resp {
        RemoveResponse::Success => Ok(format!(
            "辞書から{}を削除しました。",
            sanitize_response(&option.word)
        )
        .into()),
        RemoveResponse::WordDoesNotExist => Ok(format!(
            "{}は辞書に登録されていません。",
            sanitize_response(&option.word)
        )
        .into()),
    }
}

async fn handle_dict_view(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<CommandResponse> {
    let guild_id = match command.guild_id {
        Some(id) => id,
        None => return Ok("`/dict view` はサーバー内でのみ使えます。".into()),
    };

    let state = app_state::get(ctx).await?;
    let mut conn = state.redis_client.get_async_connection().await?;

    let dict = koe_db::dict::get_all(
        &mut conn,
        GetAllOption {
            guild_id: guild_id.to_string(),
        },
    )
    .await?;

    let mut embed = CreateEmbed::default();

    let guild_name = guild_id
        .name(&ctx.cache)
        .await
        .unwrap_or_else(|| "サーバー".to_string());
    embed.title(format!("📕 {}の辞書", guild_name));

    embed.fields(
        dict.into_iter()
            .map(|(word, read_as)| (word, sanitize_response(&read_as), false)),
    );

    Ok(CommandResponse::Embed(embed))
}

async fn handle_help(
    _ctx: &Context,
    _command: &ApplicationCommandInteraction,
) -> Result<CommandResponse> {
    Ok("使い方はこちらをご覧ください:\nhttps://github.com/ciffelia/koe/blob/main/README.md".into())
}

async fn get_user_voice_channel(
    ctx: &Context,
    guild_id: &GuildId,
    user_id: &UserId,
) -> Result<Option<ChannelId>> {
    let guild = guild_id
        .to_guild_cached(&ctx.cache)
        .await
        .context("Failed to find guild in the cache")?;

    let channel_id = guild
        .voice_states
        .get(user_id)
        .and_then(|voice_state| voice_state.channel_id);

    Ok(channel_id)
}

pub fn sanitize_response(text: &str) -> String {
    format!("`{}`", text.replace('`', ""))
}

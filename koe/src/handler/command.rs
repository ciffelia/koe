use crate::context_store;
use crate::speech::{NewSpeechQueueOption, SpeechQueue};
use crate::status::{VoiceConnectionStatus, VoiceConnectionStatusMap};
use crate::voice_client::VoiceClient;
use anyhow::{Context as _, Result};
use koe_db::dict::{GetAllOption, InsertOption, InsertResponse, RemoveOption, RemoveResponse};
use koe_db::redis;
use koe_speech::SpeechProvider;
use log::error;
use serenity::{
    client::Context,
    model::{
        id::{ChannelId, GuildId, UserId},
        interactions::{
            application_command::{
                ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
            },
            InteractionResponseType,
        },
    },
};

pub async fn handle_command(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<()> {
    let response_text = execute_command(ctx, command).await;

    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(response_text))
        })
        .await
        .context("Failed to create interaction response")?;

    Ok(())
}

async fn execute_command(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
    let res = match command.data.name.as_str() {
        "join" | "kjoin" => handle_join(ctx, command).await,
        "leave" | "kleave" => handle_leave(ctx, command).await,
        "dict" => handle_dict(ctx, command).await,
        "help" => handle_help(ctx, command).await,
        _ => Ok("エラー: コマンドが登録されていません。".to_string()),
    };

    match res {
        Ok(message) => message,
        Err(err) => {
            error!("Error while executing command: {}", err);
            "内部エラーが発生しました。".to_string()
        }
    }
}

async fn handle_join(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<String> {
    let guild_id = match command.guild_id {
        Some(id) => id,
        None => return Ok("`/join`, `/kjoin` はサーバー内でのみ使えます。".to_string()),
    };
    let user_id = command.user.id;
    let text_channel_id = command.channel_id;

    let voice_channel_id = match get_user_voice_channel(ctx, &guild_id, &user_id).await? {
        Some(channel) => channel,
        None => {
            return Ok("あなたはボイスチャンネルに接続していません。".to_string());
        }
    };

    let voice_client = context_store::extract::<VoiceClient>(ctx).await?;
    let call = voice_client.join(ctx, guild_id, voice_channel_id).await?;

    let speech_provider = context_store::extract::<SpeechProvider>(ctx).await?;

    let status_map = context_store::extract::<VoiceConnectionStatusMap>(ctx).await?;
    status_map.insert(
        guild_id,
        VoiceConnectionStatus {
            bound_text_channel: text_channel_id,
            last_message_read: None,
            speech_queue: SpeechQueue::new(NewSpeechQueueOption {
                guild_id,
                speech_provider,
                call,
            }),
        },
    );

    Ok("接続しました。".to_string())
}

async fn handle_leave(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<String> {
    let guild_id = match command.guild_id {
        Some(id) => id,
        None => return Ok("`/leave`, `/kleave` はサーバー内でのみ使えます。".to_string()),
    };

    let voice_client = context_store::extract::<VoiceClient>(ctx).await?;

    if !voice_client.is_connected(ctx, guild_id).await? {
        return Ok("どのボイスチャンネルにも接続していません。".to_string());
    }

    voice_client.leave(ctx, guild_id).await?;

    let status_map = context_store::extract::<VoiceConnectionStatusMap>(ctx).await?;
    status_map.remove(&guild_id);

    Ok("切断しました。".to_string())
}

async fn handle_dict(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<String> {
    let option = match command.data.options.get(0) {
        Some(option) => option,
        None => return Ok("エラー: サブコマンドを認識できません。".to_string()),
    };

    match option.name.as_str() {
        "add" => handle_dict_add(ctx, command).await,
        "remove" => handle_dict_remove(ctx, command).await,
        "view" => handle_dict_view(ctx, command).await,
        _ => Ok("エラー: サブコマンドが登録されていません。".to_string()),
    }
}

async fn handle_dict_add(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<String> {
    let guild_id = match command.guild_id {
        Some(id) => id,
        None => return Ok("`/dict add` はサーバー内でのみ使えます。".to_string()),
    };

    let option_word = match command.data.options[0].options.get(0) {
        Some(option) => option,
        None => return Ok("エラー: 語句を認識できません。".to_string()),
    };
    let option_read_as = match command.data.options[0].options.get(1) {
        Some(option) => option,
        None => return Ok("エラー: 読み方を認識できません。".to_string()),
    };

    let word = match &option_word.resolved {
        Some(ApplicationCommandInteractionDataOptionValue::String(val)) => val,
        _ => return Ok("エラー: 語句が文字列として入力されていません。".to_string()),
    };
    let read_as = match &option_read_as.resolved {
        Some(ApplicationCommandInteractionDataOptionValue::String(val)) => val,
        _ => return Ok("エラー: 読み方が文字列として入力されていません。".to_string()),
    };

    let client = context_store::extract::<redis::Client>(ctx).await?;
    let mut conn = client.get_async_connection().await?;

    let resp = koe_db::dict::insert(
        &mut conn,
        InsertOption {
            guild_id: guild_id.to_string(),
            word: word.clone(),
            read_as: read_as.clone(),
        },
    )
    .await?;

    match resp {
        InsertResponse::Success => Ok(format!(
            "{}の読み方を{}として辞書に登録しました。",
            word, read_as
        )),
        InsertResponse::WordAlreadyExists => {
            Ok(format!("すでに{}は辞書に登録されています。", word,))
        }
    }
}

async fn handle_dict_remove(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<String> {
    let guild_id = match command.guild_id {
        Some(id) => id,
        None => return Ok("`/dict remove` はサーバー内でのみ使えます。".to_string()),
    };

    let option_word = match command.data.options[0].options.get(0) {
        Some(option) => option,
        None => return Ok("エラー: 語句を認識できません。".to_string()),
    };

    let word = match &option_word.resolved {
        Some(ApplicationCommandInteractionDataOptionValue::String(val)) => val,
        _ => return Ok("エラー: 語句が文字列として入力されていません。".to_string()),
    };

    let client = context_store::extract::<redis::Client>(ctx).await?;
    let mut conn = client.get_async_connection().await?;

    let resp = koe_db::dict::remove(
        &mut conn,
        RemoveOption {
            guild_id: guild_id.to_string(),
            word: word.clone(),
        },
    )
    .await?;

    match resp {
        RemoveResponse::Success => Ok(format!("辞書から{}を削除しました。", word)),
        RemoveResponse::WordDoesNotExist => Ok(format!("{}は辞書に登録されていません。", word,)),
    }
}

async fn handle_dict_view(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<String> {
    let guild_id = match command.guild_id {
        Some(id) => id,
        None => return Ok("`/dict view` はサーバー内でのみ使えます。".to_string()),
    };

    let client = context_store::extract::<redis::Client>(ctx).await?;
    let mut conn = client.get_async_connection().await?;

    let dict = koe_db::dict::get_all(
        &mut conn,
        GetAllOption {
            guild_id: guild_id.to_string(),
        },
    )
    .await?;

    let dict_str = dict
        .into_iter()
        .map(|(word, read_as)| format!("{}: {}", &word, &read_as))
        .collect::<Vec<_>>()
        .join("\n");

    Ok(format!("**サーバー辞書**\n{}", dict_str))
}

async fn handle_help(_ctx: &Context, _command: &ApplicationCommandInteraction) -> Result<String> {
    Ok(
        "使い方はこちらをご覧ください:\nhttps://github.com/ciffelia/koe/blob/main/README.md"
            .to_string(),
    )
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

use anyhow::{Context as _, Result, bail};
use serenity::{
    builder::{
        CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu,
        CreateSelectMenuKind, CreateSelectMenuOption,
    },
    client::Context,
    model::application::{ComponentInteraction, ComponentInteractionDataKind},
};

use super::super::respond_text;
use crate::{
    app_state,
    db::{self, voice::SetOption},
};

const CUSTOM_ID_VOICE_SELECT: &str = "voice";

pub fn custom_id_matches(custom_id: &str) -> bool {
    custom_id == CUSTOM_ID_VOICE_SELECT
}

pub fn component(
    available_presets: &[crate::tts::voicevox::Preset],
    current_preset: i64,
) -> CreateSelectMenu {
    let option_list = available_presets
        .iter()
        .map(|p| {
            CreateSelectMenuOption::new(&p.name, p.id.to_string())
                .default_selection(p.id == current_preset)
        })
        .collect();

    CreateSelectMenu::new(
        CUSTOM_ID_VOICE_SELECT,
        CreateSelectMenuKind::String {
            options: option_list,
        },
    )
}

pub async fn handle_interaction(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    let guild_id = interaction
        .guild_id
        .context("Guild ID not available in interaction")?;

    let ComponentInteractionDataKind::StringSelect { values } = &interaction.data.kind else {
        bail!("Expected string select interaction")
    };

    let selected_preset_id: i64 = values
        .first()
        .context("Value not available in message component interaction")?
        .parse()?;

    let state = app_state::get(ctx).await?;

    let available_presets = state.voicevox_client.presets().await?;
    let Some(selected_preset) = available_presets
        .into_iter()
        .find(|p| p.id == selected_preset_id)
    else {
        let message = CreateInteractionResponseMessage::new()
            .content("選択された話者が見つかりませんでした。既に削除された可能性があります。")
            .ephemeral(true);

        interaction
            .create_response(&ctx.http, CreateInteractionResponse::Message(message))
            .await
            .context("Failed to create interaction response")?;

        return Ok(());
    };

    let mut conn = state
        .redis_client
        .get_multiplexed_async_connection()
        .await?;
    db::voice::set(
        &mut conn,
        SetOption {
            guild_id: guild_id.into(),
            user_id: interaction.user.id.into(),
            value: selected_preset_id,
        },
    )
    .await?;

    respond_text(
        ctx,
        interaction,
        format!(
            "<@{}>の声を`{}`に設定しました。",
            interaction.user.id, selected_preset.name
        ),
    )
    .await?;
    Ok(())
}

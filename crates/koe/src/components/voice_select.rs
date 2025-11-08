use anyhow::{Result, anyhow, bail};
use koe_db::voice::{GetOption, SetOption};
use rand::seq::IndexedRandom;
use serenity::{
    builder::{CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption},
    client::Context,
    model::{
        application::{ComponentInteraction, ComponentInteractionDataKind},
        id::{GuildId, UserId},
    },
};

use crate::app_state;

const CUSTOM_ID_VOICE_SELECT: &str = "voice";

pub fn custom_id_matches(custom_id: &str) -> bool {
    custom_id == CUSTOM_ID_VOICE_SELECT
}

pub async fn component(
    ctx: &Context,
    guild_id: GuildId,
    user_id: UserId,
) -> Result<CreateSelectMenu> {
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
            user_id: user_id.into(),
            fallback: fallback_preset_id,
        },
    )
    .await?;

    let option_list = available_presets
        .iter()
        .map(|p| {
            CreateSelectMenuOption::new(&p.name, p.id.to_string())
                .default_selection(p.id == current_preset)
        })
        .collect::<Vec<_>>();

    let select_menu = CreateSelectMenu::new(
        CUSTOM_ID_VOICE_SELECT,
        CreateSelectMenuKind::String {
            options: option_list,
        },
    );

    Ok(select_menu)
}

pub async fn handle(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    let guild_id = interaction
        .guild_id
        .ok_or_else(|| anyhow!("Failed to get guild ID"))?;

    let ComponentInteractionDataKind::StringSelect { values } = &interaction.data.kind else {
        bail!("Expected string select interaction")
    };

    let selected_preset_id = values
        .first()
        .ok_or_else(|| anyhow!("Value not available in message component interaction"))?
        .parse::<i64>()?;

    let state = app_state::get(ctx).await?;

    let available_presets = state.voicevox_client.presets().await?;
    let selected_preset = available_presets
        .into_iter()
        .find(|p| p.id == selected_preset_id)
        .ok_or_else(|| anyhow!("Preset {selected_preset_id} not available"))?;

    let mut conn = state
        .redis_client
        .get_multiplexed_async_connection()
        .await?;
    koe_db::voice::set(
        &mut conn,
        SetOption {
            guild_id: guild_id.into(),
            user_id: interaction.user.id.into(),
            value: selected_preset_id,
        },
    )
    .await?;

    super::r(
        ctx,
        interaction,
        format!(
            "<@{}>さんの声を`{}`に変更しました。",
            interaction.user.id, selected_preset.name
        ),
    )
    .await?;
    Ok(())
}

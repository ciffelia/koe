use anyhow::{Result, anyhow};
use koe_db::voice::GetOption;
use poise::{CreateReply, serenity_prelude as serenity};
use rand::seq::IndexedRandom;
use serenity::builder::{
    CreateActionRow, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption,
};

use crate::{app_state, component_interaction::custom_id};

/// 読み上げの声を変更
#[poise::command(slash_command, guild_only)]
pub async fn voice(ctx: app_state::Context<'_>) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();
    let user_id = ctx.author().id;
    let state = ctx.data();

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
        custom_id::CUSTOM_ID_VOICE,
        CreateSelectMenuKind::String {
            options: option_list,
        },
    );

    let action_row = CreateActionRow::SelectMenu(select_menu);

    ctx.send(
        CreateReply::default()
            .ephemeral(true)
            .components(vec![action_row]),
    )
    .await?;

    Ok(())
}

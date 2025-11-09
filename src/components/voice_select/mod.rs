mod button;
mod select;

use anyhow::{Context as _, Result, bail};
use log::warn;
use rand::seq::IndexedRandom as _;
use serenity::{
    builder::CreateActionRow,
    client::Context,
    model::{
        application::ComponentInteraction,
        id::{GuildId, UserId},
    },
};

use crate::{
    app_state,
    db::{self, voice::GetOption},
};

pub fn custom_id_matches(custom_id: &str) -> bool {
    select::custom_id_matches(custom_id) || button::custom_id_matches(custom_id)
}

pub async fn components(
    ctx: &Context,
    guild_id: GuildId,
    user_id: UserId,
    page_idx: Option<usize>,
) -> Result<Vec<CreateActionRow>> {
    let state = app_state::get(ctx).await?;

    let available_presets = state.voicevox_client.presets().await?;
    let fallback_preset_id = available_presets
        .choose(&mut rand::rng())
        .map(|p| p.id)
        .context("No presets available")?;

    let mut conn = state
        .redis_client
        .get_multiplexed_async_connection()
        .await?;
    let current_preset = db::voice::get(
        &mut conn,
        GetOption {
            guild_id: guild_id.into(),
            user_id: user_id.into(),
            fallback: fallback_preset_id,
        },
    )
    .await?;

    const MAX_ITEMS_PER_PAGE: usize = 25;
    const MAX_BUTTONS_PER_ACTION_ROW: usize = 5;
    const MAX_BUTTON_ACTION_ROWS: usize = 4;
    const MAX_PAGES: usize = MAX_BUTTONS_PER_ACTION_ROW * MAX_BUTTON_ACTION_ROWS;
    if available_presets.len() <= MAX_ITEMS_PER_PAGE {
        Ok(vec![CreateActionRow::SelectMenu(select::component(
            &available_presets,
            current_preset,
        ))])
    } else {
        if available_presets.len() > MAX_ITEMS_PER_PAGE * MAX_PAGES {
            warn!(
                "Number of available presets ({}) exceeds the maximum supported ({}). Truncating \
                 the list.",
                available_presets.len(),
                MAX_ITEMS_PER_PAGE * MAX_PAGES
            );
        }

        let pages = available_presets
            .chunks(MAX_ITEMS_PER_PAGE)
            .take(MAX_PAGES)
            .collect::<Vec<_>>();
        let current_preset_page_idx = pages
            .iter()
            .position(|page| page.iter().any(|p| p.id == current_preset));
        let page_idx = page_idx.or(current_preset_page_idx).unwrap_or(0);

        let select_menu = select::component(pages[page_idx], current_preset);
        let buttons: Vec<_> = pages
            .iter()
            .enumerate()
            .map(|(idx, _)| button::component(idx, idx != page_idx))
            .collect();

        let mut action_rows = vec![CreateActionRow::SelectMenu(select_menu)];
        action_rows.extend(
            buttons
                .chunks(5)
                .map(|chunk| CreateActionRow::Buttons(chunk.to_vec())),
        );

        Ok(action_rows)
    }
}

pub async fn handle_interaction(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
    if select::custom_id_matches(&interaction.data.custom_id) {
        select::handle_interaction(ctx, interaction).await
    } else if button::custom_id_matches(&interaction.data.custom_id) {
        button::handle_interaction(ctx, interaction).await
    } else {
        bail!(
            "Unknown custom_id for voice select component: {}",
            interaction.data.custom_id
        )
    }
}

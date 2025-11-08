use anyhow::{Context as _, Result, anyhow};
use koe_db::voice::GetOption;
use rand::seq::IndexedRandom;
use serenity::{
    builder::{
        CreateActionRow, CreateCommand, CreateInteractionResponse,
        CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind,
        CreateSelectMenuOption,
    },
    client::Context,
    model::application::{CommandInteraction, InteractionContext},
};

use crate::{app_state, component_interaction::custom_id};

pub fn commands() -> Vec<CreateCommand> {
    vec![
        CreateCommand::new("voice")
            .description("話者の設定")
            .contexts(vec![InteractionContext::Guild]),
    ]
}

pub fn matches(cmd: &CommandInteraction) -> bool {
    cmd.data.name == "voice"
}

pub async fn handle(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    let guild_id = cmd.guild_id.expect("guild_id is Some");

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

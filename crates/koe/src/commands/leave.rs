use anyhow::Result;

use crate::app_state;

/// ボイスチャンネルから切断
#[poise::command(slash_command, aliases("kleave"), guild_only)]
pub async fn leave(ctx: app_state::Context<'_>) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();

    if !koe_call::is_connected(ctx.serenity_context(), guild_id).await? {
        ctx.say("どのボイスチャンネルにも接続していません。")
            .await?;
        return Ok(());
    }

    koe_call::leave(ctx.serenity_context(), guild_id).await?;

    let state = ctx.data();
    state.connected_guild_states.remove(&guild_id);

    ctx.say("切断しました。").await?;
    Ok(())
}

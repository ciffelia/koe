use anyhow::Result;

use crate::app_state;

/// 読み上げ中のメッセージをスキップ
#[poise::command(slash_command, aliases("kskip"), guild_only)]
pub async fn skip(ctx: app_state::Context<'_>) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();

    if !koe_call::is_connected(ctx.serenity_context(), guild_id).await? {
        ctx.say("どのボイスチャンネルにも接続していません。")
            .await?;
        return Ok(());
    }

    koe_call::skip(ctx.serenity_context(), guild_id).await?;

    ctx.say("読み上げ中のメッセージをスキップしました。")
        .await?;
    Ok(())
}

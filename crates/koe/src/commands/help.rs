use anyhow::Result;

use crate::app_state;

/// 使い方を表示
#[poise::command(slash_command)]
pub async fn help(ctx: app_state::Context<'_>) -> Result<()> {
    ctx.say("使い方はこちらをご覧ください:\nhttps://github.com/ciffelia/koe/blob/main/docs/user_guide.md")
        .await?;
    Ok(())
}

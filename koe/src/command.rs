use anyhow::{Context as _, Result};
use serenity::{
    client::Context,
    model::{id::GuildId, interactions::application_command::ApplicationCommand},
};

pub async fn setup_global_commands(ctx: &Context) -> Result<()> {
    ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
        // TODO: add global application commands
        commands
    })
    .await
    .context("Failed to set global application commands")?;

    Ok(())
}

pub async fn setup_guild_commands(ctx: &Context, guild_id: GuildId) -> Result<()> {
    guild_id
        .set_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("help").description("使い方を表示")
                })
                .create_application_command(|command| {
                    command
                        .name("join")
                        .description("ボイスチャンネルに接続し、読み上げを開始")
                })
                .create_application_command(|command| {
                    command
                        .name("kjoin")
                        .description("ボイスチャンネルに接続し、読み上げを開始")
                })
                .create_application_command(|command| {
                    command
                        .name("leave")
                        .description("ボイスチャンネルから退出")
                })
                .create_application_command(|command| {
                    command
                        .name("kleave")
                        .description("ボイスチャンネルから退出")
                })
        })
        .await
        .context("Failed to set guild application commands")?;

    Ok(())
}

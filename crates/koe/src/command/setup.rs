use anyhow::{Context as _, Result};
use serenity::{
    client::Context,
    model::{application::command::CommandOptionType, id::GuildId},
};

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
                .create_application_command(|command| {
                    command
                        .name("skip")
                        .description("読み上げ中のメッセージをスキップ")
                })
                .create_application_command(|command| {
                    command
                        .name("kskip")
                        .description("読み上げ中のメッセージをスキップ")
                })
                .create_application_command(|command| {
                    command.name("voice").description("話者の設定")
                })
                .create_application_command(|command| {
                    command
                        .name("dict")
                        .description("読み上げ辞書の閲覧と編集")
                        .create_option(|option| {
                            option
                                .name("add")
                                .description("辞書に項目を追加")
                                .kind(CommandOptionType::SubCommand)
                                .create_sub_option(|option| {
                                    option
                                        .name("word")
                                        .description("読み方を指定したい語句")
                                        .kind(CommandOptionType::String)
                                        .required(true)
                                })
                                .create_sub_option(|option| {
                                    option
                                        .name("read-as")
                                        .description("語句の読み方")
                                        .kind(CommandOptionType::String)
                                        .required(true)
                                })
                        })
                        .create_option(|option| {
                            option
                                .name("remove")
                                .description("辞書から項目を削除")
                                .kind(CommandOptionType::SubCommand)
                                .create_sub_option(|option| {
                                    option
                                        .name("word")
                                        .description("削除したい語句")
                                        .kind(CommandOptionType::String)
                                        .required(true)
                                })
                        })
                        .create_option(|option| {
                            option
                                .name("view")
                                .description("辞書を表示")
                                .kind(CommandOptionType::SubCommand)
                        })
                })
        })
        .await
        .context("Failed to set guild application commands")?;

    Ok(())
}

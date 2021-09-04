use anyhow::{Context as _, Result};
use serenity::model::interactions::application_command::ApplicationCommandOptionType;
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
                .create_application_command(|command| {
                    command
                        .name("dict")
                        .description("読み上げ辞書の閲覧と編集")
                        .create_option(|option| {
                            option
                                .name("add")
                                .description("辞書に項目を追加")
                                .kind(ApplicationCommandOptionType::SubCommand)
                                .create_sub_option(|option| {
                                    option
                                        .name("word")
                                        .description("読み方を指定したい語句")
                                        .kind(ApplicationCommandOptionType::String)
                                        .required(true)
                                })
                                .create_sub_option(|option| {
                                    option
                                        .name("read-as")
                                        .description("語句の読み方")
                                        .kind(ApplicationCommandOptionType::String)
                                        .required(true)
                                })
                        })
                        .create_option(|option| {
                            option
                                .name("remove")
                                .description("辞書から項目を削除")
                                .kind(ApplicationCommandOptionType::SubCommand)
                                .create_sub_option(|option| {
                                    option
                                        .name("word")
                                        .description("削除したい語句")
                                        .kind(ApplicationCommandOptionType::String)
                                        .required(true)
                                })
                        })
                        .create_option(|option| {
                            option
                                .name("view")
                                .description("辞書を表示")
                                .kind(ApplicationCommandOptionType::SubCommand)
                        })
                })
                .create_application_command(|command| {
                    command
                        .name("voice")
                        .description("声質を設定")
                        .create_option(|option| {
                            option
                                .name("kind")
                                .description("声の種類を設定")
                                .kind(ApplicationCommandOptionType::SubCommand)
                                .create_sub_option(|option| {
                                    option
                                        .name("kind")
                                        .description("声の種類")
                                        .kind(ApplicationCommandOptionType::String)
                                        .required(true)
                                        .add_string_choice("A: 女性1", "A")
                                        .add_string_choice("B: 女性2（デフォルト）", "B")
                                        .add_string_choice("C: 男性1", "C")
                                        .add_string_choice("D: 男性2", "D")
                                })
                        })
                        .create_option(|option| {
                            option
                                .name("speed")
                                .description("声の速さを設定")
                                .kind(ApplicationCommandOptionType::SubCommand)
                                .create_sub_option(|option| {
                                    option
                                        .name("speed")
                                        .description("声の速さ (0.25 ～ 4.0, デフォルト: 1.3)")
                                        .kind(ApplicationCommandOptionType::Number)
                                        .required(true)
                                })
                        })
                        .create_option(|option| {
                            option
                                .name("pitch")
                                .description("声のピッチを設定")
                                .kind(ApplicationCommandOptionType::SubCommand)
                                .create_sub_option(|option| {
                                    option
                                        .name("pitch")
                                        .description(
                                            "声のピッチ (-20.0 ～ 20.0, 単位: 半音, デフォルト: 0)",
                                        )
                                        .kind(ApplicationCommandOptionType::Number)
                                        .required(true)
                                })
                        })
                })
        })
        .await
        .context("Failed to set guild application commands")?;

    Ok(())
}

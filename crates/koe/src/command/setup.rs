use anyhow::{Context as _, Result};
use serenity::{
    builder::{CreateCommand, CreateCommandOption},
    client::Context,
    model::{application::CommandOptionType, id::GuildId},
};

pub async fn setup_guild_commands(ctx: &Context, guild_id: GuildId) -> Result<()> {
    let commands = vec![
        CreateCommand::new("help").description("使い方を表示"),
        CreateCommand::new("join").description("ボイスチャンネルに接続し、読み上げを開始"),
        CreateCommand::new("kjoin").description("ボイスチャンネルに接続し、読み上げを開始"),
        CreateCommand::new("leave").description("ボイスチャンネルから退出"),
        CreateCommand::new("kleave").description("ボイスチャンネルから退出"),
        CreateCommand::new("skip").description("読み上げ中のメッセージをスキップ"),
        CreateCommand::new("kskip").description("読み上げ中のメッセージをスキップ"),
        CreateCommand::new("voice").description("話者の設定"),
        CreateCommand::new("dict")
            .description("読み上げ辞書の閲覧と編集")
            .add_option(
                CreateCommandOption::new(CommandOptionType::SubCommand, "add", "辞書に項目を追加")
                    .add_sub_option(
                        CreateCommandOption::new(
                            CommandOptionType::String,
                            "word",
                            "読み方を指定したい語句",
                        )
                        .required(true),
                    )
                    .add_sub_option(
                        CreateCommandOption::new(
                            CommandOptionType::String,
                            "read-as",
                            "語句の読み方",
                        )
                        .required(true),
                    ),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "remove",
                    "辞書から項目を削除",
                )
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::String, "word", "削除したい語句")
                        .required(true),
                ),
            )
            .add_option(CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "view",
                "辞書を表示",
            )),
    ];

    guild_id
        .set_commands(&ctx.http, commands)
        .await
        .context("Failed to set guild commands")?;

    Ok(())
}

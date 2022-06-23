use anyhow::Result;
use redis::aio::Connection;

#[derive(Debug, Clone)]
pub struct GetOption {
    pub guild_id: String,
    pub user_id: String,
    pub fallback: i64,
}

/// ユーザーの声を返す
/// 未設定の場合は`option.fallback`の値を設定して返す
pub async fn get(connection: &mut Connection, option: GetOption) -> Result<i64> {
    let key = voice_key(&option.guild_id, &option.user_id);

    let (resp,) = redis::pipe()
        .set_nx(&key, option.fallback)
        .ignore()
        .get(&key)
        .query_async(connection)
        .await?;

    Ok(resp)
}

fn voice_key(guild_id: &str, user_id: &str) -> String {
    format!("guild:{}:user:{}:voice", guild_id, user_id)
}

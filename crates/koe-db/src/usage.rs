use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use redis::aio::Connection;

pub async fn add_and_get_chars_used_today(
    connection: &mut Connection,
    count: usize,
) -> Result<i64> {
    let (chars_used, _): (i64, i64) = redis::pipe()
        .atomic()
        .incr("chars:used", count)
        .expire_at("chars:used", next_usage_reset().timestamp() as usize)
        .query_async(connection)
        .await?;

    Ok(chars_used)
}

fn next_usage_reset() -> DateTime<Utc> {
    (Utc::today() + Duration::days(1)).and_hms(0, 0, 0)
}

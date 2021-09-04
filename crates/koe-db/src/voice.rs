use anyhow::Result;
use redis::aio::Connection;
use redis::AsyncCommands;

#[derive(Debug, Clone)]
pub struct SetKindOption {
    pub user_id: String,
    pub kind: String,
}

pub async fn set_kind(connection: &mut Connection, option: SetKindOption) -> Result<()> {
    connection
        .set(voice_kind_key(&option.user_id), option.kind)
        .await?;

    Ok(())
}

#[derive(Debug, Clone)]
pub struct SetSpeedOption {
    pub user_id: String,
    pub speed: f64,
}

pub async fn set_speed(connection: &mut Connection, option: SetSpeedOption) -> Result<()> {
    connection
        .set(voice_speed_key(&option.user_id), option.speed)
        .await?;

    Ok(())
}

#[derive(Debug, Clone)]
pub struct SetPitchOption {
    pub user_id: String,
    pub pitch: f64,
}

pub async fn set_pitch(connection: &mut Connection, option: SetPitchOption) -> Result<()> {
    connection
        .set(voice_pitch_key(&option.user_id), option.pitch)
        .await?;

    Ok(())
}

pub async fn get_kind(connection: &mut Connection, user_id: String) -> Result<Option<String>> {
    let res = connection.get(voice_kind_key(&user_id)).await?;

    Ok(res)
}

pub async fn get_speed(connection: &mut Connection, user_id: String) -> Result<Option<f64>> {
    let res = connection.get(voice_speed_key(&user_id)).await?;

    Ok(res)
}

pub async fn get_pitch(connection: &mut Connection, user_id: String) -> Result<Option<f64>> {
    let res = connection.get(voice_pitch_key(&user_id)).await?;

    Ok(res)
}

fn voice_kind_key(user_id: &str) -> String {
    format!("user:{}:voice:kind", user_id)
}

fn voice_speed_key(user_id: &str) -> String {
    format!("user:{}:voice:speed", user_id)
}

fn voice_pitch_key(user_id: &str) -> String {
    format!("user:{}:voice:pitch", user_id)
}

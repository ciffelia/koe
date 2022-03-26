use anyhow::{Context, Result};
use koe_audio::EncodedAudio;
use songbird::{
    input::{Codec, Container, Input, Reader},
    Call,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn enqueue(call: Arc<Mutex<Call>>, audio: EncodedAudio) -> Result<()> {
    let decoded = audio.decode().await?;

    let mut handler = call.lock().await;
    handler.enqueue_source(Input::new(
        false,
        Reader::from_memory(decoded.into()),
        Codec::Pcm,
        Container::Raw,
        None,
    ));

    Ok(())
}

pub async fn skip(call: Arc<Mutex<Call>>) -> Result<()> {
    let handler = call.lock().await;
    let current_track = handler.queue().current();

    if let Some(track) = current_track {
        track.stop().context("Failed to stop current track")?;
    }

    Ok(())
}

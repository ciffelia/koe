use anyhow::{bail, Context, Result};
use log::trace;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

/// Convert any type of audio source into 16-bit signed little-endian samples (i.e. wav) with ffmpeg.
pub async fn convert_to_pcm_s16le(source: Vec<u8>) -> Result<Vec<u8>> {
    let mut child = Command::new("ffmpeg")
        // input: stdin
        .args(["-i", "pipe:"])
        // format: 16-bit signed little-endian
        .args(["-f", "s16le"])
        // channels: 1 (mono)
        .args(["-ac", "1"])
        // sampling rate: 48kHz
        .args(["-ar", "48000"])
        // codec: pcm
        .args(["-acodec", "pcm_s16le"])
        // output: stdout
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to spawn ffmpeg")?;
    trace!("Spawned ffmpeg");

    // Write to stdin in another thread to avoid deadlock: https://doc.rust-lang.org/std/process/struct.Stdio.html#method.piped
    let mut stdin = child
        .stdin
        .take()
        .context("Failed to open ffmpeg's stdin")?;
    tokio::spawn(async move {
        stdin
            .write_all(&source)
            .await
            .expect("Failed to write to ffmpeg's stdin");
        trace!("Wrote to ffmpeg's stdin");
    });

    let out = child
        .wait_with_output()
        .await
        .context("Failed to read ffmpeg's output")?;
    trace!("Received ffmpeg's output");

    if !out.status.success() {
        bail!(
            "ffmpeg exited with code {}:\n{}",
            out.status,
            std::str::from_utf8(&out.stderr)?
        );
    }

    Ok(out.stdout)
}

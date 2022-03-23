use crate::ffmpeg::convert_to_pcm_s16le;
use anyhow::Result;

/// Representation of encoded (compressed) audio.
pub struct EncodedAudio(Vec<u8>);

impl EncodedAudio {
    /// Decode into [`DecodedAudio`] with ffmpeg.
    pub async fn decode(self) -> Result<DecodedAudio> {
        let decoded_buf = convert_to_pcm_s16le(self.0).await?;
        Ok(DecodedAudio::from(decoded_buf))
    }
}

impl From<Vec<u8>> for EncodedAudio {
    fn from(buf: Vec<u8>) -> Self {
        Self(buf)
    }
}

impl From<EncodedAudio> for Vec<u8> {
    fn from(audio: EncodedAudio) -> Self {
        audio.0
    }
}

/// Representation of wav audio (16-bit signed little-endian samples).
pub struct DecodedAudio(Vec<u8>);

impl From<Vec<u8>> for DecodedAudio {
    fn from(buf: Vec<u8>) -> Self {
        Self(buf)
    }
}

impl From<DecodedAudio> for Vec<u8> {
    fn from(audio: DecodedAudio) -> Self {
        audio.0
    }
}

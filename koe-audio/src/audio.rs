use crate::ffmpeg::convert_to_pcm_s16le;
use anyhow::Result;

/// Representation of encoded (compressed) audio.
pub struct EncodedAudio {
    data: Vec<u8>,
}

impl EncodedAudio {
    pub fn from(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn data(self) -> Vec<u8> {
        self.data
    }

    /// Decode into [`DecodedAudio`] with ffmpeg.
    pub async fn into_decoded(self) -> Result<DecodedAudio> {
        let decoded_data = convert_to_pcm_s16le(self.data).await?;
        Ok(DecodedAudio::from(decoded_data))
    }
}

/// Representation of wav audio (16-bit signed little-endian samples).
pub struct DecodedAudio {
    data: Vec<u8>,
}

impl DecodedAudio {
    pub fn from(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn data(self) -> Vec<u8> {
        self.data
    }
}

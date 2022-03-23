use crate::voicevox::{GenerateQueryParams, SynthesisParams, VoicevoxClient};
use anyhow::Result;
use koe_audio::EncodedAudio;

pub struct SpeechProvider {
    client: VoicevoxClient,
}

impl SpeechProvider {
    pub fn new(api_base: String) -> Self {
        Self {
            client: VoicevoxClient::new(api_base),
        }
    }

    pub async fn make_speech(&self, option: SpeechRequest) -> Result<EncodedAudio> {
        let query = self
            .client
            .generate_query(GenerateQueryParams {
                style_id: "1".to_string(),
                text: option.text,
            })
            .await?;

        let audio = self
            .client
            .synthesis(SynthesisParams {
                style_id: "1".to_string(),
                query,
            })
            .await?;

        Ok(audio)
    }
}

#[derive(Debug, Clone)]
pub struct SpeechRequest {
    pub text: String,
}

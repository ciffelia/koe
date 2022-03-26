use crate::voicevox::{GenerateQueryFromPresetParams, SynthesisParams, VoicevoxClient};
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
        let preset_list = self.client.get_presets().await?;
        let preset = &preset_list[0];

        let query = self
            .client
            .generate_query_from_preset(GenerateQueryFromPresetParams {
                preset_id: preset.id,
                text: option.text,
            })
            .await?;

        let audio = self
            .client
            .synthesis(SynthesisParams {
                style_id: preset.style_id,
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

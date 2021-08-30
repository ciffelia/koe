use crate::google_cloud;
use anyhow::{anyhow, Context, Result};
use google_texttospeech1::api::{
    AudioConfig, SynthesisInput, SynthesizeSpeechRequest, SynthesizeSpeechResponse,
    VoiceSelectionParams,
};
use google_texttospeech1::Texttospeech;
use koe_audio::EncodedAudio;
use std::path::Path;

pub struct SpeechProvider {
    hub: Texttospeech,
}

impl SpeechProvider {
    pub async fn new(service_account_key_path: impl AsRef<Path>) -> Result<Self> {
        let auth = google_cloud::auth(service_account_key_path).await?;
        let client =
            hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots());

        let hub = Texttospeech::new(client, auth);

        Ok(Self { hub })
    }

    pub async fn make_speech(&self, text: String) -> Result<EncodedAudio> {
        let (_, resp) = self
            .hub
            .text()
            .synthesize(SynthesizeSpeechRequest {
                input: Some(SynthesisInput {
                    text: Some(text),
                    ..Default::default()
                }),
                voice: Some(VoiceSelectionParams {
                    language_code: Some("ja-JP".to_string()),
                    name: Some("ja-JP-Wavenet-B".to_string()),
                    ..Default::default()
                }),
                audio_config: Some(AudioConfig {
                    audio_encoding: Some("OGG_OPUS".to_string()),
                    speaking_rate: Some(1.3),
                    ..Default::default()
                }),
            })
            .doit()
            .await
            .context("Failed to make text.synthesize request")?;

        let speech = SpeechProvider::parse_resp(resp)?;

        Ok(speech)
    }

    fn parse_resp(resp: SynthesizeSpeechResponse) -> Result<EncodedAudio> {
        let speech_base64 = resp.audio_content.ok_or_else(|| {
            anyhow!("No audio_content found in the response from Text-to-Speech API")
        })?;

        let speech_bytes =
            base64::decode(speech_base64).context("Failed to decode base64 audio_content")?;
        let speech = EncodedAudio::from(speech_bytes);

        Ok(speech)
    }
}

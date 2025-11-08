use anyhow::Result;
use reqwest::Url;
use serde::Deserialize;

pub struct VoicevoxClient {
    client: reqwest::Client,
    api_base: String,
}

impl VoicevoxClient {
    #[must_use]
    pub fn new(api_base: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_base,
        }
    }

    pub async fn generate_query_from_preset(
        &self,
        params: GenerateQueryFromPresetParams,
    ) -> Result<String> {
        let url = Url::parse_with_params(
            &self.get_endpoint("/audio_query_from_preset"),
            &[
                ("text", params.text),
                ("preset_id", params.preset_id.to_string()),
            ],
        )?;

        let resp = self
            .client
            .post(url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        Ok(resp)
    }

    pub async fn synthesis(&self, params: SynthesisParams) -> Result<Vec<u8>> {
        let url = Url::parse_with_params(
            &self.get_endpoint("/synthesis"),
            &[("speaker", params.style_id.to_string())],
        )?;

        let resp = self
            .client
            .post(url)
            .header("content-type", "application/json")
            .body(params.query)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(resp.to_vec())
    }

    pub async fn presets(&self) -> Result<Vec<Preset>> {
        let url = Url::parse(&self.get_endpoint("/presets"))?;

        let resp = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(resp)
    }

    pub async fn initialize_speaker(&self, speaker_id: i64) -> Result<()> {
        let url = Url::parse_with_params(
            &self.get_endpoint("/initialize_speaker"),
            &[("speaker", speaker_id.to_string())],
        )?;

        self.client.post(url).send().await?.error_for_status()?;

        Ok(())
    }

    fn get_endpoint(&self, path: impl AsRef<str>) -> String {
        self.api_base.clone() + path.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct GenerateQueryFromPresetParams {
    pub preset_id: i64,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct SynthesisParams {
    pub style_id: i64,
    pub query: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Preset {
    pub id: i64,
    pub name: String,
    pub speaker_uuid: String,
    pub style_id: i64,
    #[serde(rename = "speedScale")]
    pub speed_scale: f64,
    #[serde(rename = "pitchScale")]
    pub pitch_scale: f64,
    #[serde(rename = "intonationScale")]
    pub intonation_scale: f64,
    #[serde(rename = "volumeScale")]
    pub volume_scale: f64,
    #[serde(rename = "prePhonemeLength")]
    pub pre_phoneme_length: f64,
    #[serde(rename = "postPhonemeLength")]
    pub post_phoneme_length: f64,
}

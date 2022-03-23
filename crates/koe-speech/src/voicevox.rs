use anyhow::Result;
use koe_audio::EncodedAudio;
use reqwest::Url;

pub struct VoicevoxClient {
    client: reqwest::Client,
    api_base: String,
}

impl VoicevoxClient {
    pub fn new(api_base: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_base,
        }
    }

    pub async fn generate_query(&self, params: GenerateQueryParams) -> Result<String> {
        let url = Url::parse_with_params(
            &self.get_endpoint("/audio_query"),
            &[("text", params.text), ("speaker", params.style_id)],
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

    pub async fn synthesis(&self, params: SynthesisParams) -> Result<EncodedAudio> {
        let url = Url::parse_with_params(
            &self.get_endpoint("/synthesis"),
            &[("speaker", params.style_id)],
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

        Ok(EncodedAudio::from(resp.to_vec()))
    }

    fn get_endpoint(&self, path: impl AsRef<str>) -> String {
        self.api_base.clone() + path.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct GenerateQueryParams {
    pub style_id: String,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct SynthesisParams {
    pub style_id: String,
    pub query: String,
}

use anyhow::{Result, anyhow};

use crate::voicevox::{GenerateQueryFromPresetParams, Preset, SynthesisParams, VoicevoxClient};

pub async fn initialize_speakers(client: &VoicevoxClient) -> Result<()> {
    let preset_list = client.presets().await?;
    for preset in preset_list {
        client.initialize_speaker(preset.style_id).await?;
    }
    Ok(())
}

pub async fn make_speech(client: &VoicevoxClient, option: SpeechRequest) -> Result<Vec<u8>> {
    let preset = get_preset(client, option.preset_id).await?;

    let query = client
        .generate_query_from_preset(GenerateQueryFromPresetParams {
            preset_id: preset.id,
            text: option.text,
        })
        .await?;

    let audio = client
        .synthesis(SynthesisParams {
            style_id: preset.style_id,
            query,
        })
        .await?;

    Ok(audio)
}

pub async fn list_preset_ids(client: &VoicevoxClient) -> Result<Vec<PresetId>> {
    let preset_list = client.presets().await?;
    let ids = preset_list.into_iter().map(|p| PresetId(p.id)).collect();
    Ok(ids)
}

async fn get_preset(client: &VoicevoxClient, id: PresetId) -> Result<Preset> {
    let preset_list = client.presets().await?;

    let preset = preset_list
        .into_iter()
        .find(|p| PresetId(p.id) == id)
        .ok_or_else(|| anyhow!("Preset {} is not available", id.0))?;

    Ok(preset)
}

#[derive(Debug, Clone)]
pub struct SpeechRequest {
    pub text: String,
    pub preset_id: PresetId,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PresetId(pub i64);

impl From<i64> for PresetId {
    fn from(x: i64) -> Self {
        Self(x)
    }
}

impl From<&i64> for PresetId {
    fn from(x: &i64) -> Self {
        Self(*x)
    }
}

impl From<PresetId> for i64 {
    fn from(x: PresetId) -> Self {
        x.0
    }
}

impl From<&PresetId> for i64 {
    fn from(x: &PresetId) -> Self {
        x.0
    }
}

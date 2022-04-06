use anyhow::{anyhow, bail, Context as _, Result};
use koe_speech::PresetId;
use rand::seq::SliceRandom;
use serenity::model::id::UserId;
use std::collections::{hash_map::Entry, BTreeMap, HashMap};

pub struct VoicePresetRegistry {
    user_to_preset: HashMap<UserId, PresetId>,
    preset_usage: PresetUsageCache,
}

impl VoicePresetRegistry {
    pub fn new() -> Self {
        Self {
            user_to_preset: HashMap::new(),
            preset_usage: PresetUsageCache::new(),
        }
    }

    pub fn get(&self, user_id: UserId) -> Option<PresetId> {
        self.user_to_preset.get(&user_id).cloned()
    }

    pub fn insert(&mut self, user_id: UserId, preset_id: PresetId) -> Result<()> {
        if self.user_to_preset.contains_key(&user_id) {
            self.remove(user_id).context("Failed to remove item")?;
        }

        self.user_to_preset.insert(user_id, preset_id);
        self.preset_usage.increase(preset_id)?;

        Ok(())
    }

    pub fn remove(&mut self, user_id: UserId) -> Result<()> {
        let preset_id = match self.user_to_preset.remove(&user_id) {
            Some(preset_id) => preset_id,
            None => bail!("user_to_preset[{}] does not exist", user_id),
        };

        self.preset_usage.decrease(preset_id)?;

        Ok(())
    }

    pub async fn pick_least_used_preset(&self, preset_list: &[PresetId]) -> Result<PresetId> {
        self.preset_usage
            .pick_least_used(preset_list)
            .ok_or_else(|| anyhow!("No preset found"))
    }
}

struct PresetUsageCache {
    preset_to_usage: HashMap<PresetId, usize>,
}

impl PresetUsageCache {
    pub fn new() -> Self {
        Self {
            preset_to_usage: HashMap::new(),
        }
    }

    pub fn get(&self, preset_id: PresetId) -> usize {
        self.preset_to_usage.get(&preset_id).cloned().unwrap_or(0)
    }

    pub fn increase(&mut self, preset_id: PresetId) -> Result<()> {
        match self.preset_to_usage.entry(preset_id) {
            Entry::Occupied(mut entry) => {
                *entry.get_mut() += 1;
            }
            Entry::Vacant(entry) => {
                entry.insert(1);
            }
        }

        Ok(())
    }

    pub fn decrease(&mut self, preset_id: PresetId) -> Result<()> {
        match self.preset_to_usage.entry(preset_id) {
            Entry::Occupied(mut entry) => {
                *entry.get_mut() -= 1;
            }
            Entry::Vacant(_) => {
                bail!("preset_to_usage[{}] does not exist", preset_id.0);
            }
        }

        Ok(())
    }

    pub fn pick_least_used(&self, preset_list: &[PresetId]) -> Option<PresetId> {
        let mut usage_to_preset = BTreeMap::<usize, Vec<PresetId>>::new();

        for preset_id in preset_list {
            let usage = self.get(*preset_id);
            usage_to_preset.entry(usage).or_default().push(*preset_id);
        }

        usage_to_preset
            .into_values()
            .next()
            .unwrap_or_default()
            .choose(&mut rand::thread_rng())
            .cloned()
    }
}

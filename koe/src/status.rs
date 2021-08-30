use crate::speech::SpeechQueue;
use dashmap::DashMap;
use serenity::model::id::{ChannelId, GuildId};

pub type VoiceConnectionStatusMap = DashMap<GuildId, VoiceConnectionStatus>;

pub struct VoiceConnectionStatus {
    pub bound_text_channel: ChannelId,
    pub speech_queue: SpeechQueue,
}

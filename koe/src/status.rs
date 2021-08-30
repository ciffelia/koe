use crate::speech::SpeechQueue;
use dashmap::DashMap;
use serenity::model::{
    channel::Message,
    id::{ChannelId, GuildId},
};

pub type VoiceConnectionStatusMap = DashMap<GuildId, VoiceConnectionStatus>;

pub struct VoiceConnectionStatus {
    pub bound_text_channel: ChannelId,
    pub last_message_read: Option<Message>,
    pub speech_queue: SpeechQueue,
}

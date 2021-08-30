use anyhow::{bail, Context as _, Result};
use koe_speech::SpeechProvider;
use log::{debug, error};
use serenity::model::id::GuildId;
use songbird::input::{Codec, Container, Input, Reader};
use songbird::Call;
use std::sync::Arc;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;

pub struct SpeechQueue {
    text_sender: UnboundedSender<String>,
    command_sender: UnboundedSender<SpeechQueueWorkerCommand>,
}

impl SpeechQueue {
    pub fn new(option: NewSpeechQueueOption) -> Self {
        let (text_sender, text_receiver) = unbounded_channel();
        let (command_sender, command_receiver) = unbounded_channel();

        let worker = SpeechQueueWorker::new(NewSpeechQueueWorkerOption {
            text_receiver,
            command_receiver,
            guild_id: option.guild_id,
            speech_provider: option.speech_provider,
            call: option.call,
        });
        worker.start();

        Self {
            text_sender,
            command_sender,
        }
    }

    pub fn push(&mut self, text: String) -> Result<()> {
        self.text_sender.send(text)?;
        Ok(())
    }
}

impl Drop for SpeechQueue {
    fn drop(&mut self) {
        if let Err(err) = self
            .command_sender
            .send(SpeechQueueWorkerCommand::Terminate)
        {
            error!(
                "Failed to send terminate command to SpeechQueueWorker: {:?}",
                err
            );
        }
    }
}

pub struct NewSpeechQueueOption {
    pub guild_id: GuildId,
    pub speech_provider: Arc<SpeechProvider>,
    pub call: Arc<Mutex<Call>>,
}

struct SpeechQueueWorker {
    text_receiver: UnboundedReceiver<String>,
    command_receiver: UnboundedReceiver<SpeechQueueWorkerCommand>,
    guild_id: GuildId,
    speech_provider: Arc<SpeechProvider>,
    call: Arc<Mutex<Call>>,
}

impl SpeechQueueWorker {
    pub fn new(option: NewSpeechQueueWorkerOption) -> Self {
        Self {
            text_receiver: option.text_receiver,
            command_receiver: option.command_receiver,
            guild_id: option.guild_id,
            speech_provider: option.speech_provider,
            call: option.call,
        }
    }

    pub fn start(self) {
        tokio::task::spawn(async move {
            let guild_id = self.guild_id;
            debug!("SpeechQueueWorker for guild {} started", guild_id);

            if let Err(err) = self.work_loop().await {
                error!(
                    "SpeechQueueWorker for guild {} exited with error: {:?}",
                    guild_id, err
                );
            } else {
                debug!("SpeechQueueWorker for guild {} exited", guild_id);
            }
        });
    }

    async fn work_loop(mut self) -> Result<()> {
        loop {
            tokio::select! {
                biased;
                Some(SpeechQueueWorkerCommand::Terminate) = self.command_receiver.recv() => {
                    return Ok(());
                }
                Some(text) = self.text_receiver.recv() => {
                    self.speak(text).await.context("Failed to speak")?;
                }
                else => {
                    bail!("Both mpsc channels are closed");
                }
            };
        }
    }

    async fn speak(&self, text: String) -> Result<()> {
        let encoded_audio = self
            .speech_provider
            .make_speech(text)
            .await
            .context("Failed to execute Text-to-Speech")?;

        let decoded_audio = encoded_audio
            .into_decoded()
            .await
            .context("Failed to decode audio")?;

        let mut handler = self.call.lock().await;
        handler.enqueue_source(Input::new(
            false,
            Reader::from_memory(decoded_audio.data()),
            Codec::Pcm,
            Container::Raw,
            None,
        ));

        Ok(())
    }
}

struct NewSpeechQueueWorkerOption {
    pub text_receiver: UnboundedReceiver<String>,
    pub command_receiver: UnboundedReceiver<SpeechQueueWorkerCommand>,
    pub guild_id: GuildId,
    pub speech_provider: Arc<SpeechProvider>,
    pub call: Arc<Mutex<Call>>,
}

#[derive(Debug)]
enum SpeechQueueWorkerCommand {
    Terminate,
}

use anyhow::{Context as _, Result};
use koe_speech::{SpeechProvider, SpeechRequest};
use log::{debug, error};
use serenity::model::id::GuildId;
use songbird::input::{Codec, Container, Input, Reader};
use songbird::Call;
use std::sync::Arc;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;

pub struct SpeechQueue {
    request_sender: UnboundedSender<SpeechRequest>,
    command_sender: UnboundedSender<SpeechQueueWorkerCommand>,
}

impl SpeechQueue {
    pub fn new(option: NewSpeechQueueOption) -> Self {
        let (request_sender, request_receiver) = unbounded_channel();
        let (command_sender, command_receiver) = unbounded_channel();

        let worker = SpeechQueueWorker::new(NewSpeechQueueWorkerOption {
            request_receiver,
            command_receiver,
            guild_id: option.guild_id,
            speech_provider: option.speech_provider,
            call: option.call,
        });
        worker.start();

        Self {
            request_sender,
            command_sender,
        }
    }

    pub fn push(&self, request: SpeechRequest) -> Result<()> {
        self.request_sender
            .send(request)
            .context("Failed to send request to SpeechQueueWorker")?;

        Ok(())
    }

    pub fn skip(&self) -> Result<()> {
        self.command_sender
            .send(SpeechQueueWorkerCommand::Skip)
            .context("Failed to send skip command to SpeechQueueWorker")?;

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
    request_receiver: UnboundedReceiver<SpeechRequest>,
    command_receiver: UnboundedReceiver<SpeechQueueWorkerCommand>,
    guild_id: GuildId,
    speech_provider: Arc<SpeechProvider>,
    call: Arc<Mutex<Call>>,
}

impl SpeechQueueWorker {
    pub fn new(option: NewSpeechQueueWorkerOption) -> Self {
        Self {
            request_receiver: option.request_receiver,
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
            self.work_loop().await;
            debug!("SpeechQueueWorker for guild {} exiting", guild_id);
        });
    }

    async fn work_loop(mut self) {
        loop {
            tokio::select! {
                biased;
                Some(command) = self.command_receiver.recv() => {
                    match command {
                        SpeechQueueWorkerCommand::Skip => {
                            if let Err(err) = self.skip().await {
                                error!("Failed to skip current track: {:?}", err);
                            }
                        }
                        SpeechQueueWorkerCommand::Terminate => {
                            break;
                        }
                    }
                }
                Some(request) = self.request_receiver.recv() => {
                    if let Err(err) = self.speak(request).await {
                        error!("Failed to speak: {:?}", err);
                    }
                }
                else => {
                    error!("Both mpsc channels are closed");
                    break;
                }
            };
        }
    }

    async fn speak(&self, request: SpeechRequest) -> Result<()> {
        let encoded_audio = self
            .speech_provider
            .make_speech(request)
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

    async fn skip(&self) -> Result<()> {
        let handler = self.call.lock().await;
        let current_track = handler.queue().current();

        if let Some(track) = current_track {
            track.stop().context("Failed to stop current track")?;
        }

        Ok(())
    }
}

struct NewSpeechQueueWorkerOption {
    pub request_receiver: UnboundedReceiver<SpeechRequest>,
    pub command_receiver: UnboundedReceiver<SpeechQueueWorkerCommand>,
    pub guild_id: GuildId,
    pub speech_provider: Arc<SpeechProvider>,
    pub call: Arc<Mutex<Call>>,
}

#[derive(Debug)]
enum SpeechQueueWorkerCommand {
    Skip,
    Terminate,
}

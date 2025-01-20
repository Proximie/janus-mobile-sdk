use crate::base_handle;
use crate::error::JanusGatewayCommunicationError;
use crate::protocol::Candidate;
use crate::protocol::GenericEvent;
use crate::protocol::Jsep;
use jarust::plugins::audio_bridge::events::PluginEvent;
use jarust::plugins::audio_bridge::handle::AudioBridgeHandle as JaAudioBridgeHandle;
use serde_json::Value;
use std::fmt::Debug;
use std::sync::Mutex;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task::AbortHandle;

#[derive(uniffi::Object)]
pub struct AudioBridgeHandle {
    inner: JaAudioBridgeHandle,
    receiver: Mutex<Option<mpsc::UnboundedReceiver<PluginEvent>>>,
    abort_handle: Mutex<Option<AbortHandle>>,
}

impl AudioBridgeHandle {
    pub fn new(
        handle: JaAudioBridgeHandle,
        receiver: mpsc::UnboundedReceiver<PluginEvent>,
    ) -> Self {
        Self {
            inner: handle,
            receiver: Mutex::new(Some(receiver)),
            abort_handle: Mutex::new(None),
        }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl AudioBridgeHandle {
    pub async fn start_event_loop(&self, cb: Box<dyn AudioBridgeHandleCallback>) {
        let Ok(Some(mut receiver)) = self.receiver.lock().map(|mut x| x.take()) else {
            return;
        };

        let join_handle = tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                match event {
                    _ => {}
                }
            }
        });

        if let Ok(mut abort_handle) = self.abort_handle.lock() {
            *abort_handle = Some(join_handle.abort_handle());
        }
    }
}

base_handle!(AudioBridgeHandle);

#[uniffi::export(callback_interface)]
pub trait AudioBridgeHandleCallback: Send + Sync + Debug {
    fn on_handle_event(&self, event: GenericEvent);
}

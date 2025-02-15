use crate::base_handle;
use crate::error::JanusGatewayCommunicationError;
use crate::protocol::Candidate;
use crate::protocol::GenericEvent;
use crate::protocol::Jsep;
use jarust::core::jahandle::JaHandle;
use jarust::interface::japrotocol::JaHandleEvent;
use jarust::interface::japrotocol::JaResponse;
use jarust::interface::japrotocol::ResponseType;
use serde_json::Value;
use std::fmt::Debug;
use std::sync::Mutex;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task::AbortHandle;

#[derive(uniffi::Object)]
pub struct Handle {
    inner: JaHandle,
    receiver: Mutex<Option<mpsc::UnboundedReceiver<JaResponse>>>,
    abort_handle: Mutex<Option<AbortHandle>>,
}

impl Handle {
    pub fn new(handle: JaHandle, receiver: mpsc::UnboundedReceiver<JaResponse>) -> Self {
        Self {
            inner: handle,
            receiver: Mutex::new(Some(receiver)),
            abort_handle: Mutex::new(None),
        }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl Handle {
    pub async fn start_event_loop(&self, cb: Box<dyn HandleCallback>) {
        let Ok(Some(mut receiver)) = self.receiver.lock().map(|mut x| x.take()) else {
            return;
        };

        let join_handle = tokio::spawn(async move {
            while let Some(item) = receiver.recv().await {
                if let ResponseType::Event(event) = item.janus {
                    match event {
                        JaHandleEvent::PluginEvent { plugin_data } => {
                            if let Ok(plugin_data) = serde_json::to_vec(&plugin_data) {
                                cb.on_plugin_event(plugin_data);
                            }
                        }
                        JaHandleEvent::GenericEvent(generic_event) => {
                            cb.on_handle_event(generic_event);
                        }
                    }
                };
            }
        });

        if let Ok(mut abort_handle) = self.abort_handle.lock() {
            *abort_handle = Some(join_handle.abort_handle());
        }
    }
}

base_handle!(Handle);

#[uniffi::export(callback_interface)]
pub trait HandleCallback: Send + Sync + Debug {
    fn on_plugin_event(&self, event: Vec<u8>);
    fn on_handle_event(&self, event: GenericEvent);
}

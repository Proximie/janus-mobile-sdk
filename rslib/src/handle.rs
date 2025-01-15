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
    pub async fn fire_and_forget(
        &self,
        data: Vec<u8>,
    ) -> Result<(), JanusGatewayCommunicationError> {
        let Ok(body) = serde_json::from_slice(&data) else {
            return Err(JanusGatewayCommunicationError::Serialize {
                body: String::from_utf8_lossy(&data).to_string(),
            });
        };
        if let Err(why) = self.inner.fire_and_forget(body).await {
            return Err(JanusGatewayCommunicationError::SendFailure {
                reason: why.to_string(),
            });
        };
        Ok(())
    }

    pub async fn fire_and_forget_with_jsep(
        &self,
        data: Vec<u8>,
        jsep: Jsep,
    ) -> Result<(), JanusGatewayCommunicationError> {
        let Ok(body) = serde_json::from_slice(&data) else {
            return Err(JanusGatewayCommunicationError::Serialize {
                body: String::from_utf8_lossy(&data).to_string(),
            });
        };
        if let Err(why) = self
            .inner
            .fire_and_forget_with_jsep(body, jsep.into())
            .await
        {
            return Err(JanusGatewayCommunicationError::SendFailure {
                reason: why.to_string(),
            });
        };
        Ok(())
    }

    pub async fn send_waiton_ack(
        &self,
        data: Vec<u8>,
        timeout: Duration,
    ) -> Result<(), JanusGatewayCommunicationError> {
        let Ok(body) = serde_json::from_slice(&data) else {
            return Err(JanusGatewayCommunicationError::Serialize {
                body: String::from_utf8_lossy(&data).to_string(),
            });
        };
        if let Err(why) = self.inner.send_waiton_ack(body, timeout).await {
            return Err(JanusGatewayCommunicationError::SendFailure {
                reason: why.to_string(),
            });
        };
        Ok(())
    }

    pub async fn send_waiton_result(
        &self,
        data: Vec<u8>,
        timeout: Duration,
    ) -> Result<Vec<u8>, JanusGatewayCommunicationError> {
        let Ok(body) = serde_json::from_slice(&data) else {
            return Err(JanusGatewayCommunicationError::Serialize {
                body: String::from_utf8_lossy(&data).to_string(),
            });
        };
        let result = match self.inner.send_waiton_rsp::<Value>(body, timeout).await {
            Ok(result) => result,
            Err(why) => {
                return Err(JanusGatewayCommunicationError::SendFailure {
                    reason: why.to_string(),
                })
            }
        };
        let Ok(result) = serde_json::from_value(result) else {
            return Err(JanusGatewayCommunicationError::Serialize {
                body: String::from_utf8_lossy(&data).to_string(),
            });
        };
        Ok(result)
    }

    pub async fn start_event_loop(&self, cb: Box<dyn HandleCallback>) {
        let Ok(Some(mut receiver)) = self.receiver.lock().map(|mut x| x.take()) else {
            return;
        };

        let join_handle = tokio::spawn(async move {
            while let Some(item) = receiver.recv().await {
                if let ResponseType::Event(event) = item.janus { match event {
                    JaHandleEvent::PluginEvent { plugin_data } => {
                        if let Ok(plugin_data) = serde_json::to_string(&plugin_data) {
                            cb.on_plugin_event(plugin_data);
                        }
                    }
                    JaHandleEvent::GenericEvent(generic_event) => {
                        cb.on_handle_event(generic_event.into());
                    }
                } };
            }
        });

        if let Ok(mut abort_handle) = self.abort_handle.lock() {
            *abort_handle = Some(join_handle.abort_handle());
        }
    }

    pub async fn hangup(&self, timeout: Duration) -> Result<(), JanusGatewayCommunicationError> {
        self.inner.hangup(timeout).await.map_err(|err| {
            JanusGatewayCommunicationError::SendFailure {
                reason: err.to_string(),
            }
        })
    }

    pub async fn detach(&self, timeout: Duration) -> Result<(), JanusGatewayCommunicationError> {
        self.inner.detach(timeout).await.map_err(|err| {
            JanusGatewayCommunicationError::SendFailure {
                reason: err.to_string(),
            }
        })
    }

    pub async fn trickle_single_candidate(
        &self,
        candidate: Candidate,
        timeout: Duration,
    ) -> Result<(), JanusGatewayCommunicationError> {
        self.inner
            .trickle_single_candidate(candidate.into(), timeout)
            .await
            .map_err(|err| JanusGatewayCommunicationError::SendFailure {
                reason: err.to_string(),
            })
    }

    pub async fn trickle_candidates(
        &self,
        candidates: Vec<Candidate>,
        timeout: Duration,
    ) -> Result<(), JanusGatewayCommunicationError> {
        self.inner
            .trickle_candidates(candidates.into_iter().map(Into::into).collect(), timeout)
            .await
            .map_err(|err| JanusGatewayCommunicationError::SendFailure {
                reason: err.to_string(),
            })
    }

    pub async fn complete_trickle(
        &self,
        timeout: Duration,
    ) -> Result<(), JanusGatewayCommunicationError> {
        self.inner.complete_trickle(timeout).await.map_err(|err| {
            JanusGatewayCommunicationError::SendFailure {
                reason: err.to_string(),
            }
        })
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        if let Ok(Some(abort_handle)) = self.abort_handle.lock().map(|mut x| x.take()) {
            abort_handle.abort();
        }
    }
}

#[uniffi::export(callback_interface)]
pub trait HandleCallback: Send + Sync + Debug {
    fn on_plugin_event(&self, event: String);
    fn on_handle_event(&self, event: GenericEvent);
}

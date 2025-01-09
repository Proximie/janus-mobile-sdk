use crate::error::JanusGatewayError;
use crate::japrotocol::Jsep;
use jarust::core::jahandle::JaHandle;
use jarust::interface::japrotocol::JaResponse;
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
    pub async fn fire_and_forget(&self, msg: &str) -> crate::JanusGatewayResult<()> {
        let Ok(body) = serde_json::from_str(msg) else {
            return Err(JanusGatewayError::Serialize {
                body: msg.to_string(),
            });
        };
        if let Err(why) = self.inner.fire_and_forget(body).await {
            return Err(JanusGatewayError::SendFailure {
                reason: why.to_string(),
            });
        };
        Ok(())
    }

    pub async fn fire_and_forget_with_jsep(
        &self,
        msg: &str,
        jsep: Jsep,
    ) -> crate::JanusGatewayResult<()> {
        let Ok(body) = serde_json::from_str(msg) else {
            return Err(JanusGatewayError::Serialize {
                body: msg.to_string(),
            });
        };
        if let Err(why) = self
            .inner
            .fire_and_forget_with_jsep(body, jsep.into())
            .await
        {
            return Err(JanusGatewayError::SendFailure {
                reason: why.to_string(),
            });
        };
        Ok(())
    }

    pub async fn send_waiton_ack(
        &self,
        msg: &str,
        timeout: Duration,
    ) -> crate::JanusGatewayResult<()> {
        let Ok(body) = serde_json::from_str(msg) else {
            return Err(JanusGatewayError::Serialize {
                body: msg.to_string(),
            });
        };
        if let Err(why) = self.inner.send_waiton_ack(body, timeout).await {
            return Err(JanusGatewayError::SendFailure {
                reason: why.to_string(),
            });
        };
        Ok(())
    }

    pub async fn send_waiton_result(
        &self,
        msg: &str,
        timeout: Duration,
    ) -> crate::JanusGatewayResult<String> {
        let Ok(body) = serde_json::from_str(msg) else {
            return Err(JanusGatewayError::Serialize {
                body: msg.to_string(),
            });
        };
        let result = match self.inner.send_waiton_rsp::<Value>(body, timeout).await {
            Ok(result) => result,
            Err(why) => {
                return Err(JanusGatewayError::SendFailure {
                    reason: why.to_string(),
                })
            }
        };
        let Ok(result) = serde_json::from_value(result) else {
            return Err(JanusGatewayError::Serialize {
                body: msg.to_string(),
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
                if let Ok(item) = serde_json::to_string(&item) {
                    cb.on_event(item);
                }
            }
        });

        if let Ok(mut abort_handle) = self.abort_handle.lock() {
            *abort_handle = Some(join_handle.abort_handle());
        }
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
    fn on_event(&self, event: String);
}

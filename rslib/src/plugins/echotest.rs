use crate::base_handle;
use crate::error::JanusGatewayCommunicationError;
use crate::protocol::Candidate;
use crate::protocol::GenericEvent;
use crate::protocol::Jsep;
use jarust::plugins::echo_test::events::EchoTestEvent;
use jarust::plugins::echo_test::events::PluginEvent;
use jarust::plugins::echo_test::handle::EchoTestHandle as JaEchoTestHandle;
use jarust::plugins::echo_test::params::EchoTestStartParams;
use serde_json::Value;
use std::fmt::Debug;
use std::sync::Mutex;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task::AbortHandle;

#[derive(uniffi::Object)]
pub struct EchotestHandle {
    inner: JaEchoTestHandle,
    receiver: Mutex<Option<mpsc::UnboundedReceiver<PluginEvent>>>,
    abort_handle: Mutex<Option<AbortHandle>>,
}

impl EchotestHandle {
    pub fn new(handle: JaEchoTestHandle, receiver: mpsc::UnboundedReceiver<PluginEvent>) -> Self {
        Self {
            inner: handle,
            receiver: Mutex::new(Some(receiver)),
            abort_handle: Mutex::new(None),
        }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl EchotestHandle {
    pub async fn start(
        &self,
        audio: Option<bool>,
        video: Option<bool>,
        bitrate: Option<u32>,
    ) -> Result<(), JanusGatewayCommunicationError> {
        if let Err(why) = self
            .inner
            .start(EchoTestStartParams {
                audio,
                video,
                bitrate,
                ..Default::default()
            })
            .await
        {
            return Err(JanusGatewayCommunicationError::SendFailure {
                reason: why.to_string(),
            });
        };
        Ok(())
    }

    pub async fn start_with_jsep(
        &self,
        audio: Option<bool>,
        video: Option<bool>,
        bitrate: Option<u32>,
        jsep: Jsep,
        timeout: Duration,
    ) -> Result<(), JanusGatewayCommunicationError> {
        if let Err(why) = self
            .inner
            .start_with_jsep(
                EchoTestStartParams {
                    audio,
                    video,
                    bitrate,
                    ..Default::default()
                },
                jsep.into(),
                timeout,
            )
            .await
        {
            return Err(JanusGatewayCommunicationError::SendFailure {
                reason: why.to_string(),
            });
        };
        Ok(())
    }

    pub async fn start_event_loop(&self, cb: Box<dyn EchotestHandleCallback>) {
        let Ok(Some(mut receiver)) = self.receiver.lock().map(|mut x| x.take()) else {
            return;
        };

        let join_handle = tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                match event {
                    PluginEvent::EchoTestEvent(EchoTestEvent::Result { echotest, result }) => {
                        cb.on_result(echotest, result);
                    }
                    PluginEvent::EchoTestEvent(EchoTestEvent::ResultWithJsep {
                        echotest,
                        result,
                        jsep,
                    }) => cb.on_result_with_jsep(echotest, result, jsep.into()),
                    PluginEvent::EchoTestEvent(EchoTestEvent::Error { error_code, error }) => {
                        cb.on_echo_test_error(error_code, error)
                    }
                    PluginEvent::EchoTestEvent(EchoTestEvent::Other(_))
                    | PluginEvent::GenericEvent(_) => {}
                }
            }
        });

        if let Ok(mut abort_handle) = self.abort_handle.lock() {
            *abort_handle = Some(join_handle.abort_handle());
        }
    }
}

base_handle!(EchotestHandle);

#[uniffi::export(callback_interface)]
pub trait EchotestHandleCallback: Send + Sync + Debug {
    fn on_result(&self, echotest: String, result: String);
    fn on_result_with_jsep(&self, echotest: String, result: String, jsep: Jsep);
    fn on_echo_test_error(&self, error_code: u16, error: String);
    fn on_handle_event(&self, event: GenericEvent);
}

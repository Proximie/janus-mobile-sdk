use crate::base_handle_with_drop_impl;
use crate::error::JanusGatewayCommunicationError;
use crate::protocol::Candidate;
use crate::protocol::GenericEvent;
use crate::protocol::Jsep;
use jarust::plugins::JanusId;
use jarust::plugins::legacy_video_room::events::LegacyVideoRoomEvent;
use jarust::plugins::legacy_video_room::events::PluginEvent;
use jarust::plugins::legacy_video_room::handle::LegacyVideoRoomHandle as JaLegacyVideoRoomHandle;
use jarust::plugins::legacy_video_room::params::LegacyVideoRoomCreateParams;
use jarust::plugins::legacy_video_room::params::LegacyVideoRoomExistsParams;
use jarust::plugins::legacy_video_room::params::LegacyVideoRoomKickParams;
use jarust::plugins::legacy_video_room::params::LegacyVideoRoomPublisherConfigureParams;
use jarust::plugins::legacy_video_room::params::LegacyVideoRoomPublisherJoinAndConfigureParams;
use jarust::plugins::legacy_video_room::params::LegacyVideoRoomPublisherJoinParams;
use jarust::plugins::legacy_video_room::responses::LegacyVideoRoomCreatedRsp;
use serde_json::Value;
use std::fmt::Debug;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task::AbortHandle;

#[derive(uniffi::Object)]
pub struct LegacyVideoRoomHandle {
    inner: JaLegacyVideoRoomHandle,
    receiver: Mutex<Option<mpsc::UnboundedReceiver<PluginEvent>>>,
    abort_handle: Mutex<Option<AbortHandle>>,
    is_event_loop_running: AtomicBool,
}

impl LegacyVideoRoomHandle {
    pub fn new(
        handle: JaLegacyVideoRoomHandle,
        receiver: mpsc::UnboundedReceiver<PluginEvent>,
    ) -> Self {
        Self {
            inner: handle,
            receiver: Mutex::new(Some(receiver)),
            abort_handle: Mutex::new(None),
            is_event_loop_running: AtomicBool::new(false),
        }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl LegacyVideoRoomHandle {
    pub async fn start_event_loop(&self, cb: Box<dyn LegacyVideoRoomHandleCallback>) {
        if self.is_event_loop_running.load(Ordering::Relaxed) {
            return;
        }

        let Ok(Some(mut receiver)) = self.receiver.lock().map(|mut x| x.take()) else {
            return;
        };

        let join_handle = tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                match event {
                    PluginEvent::GenericEvent(generic_event) => {
                        cb.on_legacy_video_room_handle_event(generic_event);
                    }
                    PluginEvent::LegacyVideoRoomEvent(LegacyVideoRoomEvent::Error {
                        error_code,
                        error,
                    }) => {
                        cb.on_legacy_video_room_error(error_code, error);
                    }
                    PluginEvent::LegacyVideoRoomEvent(LegacyVideoRoomEvent::Other(data)) => {
                        if let Ok(data) = serde_json::to_vec(&data) {
                            cb.on_legacy_video_room_other(data);
                        }
                    }
                    _ => {}
                }
            }
        });

        if let Ok(mut abort_handle) = self.abort_handle.lock() {
            *abort_handle = Some(join_handle.abort_handle());
        }

        self.is_event_loop_running.store(true, Ordering::Relaxed);
    }

    pub async fn create_room(
        &self,
        params: LegacyVideoRoomCreateParams,
        timeout: Duration,
    ) -> Result<LegacyVideoRoomCreatedRsp, JanusGatewayCommunicationError> {
        match self.inner.create_room(params, timeout).await {
            Ok(rsp) => Ok(rsp),
            Err(why) => Err(JanusGatewayCommunicationError::SendFailure {
                reason: why.to_string(),
            }),
        }
    }

    pub async fn exist(
        &self,
        room: JanusId,
        timeout: Duration,
    ) -> Result<bool, JanusGatewayCommunicationError> {
        match self
            .inner
            .exists(LegacyVideoRoomExistsParams { room }, timeout)
            .await
        {
            Ok(rsp) => Ok(rsp),
            Err(why) => Err(JanusGatewayCommunicationError::SendFailure {
                reason: why.to_string(),
            }),
        }
    }

    pub async fn kick(
        &self,
        params: LegacyVideoRoomKickParams,
        timeout: Duration,
    ) -> Result<(), JanusGatewayCommunicationError> {
        match self.inner.kick(params, timeout).await {
            Ok(_) => Ok(()),
            Err(why) => Err(JanusGatewayCommunicationError::SendFailure {
                reason: why.to_string(),
            }),
        }
    }

    pub async fn publisher_join(
        &self,
        params: LegacyVideoRoomPublisherJoinParams,
        jsep: Option<Jsep>,
        timeout: Duration,
    ) -> Result<String, JanusGatewayCommunicationError> {
        match self.inner.publisher_join(params, jsep, timeout).await {
            Ok(transaction) => Ok(transaction),
            Err(why) => Err(JanusGatewayCommunicationError::SendFailure {
                reason: why.to_string(),
            }),
        }
    }

    pub async fn publisher_configure(
        &self,
        params: LegacyVideoRoomPublisherConfigureParams,
        timeout: Duration,
    ) -> Result<String, JanusGatewayCommunicationError> {
        match self.inner.publisher_configure(params, timeout).await {
            Ok(transaction) => Ok(transaction),
            Err(why) => Err(JanusGatewayCommunicationError::SendFailure {
                reason: why.to_string(),
            }),
        }
    }

    pub async fn publisher_join_and_configure(
        &self,
        params: LegacyVideoRoomPublisherJoinAndConfigureParams,
        jsep: Option<Jsep>,
        timeout: Duration,
    ) -> Result<String, JanusGatewayCommunicationError> {
        match self
            .inner
            .publisher_join_and_configure(params, jsep, timeout)
            .await
        {
            Ok(transaction) => Ok(transaction),
            Err(why) => Err(JanusGatewayCommunicationError::SendFailure {
                reason: why.to_string(),
            }),
        }
    }
}

base_handle_with_drop_impl!(LegacyVideoRoomHandle);

#[uniffi::export(callback_interface)]
pub trait LegacyVideoRoomHandleCallback: Send + Sync + Debug {
    fn on_legacy_video_room_other(&self, data: Vec<u8>);
    fn on_legacy_video_room_handle_event(&self, event: GenericEvent);
    fn on_legacy_video_room_error(&self, error_code: u16, error: String);
}

use crate::base_handle_with_drop_impl;
use crate::error::JanusGatewayCommunicationError;
use crate::plugins::common::JanusId;
use crate::protocol::Candidate;
use crate::protocol::GenericEvent;
use crate::protocol::Jsep;
use jarust::plugins::video_room::events::PluginEvent;
use jarust::plugins::video_room::events::VideoRoomEvent;
use jarust::plugins::video_room::handle::VideoRoomHandle as JaVideoRoomHandle;
use jarust::plugins::video_room::params::VideoRoomCreateParams;
use jarust::plugins::video_room::params::VideoRoomExistsParams;
use jarust::plugins::video_room::params::VideoRoomPublisherJoinAndConfigureParams;
use jarust::plugins::video_room::responses::ConfiguredStream;
use jarust::plugins::video_room::responses::VideoRoomCreatedRsp;
use serde_json::Value;
use std::fmt::Debug;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task::AbortHandle;

#[derive(uniffi::Object)]
pub struct VideoRoomHandle {
    inner: JaVideoRoomHandle,
    receiver: Mutex<Option<mpsc::UnboundedReceiver<PluginEvent>>>,
    abort_handle: Mutex<Option<AbortHandle>>,
    is_event_loop_running: AtomicBool,
}

impl VideoRoomHandle {
    pub fn new(handle: JaVideoRoomHandle, receiver: mpsc::UnboundedReceiver<PluginEvent>) -> Self {
        Self {
            inner: handle,
            receiver: Mutex::new(Some(receiver)),
            abort_handle: Mutex::new(None),
            is_event_loop_running: AtomicBool::new(false),
        }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl VideoRoomHandle {
    pub async fn start_event_loop(&self, cb: Box<dyn VideoRoomHandleCallback>) {
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
                        cb.on_handle_event(generic_event);
                    }
                    PluginEvent::VideoRoomEvent(VideoRoomEvent::Error { error_code, error }) => {
                        cb.on_video_room_error(error_code, error);
                    }
                    PluginEvent::VideoRoomEvent(VideoRoomEvent::ConfiguredWithJsep {
                        room,
                        audio_codec,
                        video_codec,
                        streams,
                        jsep,
                    }) => {
                        cb.on_configure_with_jsep(room, audio_codec, video_codec, streams, jsep);
                    }
                    PluginEvent::VideoRoomEvent(VideoRoomEvent::Kicked { room, participant }) => {
                        cb.on_kicked(room, participant);
                    }
                    PluginEvent::VideoRoomEvent(VideoRoomEvent::Leaving { room, reason }) => {
                        cb.on_leaving(room, reason);
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
        params: VideoRoomCreateParams,
        timeout: Duration,
    ) -> Result<VideoRoomCreatedRsp, JanusGatewayCommunicationError> {
        match self.inner.create_room_with_config(params, timeout).await {
            Ok(rsp) => Ok(rsp),
            Err(why) => Err(JanusGatewayCommunicationError::SendFailure {
                reason: why.to_string(),
            }),
        }
    }

    pub async fn exist(
        &self,
        room_id: JanusId,
        timeout: Duration,
    ) -> Result<bool, JanusGatewayCommunicationError> {
        match self
            .inner
            .exists(VideoRoomExistsParams { room: room_id }, timeout)
            .await
        {
            Ok(rsp) => Ok(rsp),
            Err(why) => Err(JanusGatewayCommunicationError::SendFailure {
                reason: why.to_string(),
            }),
        }
    }

    pub async fn publisher_join_and_configure(
        &self,
        params: VideoRoomPublisherJoinAndConfigureParams,
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

base_handle_with_drop_impl!(VideoRoomHandle);

#[uniffi::export(callback_interface)]
pub trait VideoRoomHandleCallback: Send + Sync + Debug {
    fn on_handle_event(&self, event: GenericEvent);
    fn on_video_room_error(&self, error_code: u16, error: String);
    fn on_other(&self, data: Vec<u8>);
    fn on_configure_with_jsep(
        &self,
        room: JanusId,
        audio_codec: Option<String>,
        video_codec: Option<String>,
        streams: Option<Vec<ConfiguredStream>>,
        jsep: Jsep,
    );
    fn on_kicked(&self, room: JanusId, participant: JanusId);
    fn on_leaving(&self, room: JanusId, reason: String);
}

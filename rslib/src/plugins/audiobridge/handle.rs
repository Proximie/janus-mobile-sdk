use super::responses::AudioBridgeListParticipantsRsp;
use super::responses::AudioBridgeParticipant;
use super::responses::AudioBridgeRoomCreatedRsp;
use crate::base_handle;
use crate::error::JanusGatewayCommunicationError;
use crate::plugins::common::JanusId;
use crate::protocol::Candidate;
use crate::protocol::GenericEvent;
use crate::protocol::Jsep;
use jarust::plugins::audio_bridge::events::AudioBridgeEvent;
use jarust::plugins::audio_bridge::events::PluginEvent;
use jarust::plugins::audio_bridge::handle::AudioBridgeHandle as JaAudioBridgeHandle;
use jarust::plugins::audio_bridge::params::AudioBridgeConfigureParams;
use jarust::plugins::audio_bridge::params::AudioBridgeCreateParams;
use jarust::plugins::audio_bridge::params::AudioBridgeExistsParams;
use jarust::plugins::audio_bridge::params::AudioBridgeJoinParams;
use jarust::plugins::audio_bridge::params::AudioBridgeListParticipantsParams;
use jarust::plugins::audio_bridge::params::AudioBridgeMuteParams;
use serde_json::Value;
use std::fmt::Debug;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Mutex;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task::AbortHandle;

#[derive(uniffi::Object)]
pub struct AudioBridgeHandle {
    inner: JaAudioBridgeHandle,
    receiver: Mutex<Option<mpsc::UnboundedReceiver<PluginEvent>>>,
    abort_handle: Mutex<Option<AbortHandle>>,
    is_event_loop_running: AtomicBool,
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
            is_event_loop_running: AtomicBool::new(false),
        }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl AudioBridgeHandle {
    pub async fn start_event_loop(&self, cb: Box<dyn AudioBridgeHandleCallback>) {
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
                    PluginEvent::AudioBridgeEvent(AudioBridgeEvent::Result {
                        transaction,
                        result,
                    }) => cb.on_result(transaction, result),
                    PluginEvent::AudioBridgeEvent(AudioBridgeEvent::ResultWithJsep {
                        transaction,
                        result,
                        jsep,
                    }) => cb.on_result_with_jsep(transaction, result, jsep),
                    PluginEvent::AudioBridgeEvent(AudioBridgeEvent::Error {
                        error_code,
                        error,
                    }) => cb.on_audio_bridge_error(error_code, error),
                    PluginEvent::AudioBridgeEvent(AudioBridgeEvent::Other(data)) => {
                        if let Ok(data) = serde_json::to_vec(&data) {
                            cb.on_other(data);
                        }
                    }
                    PluginEvent::AudioBridgeEvent(AudioBridgeEvent::RoomJoined {
                        id,
                        room,
                        participants,
                    }) => cb.on_room_joined(id, room, participants),
                    PluginEvent::AudioBridgeEvent(AudioBridgeEvent::RoomJoinedWithJsep {
                        id,
                        room,
                        participants,
                        jsep,
                    }) => cb.on_room_joined_with_jsep(id, room, participants, jsep),
                    PluginEvent::AudioBridgeEvent(AudioBridgeEvent::ParticipantsJoined {
                        room,
                        participants,
                    }) => cb.on_participants_joined(room, participants),
                    PluginEvent::AudioBridgeEvent(AudioBridgeEvent::ParticipantsUpdated {
                        room,
                        participants,
                    }) => cb.on_participants_updated(room, participants),
                    PluginEvent::AudioBridgeEvent(AudioBridgeEvent::ParticipantLeft {
                        room,
                        leaving,
                    }) => cb.on_participant_left(room, leaving),
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
        params: AudioBridgeCreateParams,
        timeout: Duration,
    ) -> Result<AudioBridgeRoomCreatedRsp, JanusGatewayCommunicationError> {
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
            .exists(AudioBridgeExistsParams { room: room_id }, timeout)
            .await
        {
            Ok(rsp) => Ok(rsp),
            Err(why) => Err(JanusGatewayCommunicationError::SendFailure {
                reason: why.to_string(),
            }),
        }
    }

    pub async fn list_participants(
        &self,
        room_id: JanusId,
        timeout: Duration,
    ) -> Result<AudioBridgeListParticipantsRsp, JanusGatewayCommunicationError> {
        match self
            .inner
            .list_participants(AudioBridgeListParticipantsParams { room: room_id }, timeout)
            .await
        {
            Ok(rsp) => Ok(rsp),
            Err(why) => Err(JanusGatewayCommunicationError::SendFailure {
                reason: why.to_string(),
            }),
        }
    }

    pub async fn join_room(
        &self,
        params: AudioBridgeJoinParams,
        jsep: Option<Jsep>,
        timeout: Duration,
    ) -> Result<String, JanusGatewayCommunicationError> {
        match self.inner.join_room(params, jsep, timeout).await {
            Ok(rsp) => Ok(rsp),
            Err(why) => Err(JanusGatewayCommunicationError::SendFailure {
                reason: why.to_string(),
            }),
        }
    }

    pub async fn mute(
        &self,
        params: AudioBridgeMuteParams,
    ) -> Result<String, JanusGatewayCommunicationError> {
        match self.inner.mute(params).await {
            Ok(rsp) => Ok(rsp),
            Err(why) => Err(JanusGatewayCommunicationError::SendFailure {
                reason: why.to_string(),
            }),
        }
    }

    pub async fn unmute(
        &self,
        params: AudioBridgeMuteParams,
    ) -> Result<String, JanusGatewayCommunicationError> {
        match self.inner.unmute(params).await {
            Ok(rsp) => Ok(rsp),
            Err(why) => Err(JanusGatewayCommunicationError::SendFailure {
                reason: why.to_string(),
            }),
        }
    }

    pub async fn configure(
        &self,
        params: AudioBridgeConfigureParams,
        jsep: Option<Jsep>,
        timeout: Duration,
    ) -> Result<String, JanusGatewayCommunicationError> {
        match self.inner.configure(params, jsep, timeout).await {
            Ok(rsp) => Ok(rsp),
            Err(why) => Err(JanusGatewayCommunicationError::SendFailure {
                reason: why.to_string(),
            }),
        }
    }
}

base_handle!(AudioBridgeHandle);

#[uniffi::export(callback_interface)]
pub trait AudioBridgeHandleCallback: Send + Sync + Debug {
    fn on_result(&self, transaction: String, result: String);
    fn on_result_with_jsep(&self, transaction: String, result: String, jsep: Jsep);
    fn on_room_joined_with_jsep(
        &self,
        id: JanusId,
        room: JanusId,
        participants: Vec<AudioBridgeParticipant>,
        jsep: Jsep,
    );
    fn on_room_joined(&self, id: JanusId, room: JanusId, participants: Vec<AudioBridgeParticipant>);
    fn on_participants_joined(&self, room: JanusId, participants: Vec<AudioBridgeParticipant>);
    fn on_participants_updated(&self, room: JanusId, participants: Vec<AudioBridgeParticipant>);
    fn on_participant_left(&self, room: JanusId, participant_id: JanusId);
    fn on_handle_event(&self, event: GenericEvent);
    fn on_audio_bridge_error(&self, error_code: u16, error: String);
    fn on_other(&self, data: Vec<u8>);
}

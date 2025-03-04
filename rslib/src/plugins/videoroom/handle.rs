use crate::base_handle_with_drop_impl;
use crate::error::JanusGatewayCommunicationError;
use crate::plugins::common::JanusId;
use crate::protocol::Candidate;
use crate::protocol::GenericEvent;
use crate::protocol::Jsep;
use jarust::plugins::audio_bridge::events::AudioBridgeEvent;
use jarust::plugins::audio_bridge::events::PluginEvent;
use jarust::plugins::video_room::handle::VideoRoomHandle as JaVideoRoomHandle;
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

base_handle_with_drop_impl!(VideoRoomHandle);

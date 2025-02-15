use crate::plugins::common::JanusId;
use jarust::plugins::audio_bridge::responses;

pub type AudioBridgeRoomCreatedRsp = responses::AudioBridgeRoomCreatedRsp;

#[uniffi::remote(Record)]
pub struct AudioBridgeRoomCreatedRsp {
    room: JanusId,
    permanent: bool,
}

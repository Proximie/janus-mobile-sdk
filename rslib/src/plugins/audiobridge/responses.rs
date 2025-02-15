use crate::plugins::common::JanusId;
use jarust::plugins::audio_bridge::common;
use jarust::plugins::audio_bridge::responses;

pub type AudioBridgeRoomCreatedRsp = responses::AudioBridgeRoomCreatedRsp;
pub type AudioBridgeListParticipantsRsp = responses::AudioBridgeListParticipantsRsp;
pub type AudioBridgeParticipant = common::AudioBridgeParticipant;

#[uniffi::remote(Record)]
pub struct AudioBridgeRoomCreatedRsp {
    room: JanusId,
    permanent: bool,
}

#[uniffi::remote(Record)]
pub struct AudioBridgeParticipant {
    pub id: JanusId,
    pub display: Option<String>,
    pub setup: bool,
    pub muted: bool,
    pub suspended: Option<bool>,
    pub talking: Option<bool>,
    pub spatial_position: Option<u64>,
}

#[uniffi::remote(Record)]
pub struct AudioBridgeListParticipantsRsp {
    pub room: JanusId,
    pub participants: Vec<AudioBridgeParticipant>,
}

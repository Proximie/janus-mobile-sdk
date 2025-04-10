use crate::plugins::common::JanusId;
use jarust::plugins::legacy_video_room::responses;

pub type LegacyVideoRoomCreatedRsp = responses::LegacyVideoRoomCreatedRsp;
pub type LegacyVideoRoomPublisher = responses::LegacyVideoRoomPublisher;

#[uniffi::remote(Record)]
pub struct LegacyVideoRoomCreatedRsp {
    pub room: JanusId,
    pub permanent: bool,
}

#[uniffi::remote(Record)]
pub struct LegacyVideoRoomPublisher {
    pub id: JanusId,
    pub display: Option<String>,
    pub substream: Option<u8>,
}

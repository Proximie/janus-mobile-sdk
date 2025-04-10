use crate::plugins::common::JanusId;
use jarust::plugins::legacy_video_room::responses;

pub type LegacyVideoRoomCreatedRsp = responses::LegacyVideoRoomCreatedRsp;

#[uniffi::remote(Record)]
pub struct LegacyVideoRoomCreatedRsp {
    pub room: JanusId,
    pub permanent: bool,
}

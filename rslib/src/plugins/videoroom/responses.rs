use crate::plugins::common::JanusId;
use jarust::plugins::video_room::responses;

pub type VideoRoomCreatedRsp = responses::VideoRoomCreatedRsp;

#[uniffi::remote(Record)]
pub struct VideoRoomCreatedRsp {
    pub room: JanusId,
    pub permanent: bool,
}

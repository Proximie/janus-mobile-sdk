use crate::plugins::common::JanusId;
use jarust::plugins::video_room::responses;

pub type VideoRoomCreatedRsp = responses::VideoRoomCreatedRsp;
pub type ConfiguredStream = responses::ConfiguredStream;

#[uniffi::remote(Record)]
pub struct VideoRoomCreatedRsp {
    pub room: JanusId,
    pub permanent: bool,
}

#[uniffi::remote(Record)]
pub struct ConfiguredStream {
    pub media_type: String,
    pub mindex: u64,
    pub mid: String,
    pub disabled: bool,
    pub codec: String,
    pub stereo: bool,
    pub fec: bool,
    pub dtx: bool,
    pub h264_profile: Option<String>,
    pub vp9_profile: Option<String>,
    pub moderated: bool,
    pub simulcast: bool,
    pub svc: bool,
}

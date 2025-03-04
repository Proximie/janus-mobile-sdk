use crate::plugins::common::JanusId;
use jarust::plugins::video_room::params;

pub type VideoRoomAudioCodec = params::VideoRoomAudioCodec;
pub type VideoRoomAudioCodecList = params::VideoRoomAudioCodecList;
pub type VideoRoomCreateParams = params::VideoRoomCreateParams;
pub type VideoRoomVideoCodec = params::VideoRoomVideoCodec;
pub type VideoRoomVideoCodecList = params::VideoRoomVideoCodecList;

#[uniffi::remote(Record)]
pub struct VideoRoomCreateParams {
    admin_key: Option<String>,
    room: Option<JanusId>,
    description: Option<String>,
    is_private: Option<bool>,
    allowed: Option<Vec<String>>,
    secret: Option<String>,
    pin: Option<String>,
    require_pvtid: Option<bool>,
    signed_tokens: Option<bool>,
    bitrate: Option<u64>,
    bitrate_cap: Option<bool>,
    fir_freq: Option<u64>,
    publishers: Option<u64>,
    audiocodec: Option<VideoRoomAudioCodecList>,
    videocodec: Option<VideoRoomVideoCodecList>,
    vp9_profile: Option<String>,
    h264_profile: Option<String>,
    opus_fec: Option<bool>,
    opus_dtx: Option<bool>,
    audiolevel_ext: Option<bool>,
    audiolevel_event: Option<bool>,
    audio_active_packets: Option<u64>,
    audio_level_average: Option<u64>,
    videoorient_ext: Option<bool>,
    playoutdelay_ext: Option<bool>,
    transport_wide_cc_ext: Option<bool>,
    record: Option<bool>,
    record_dir: Option<String>,
    lock_record: Option<bool>,
    permanent: Option<bool>,
    notify_joining: Option<bool>,
    require_e2ee: Option<bool>,
    dummy_publisher: Option<bool>,
    dummy_streams: Option<bool>,
}

#[uniffi::remote(Enum)]
pub enum VideoRoomAudioCodec {
    OPUS,
    G722,
    PCMU,
    PCMA,
    ISAC32,
    ISAC16,
}

#[uniffi::remote(Record)]
pub struct VideoRoomAudioCodecList {
    codecs: Vec<VideoRoomAudioCodec>,
}

#[uniffi::remote(Enum)]
pub enum VideoRoomVideoCodec {
    VP8,
    VP9,
    H264,
    AV1,
    H265,
}

#[uniffi::remote(Record)]
pub struct VideoRoomVideoCodecList {
    pub codecs: Vec<VideoRoomVideoCodec>,
}

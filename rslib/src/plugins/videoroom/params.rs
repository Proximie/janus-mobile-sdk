use crate::plugins::common::JanusId;
use jarust::plugins::video_room::params;

pub type VideoRoomAudioCodec = params::VideoRoomAudioCodec;
pub type VideoRoomAudioCodecList = params::VideoRoomAudioCodecList;
pub type VideoRoomCreateParams = params::VideoRoomCreateParams;
pub type VideoRoomVideoCodec = params::VideoRoomVideoCodec;
pub type VideoRoomVideoCodecList = params::VideoRoomVideoCodecList;

#[uniffi::remote(Record)]
pub struct VideoRoomCreateParams {
    #[uniffi(default = None)]
    admin_key: Option<String>,
    #[uniffi(default = None)]
    room: Option<JanusId>,
    #[uniffi(default = None)]
    description: Option<String>,
    #[uniffi(default = None)]
    is_private: Option<bool>,
    #[uniffi(default = None)]
    allowed: Option<Vec<String>>,
    #[uniffi(default = None)]
    secret: Option<String>,
    #[uniffi(default = None)]
    pin: Option<String>,
    #[uniffi(default = None)]
    require_pvtid: Option<bool>,
    #[uniffi(default = None)]
    signed_tokens: Option<bool>,
    #[uniffi(default = None)]
    bitrate: Option<u64>,
    #[uniffi(default = None)]
    bitrate_cap: Option<bool>,
    #[uniffi(default = None)]
    fir_freq: Option<u64>,
    #[uniffi(default = None)]
    publishers: Option<u64>,
    #[uniffi(default = None)]
    audiocodec: Option<VideoRoomAudioCodecList>,
    #[uniffi(default = None)]
    videocodec: Option<VideoRoomVideoCodecList>,
    #[uniffi(default = None)]
    vp9_profile: Option<String>,
    #[uniffi(default = None)]
    h264_profile: Option<String>,
    #[uniffi(default = None)]
    opus_fec: Option<bool>,
    #[uniffi(default = None)]
    opus_dtx: Option<bool>,
    #[uniffi(default = None)]
    audiolevel_ext: Option<bool>,
    #[uniffi(default = None)]
    audiolevel_event: Option<bool>,
    #[uniffi(default = None)]
    audio_active_packets: Option<u64>,
    #[uniffi(default = None)]
    audio_level_average: Option<u64>,
    #[uniffi(default = None)]
    videoorient_ext: Option<bool>,
    #[uniffi(default = None)]
    playoutdelay_ext: Option<bool>,
    #[uniffi(default = None)]
    transport_wide_cc_ext: Option<bool>,
    #[uniffi(default = None)]
    record: Option<bool>,
    #[uniffi(default = None)]
    record_dir: Option<String>,
    #[uniffi(default = None)]
    lock_record: Option<bool>,
    #[uniffi(default = None)]
    permanent: Option<bool>,
    #[uniffi(default = None)]
    notify_joining: Option<bool>,
    #[uniffi(default = None)]
    require_e2ee: Option<bool>,
    #[uniffi(default = None)]
    dummy_publisher: Option<bool>,
    #[uniffi(default = None)]
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

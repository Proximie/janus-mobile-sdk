use crate::plugins::common::JanusId;
use jarust::plugins::legacy_video_room::params;

pub type LegacyVideoRoomAudioCodec = params::LegacyVideoRoomAudioCodec;
pub type LegacyVideoRoomAudioCodecList = params::LegacyVideoRoomAudioCodecList;
pub type LegacyVideoRoomCreateParams = params::LegacyVideoRoomCreateParams;
pub type LegacyVideoRoomVideoCodec = params::LegacyVideoRoomVideoCodec;
pub type LegacyVideoRoomVideoCodecList = params::LegacyVideoRoomVideoCodecList;
pub type LegacyVideoRoomKickParams = params::LegacyVideoRoomKickParams;
pub type LegacyVideoRoomPublisherJoinParams = params::LegacyVideoRoomPublisherJoinParams;
pub type LegacyVideoRoomPublisherJoinParamsOptional =
    params::LegacyVideoRoomPublisherJoinParamsOptional;

#[uniffi::remote(Record)]
pub struct LegacyVideoRoomCreateParams {
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
    audiocodec: Option<LegacyVideoRoomAudioCodecList>,
    #[uniffi(default = None)]
    videocodec: Option<LegacyVideoRoomVideoCodecList>,
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
pub enum LegacyVideoRoomAudioCodec {
    OPUS,
    G722,
    PCMU,
    PCMA,
    ISAC32,
    ISAC16,
}

#[uniffi::remote(Record)]
pub struct LegacyVideoRoomAudioCodecList {
    codecs: Vec<LegacyVideoRoomAudioCodec>,
}

#[uniffi::remote(Enum)]
pub enum LegacyVideoRoomVideoCodec {
    VP8,
    VP9,
    H264,
    AV1,
    H265,
}

#[uniffi::remote(Record)]
pub struct LegacyVideoRoomVideoCodecList {
    codecs: Vec<LegacyVideoRoomVideoCodec>,
}

#[uniffi::remote(Record)]
pub struct LegacyVideoRoomKickParams {
    pub room: JanusId,
    pub participant: JanusId,
    pub secret: Option<String>,
}

#[uniffi::remote(Record)]
pub struct LegacyVideoRoomPublisherJoinParams {
    pub room: JanusId,
    pub optional: LegacyVideoRoomPublisherJoinParamsOptional,
}

#[uniffi::remote(Record)]
pub struct LegacyVideoRoomPublisherJoinParamsOptional {
    #[uniffi(default = None)]
    pub id: Option<JanusId>,
    #[uniffi(default = None)]
    pub display: Option<String>,
    #[uniffi(default = None)]
    pub token: Option<String>,
}

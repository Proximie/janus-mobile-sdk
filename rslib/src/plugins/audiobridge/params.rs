use crate::plugins::common::JanusId;
use jarust::plugins::audio_bridge::params;

pub type AudioBridgeCreateParams = params::AudioBridgeCreateParams;
pub type AudioBridgeConfigureParams = params::AudioBridgeConfigureParams;
pub type AudioBridgeJoinParamsOptional = params::AudioBridgeJoinParamsOptional;
pub type AudioBridgeCodec = params::AudioBridgeCodec;
pub type AudioBridgeRTP = params::AudioBridgeRTP;
pub type AudioBridgeRTPRequired = params::AudioBridgeRTPRequired;
pub type AudioBridgeRTPOptional = params::AudioBridgeRTPOptional;

#[uniffi::remote(Record)]
pub struct AudioBridgeCreateParams {
    #[uniffi(default = None)]
    room: Option<JanusId>,
    #[uniffi(default = None)]
    permanent: Option<bool>,
    #[uniffi(default = None)]
    description: Option<String>,
    #[uniffi(default = None)]
    secret: Option<String>,
    #[uniffi(default = None)]
    pin: Option<String>,
    #[uniffi(default = None)]
    is_private: Option<bool>,
    #[uniffi(default = None)]
    allowed: Option<Vec<String>>,
    #[uniffi(default = None)]
    sampling_rate: Option<u64>,
    #[uniffi(default = None)]
    spatial_audio: Option<bool>,
    #[uniffi(default = None)]
    audiolevel_ext: Option<bool>,
    #[uniffi(default = None)]
    audiolevel_event: Option<bool>,
    #[uniffi(default = None)]
    audio_active_packets: Option<u64>,
    #[uniffi(default = None)]
    default_expectedloss: Option<u64>,
    #[uniffi(default = None)]
    default_bitrate: Option<u64>,
    #[uniffi(default = None)]
    record: Option<bool>,
    #[uniffi(default = None)]
    record_file: Option<String>,
    #[uniffi(default = None)]
    record_dir: Option<String>,
    #[uniffi(default = None)]
    mjrs: Option<bool>,
    #[uniffi(default = None)]
    mjrs_dir: Option<String>,
    #[uniffi(default = None)]
    allow_rtp_participants: Option<bool>,
    #[uniffi(default = None)]
    groups: Option<Vec<String>>,
}

#[uniffi::remote(Record)]
pub struct AudioBridgeJoinParamsOptional {
    #[uniffi(default = None)]
    id: Option<JanusId>,
    #[uniffi(default = None)]
    group: Option<String>,
    #[uniffi(default = None)]
    pin: Option<String>,
    #[uniffi(default = None)]
    display: Option<String>,
    #[uniffi(default = None)]
    token: Option<String>,
    #[uniffi(default = None)]
    muted: Option<bool>,
    #[uniffi(default = None)]
    suspended: Option<bool>,
    #[uniffi(default = None)]
    pause_events: Option<bool>,
    #[uniffi(default = None)]
    codec: Option<AudioBridgeCodec>,
    #[uniffi(default = None)]
    bitrate: Option<u64>,
    #[uniffi(default = None)]
    quality: Option<u8>,
    #[uniffi(default = None)]
    expected_loss: Option<u8>,
    #[uniffi(default = None)]
    volume: Option<u64>,
    #[uniffi(default = None)]
    spatial_position: Option<u8>,
    #[uniffi(default = None)]
    secret: Option<String>,
    #[uniffi(default = None)]
    audio_level_average: Option<u64>,
    #[uniffi(default = None)]
    audio_active_packets: Option<u64>,
    #[uniffi(default = None)]
    record: Option<bool>,
    #[uniffi(default = None)]
    filename: Option<String>,
    #[uniffi(default = None)]
    generate_offer: Option<bool>,
    #[uniffi(default = None)]
    rtp: Option<AudioBridgeRTP>,
}

#[uniffi::remote(Enum)]
pub enum AudioBridgeCodec {
    Opus,
    /// A-Law
    Pcma,
    /// mu-Law
    Pcmu,
}

#[uniffi::remote(Record)]
pub struct AudioBridgeRTP {
    required: AudioBridgeRTPRequired,
    optional: AudioBridgeRTPOptional,
}

#[uniffi::remote(Record)]
pub struct AudioBridgeRTPRequired {
    ip: String,
    port: u16,
}

#[uniffi::remote(Record)]
pub struct AudioBridgeRTPOptional {
    #[uniffi(default = None)]
    payload_type: Option<String>,
    #[uniffi(default = None)]
    audiolevel_ext: Option<String>,
    #[uniffi(default = None)]
    fec: Option<bool>,
}

#[uniffi::remote(Record)]
pub struct AudioBridgeConfigureParams {
    #[uniffi(default = None)]
    muted: Option<bool>,
    #[uniffi(default = None)]
    display: Option<String>,
    #[uniffi(default = None)]
    bitrate: Option<u64>,
    #[uniffi(default = None)]
    quality: Option<u8>,
    #[uniffi(default = None)]
    expected_loss: Option<u8>,
    #[uniffi(default = None)]
    volume: Option<u64>,
    #[uniffi(default = None)]
    spatial_position: Option<u8>,
    #[uniffi(default = None)]
    denoise: Option<bool>,
    #[uniffi(default = None)]
    record: Option<bool>,
    #[uniffi(default = None)]
    filename: Option<String>,
    #[uniffi(default = None)]
    group: Option<String>,
}

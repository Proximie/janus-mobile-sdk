use crate::plugins::common::JanusId;
use jarust::plugins::audio_bridge::params;

pub type AudioBridgeCreateParams = params::AudioBridgeCreateParams;

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

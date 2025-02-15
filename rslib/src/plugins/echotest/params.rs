use jarust::plugins::echo_test::params;

pub type EchoTestStartParams = params::EchoTestStartParams;

#[uniffi::remote(Record)]
pub struct EchoTestStartParams {
    #[uniffi(default = None)]
    audio: Option<bool>,
    #[uniffi(default = None)]
    video: Option<bool>,
    #[uniffi(default = None)]
    bitrate: Option<u32>,
    #[uniffi(default = None)]
    record: Option<bool>,
    #[uniffi(default = None)]
    filename: Option<String>,
    #[uniffi(default = None)]
    substream: Option<u32>,
    #[uniffi(default = None)]
    temporal: Option<u32>,
    #[uniffi(default = None)]
    fallback: Option<u32>,
    #[uniffi(default = None)]
    svc: Option<bool>,
    #[uniffi(default = None)]
    spatial_layer: Option<u32>,
    #[uniffi(default = None)]
    temporal_layer: Option<u32>,
    #[uniffi(default = None)]
    audiocodec: Option<String>,
    #[uniffi(default = None)]
    videocodec: Option<String>,
    #[uniffi(default = None)]
    videoprofile: Option<String>,
    #[uniffi(default = None)]
    opusred: Option<bool>,
    #[uniffi(default = None)]
    min_delay: Option<i32>,
    #[uniffi(default = None)]
    max_delay: Option<i32>,
}

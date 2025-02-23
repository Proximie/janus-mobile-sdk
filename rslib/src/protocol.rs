use jarust::interface::japrotocol;
use std::collections::HashMap;

pub type Candidate = japrotocol::Candidate;
pub type GenericEvent = japrotocol::GenericEvent;
pub type Jsep = japrotocol::Jsep;
pub type JsepType = japrotocol::JsepType;
pub type MetaData = japrotocol::MetaData;
pub type ServerInfoRsp = japrotocol::ServerInfoRsp;

#[uniffi::remote(Record)]
pub struct Candidate {
    pub candidate: String,
    pub sdp_mid: String,
    pub sdp_mline_index: u32,
}

#[uniffi::remote(Enum)]
pub enum GenericEvent {
    Detached,
    /// The PeerConnection was closed, either by Janus or by the user/application, and as such cannot be used anymore.
    Hangup,
    /// Whether Janus is receiving (receiving: true/false) audio/video (type: "audio/video") on this PeerConnection.
    Media,
    Timeout,
    /// ICE and DTLS succeeded, and so Janus correctly established a PeerConnection with the user/application.
    WebrtcUp,
    /// Whether Janus is reporting trouble sending/receiving (uplink: true/false) media on this PeerConnection.
    Slowlink,
    Trickle,
}

#[uniffi::remote(Record)]
pub struct ServerInfoRsp {
    pub name: String,
    pub version: u64,
    pub version_string: String,
    pub author: String,
    pub commit_hash: String,
    pub compile_time: String,
    pub log_to_stdout: bool,
    pub log_to_file: bool,
    pub data_channels: bool,
    pub accepting_new_sessions: bool,
    pub session_timeout: u64,
    pub reclaim_session_timeout: u64,
    pub candidates_timeout: u64,
    pub server_name: String,
    pub local_ip: String,
    pub ipv6: bool,
    pub ice_lite: bool,
    pub ice_tcp: bool,
    pub ice_nomination: String,
    pub ice_keepalive_conncheck: bool,
    pub full_trickle: bool,
    pub mdns_enabled: bool,
    pub min_nack_queue: u64,
    pub twcc_period: u64,
    pub dtls_mtu: u64,
    pub static_event_loops: u64,
    pub api_secret: bool,
    pub auth_token: bool,
    pub event_handlers: bool,
    pub opaqueid_in_api: bool,
    pub dependencies: HashMap<String, String>,
    pub transports: HashMap<String, MetaData>,
    pub plugins: HashMap<String, MetaData>,
}

#[uniffi::remote(Record)]
pub struct MetaData {
    pub name: String,
    pub author: String,
    pub description: String,
    pub version_string: String,
    pub version: u64,
}

#[uniffi::remote(Enum)]
pub enum JsepType {
    Offer,
    Answer,
}

#[uniffi::remote(Record)]
pub struct Jsep {
    pub jsep_type: JsepType,
    #[uniffi(default = None)]
    pub trickle: Option<bool>,
    pub sdp: String,
}

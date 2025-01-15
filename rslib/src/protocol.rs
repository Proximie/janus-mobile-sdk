use jarust::interface::japrotocol::Candidate as ExternalCandidate;
use jarust::interface::japrotocol::GenericEvent as ExternalGenericEvent;
use jarust::interface::japrotocol::Jsep as ExternalJsep;
use jarust::interface::japrotocol::JsepType as ExternalJsepType;
use jarust::interface::japrotocol::MetaData as ExternalMetaData;
use jarust::interface::japrotocol::ServerInfoRsp as ExternalServerInfoRsp;
use std::collections::HashMap;

#[derive(uniffi::Record)]
pub struct Jsep {
    pub jsep_type: JsepType,
    pub sdp: String,
}

#[derive(uniffi::Enum)]
pub enum JsepType {
    Offer,
    Answer,
}

impl From<Jsep> for ExternalJsep {
    fn from(val: Jsep) -> Self {
        ExternalJsep {
            jsep_type: match val.jsep_type {
                JsepType::Offer => ExternalJsepType::Offer,
                JsepType::Answer => ExternalJsepType::Answer,
            },
            trickle: None,
            sdp: val.sdp,
        }
    }
}

impl From<ExternalJsep> for Jsep {
    fn from(val: ExternalJsep) -> Self {
        Jsep {
            jsep_type: match val.jsep_type {
                ExternalJsepType::Offer => JsepType::Offer,
                ExternalJsepType::Answer => JsepType::Answer,
            },
            sdp: val.sdp,
        }
    }
}

#[derive(uniffi::Record)]
pub struct Candidate {
    pub candidate: String,
    pub sdp_mid: String,
    pub sdp_mline_index: String,
}

impl From<Candidate> for ExternalCandidate {
    fn from(val: Candidate) -> Self {
        ExternalCandidate {
            candidate: val.candidate,
            sdp_mid: val.sdp_mid,
            sdp_mline_index: val.sdp_mline_index,
        }
    }
}

#[derive(uniffi::Enum)]
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

impl From<ExternalGenericEvent> for GenericEvent {
    fn from(val: ExternalGenericEvent) -> Self {
        match val {
            ExternalGenericEvent::Detached => GenericEvent::Detached,
            ExternalGenericEvent::Hangup => GenericEvent::Hangup,
            ExternalGenericEvent::Media => GenericEvent::Media,
            ExternalGenericEvent::Timeout => GenericEvent::Timeout,
            ExternalGenericEvent::WebrtcUp => GenericEvent::WebrtcUp,
            ExternalGenericEvent::Slowlink => GenericEvent::Slowlink,
            ExternalGenericEvent::Trickle => GenericEvent::Trickle,
        }
    }
}

#[derive(uniffi::Record)]
pub struct ServerInfo {
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

impl From<ExternalServerInfoRsp> for ServerInfo {
    fn from(val: ExternalServerInfoRsp) -> Self {
        ServerInfo {
            name: val.name,
            version: val.version,
            version_string: val.version_string,
            author: val.author,
            commit_hash: val.commit_hash,
            compile_time: val.compile_time,
            log_to_stdout: val.log_to_stdout,
            log_to_file: val.log_to_file,
            data_channels: val.data_channels,
            accepting_new_sessions: val.accepting_new_sessions,
            session_timeout: val.session_timeout,
            reclaim_session_timeout: val.reclaim_session_timeout,
            candidates_timeout: val.candidates_timeout,
            server_name: val.server_name,
            local_ip: val.local_ip,
            ipv6: val.ipv6,
            ice_lite: val.ice_lite,
            ice_tcp: val.ice_tcp,
            ice_nomination: val.ice_nomination,
            ice_keepalive_conncheck: val.ice_keepalive_conncheck,
            full_trickle: val.full_trickle,
            mdns_enabled: val.mdns_enabled,
            min_nack_queue: val.min_nack_queue,
            twcc_period: val.twcc_period,
            dtls_mtu: val.dtls_mtu,
            static_event_loops: val.static_event_loops,
            api_secret: val.api_secret,
            auth_token: val.auth_token,
            event_handlers: val.event_handlers,
            opaqueid_in_api: val.opaqueid_in_api,
            dependencies: val.dependencies,
            transports: val
                .transports
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            plugins: val
                .plugins
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

#[derive(uniffi::Record)]
pub struct MetaData {
    pub name: String,
    pub author: String,
    pub description: String,
    pub version_string: String,
    pub version: u64,
}

impl From<ExternalMetaData> for MetaData {
    fn from(val: ExternalMetaData) -> Self {
        MetaData {
            name: val.name,
            author: val.author,
            description: val.description,
            version_string: val.version_string,
            version: val.version,
        }
    }
}

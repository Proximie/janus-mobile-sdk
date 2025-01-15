use jarust::interface::japrotocol::Candidate as ExternalCandidate;
use jarust::interface::japrotocol::GenericEvent as ExternalGenericEvent;
use jarust::interface::japrotocol::Jsep as ExternalJsep;
use jarust::interface::japrotocol::JsepType as ExternalJsepType;

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

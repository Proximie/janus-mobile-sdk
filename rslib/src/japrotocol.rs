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

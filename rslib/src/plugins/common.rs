use jarust::plugins::common;

pub type JanusId = common::JanusId;
pub type U63 = common::U63;

#[uniffi::remote(Record)]
pub struct U63 {
    inner: u64,
}

#[uniffi::remote(Enum)]
pub enum JanusId {
    String(String),
    Uint(U63),
}

use crate::sys::OS_LOG_TYPE_DEBUG;
use crate::sys::OS_LOG_TYPE_DEFAULT;
use crate::sys::OS_LOG_TYPE_ERROR;
use crate::sys::OS_LOG_TYPE_FAULT;
use crate::sys::OS_LOG_TYPE_INFO;

#[repr(u8)]
#[derive(Debug)]
pub enum Level {
    Debug = OS_LOG_TYPE_DEBUG,
    Info = OS_LOG_TYPE_INFO,
    Default = OS_LOG_TYPE_DEFAULT,
    Error = OS_LOG_TYPE_ERROR,
    Fault = OS_LOG_TYPE_FAULT,
}

impl From<Level> for u8 {
    fn from(other: Level) -> Self {
        match other {
            Level::Debug => OS_LOG_TYPE_DEBUG,
            Level::Info => OS_LOG_TYPE_INFO,
            Level::Default => OS_LOG_TYPE_DEFAULT,
            Level::Error => OS_LOG_TYPE_ERROR,
            Level::Fault => OS_LOG_TYPE_FAULT,
        }
    }
}

impl From<tracing_core::Level> for Level {
    fn from(other: tracing_core::Level) -> Self {
        match other {
            tracing_core::Level::TRACE => Self::Debug,
            tracing_core::Level::DEBUG => Self::Info,
            tracing_core::Level::INFO => Self::Default,
            tracing_core::Level::WARN => Self::Error,
            tracing_core::Level::ERROR => Self::Fault,
        }
    }
}

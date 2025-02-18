#![cfg(any(target_os = "ios", target_os = "macos"))]

pub mod apple_log;
pub mod level;
pub mod subscriber;
pub mod sys;
pub mod utils;

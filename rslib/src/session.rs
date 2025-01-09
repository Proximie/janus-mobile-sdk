use crate::error::JanusGatewayError;
use crate::handle::Handle;
use crate::plugins::echotest::EchotestHandle;
use jarust::core::japlugin::Attach;
use jarust::core::jasession::JaSession;
use jarust::plugins::echo_test::jahandle_ext::EchoTest;
use std::time::Duration;

#[derive(uniffi::Object)]
pub struct Session {
    inner: JaSession,
}

impl Session {
    pub fn new(session: JaSession) -> Self {
        Self { inner: session }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl Session {
    pub async fn attach(
        &self,
        plugin_id: &str,
        timeout: Duration,
    ) -> crate::JanusGatewayResult<Handle> {
        let (handle, receiver) = match self.inner.attach(plugin_id.to_string(), timeout).await {
            Ok(handle) => handle,
            Err(why) => {
                return Err(JanusGatewayError::HandleCreationFailure {
                    plugin: plugin_id.to_string(),
                    reason: why.to_string(),
                })
            }
        };
        Ok(Handle::new(handle, receiver))
    }

    pub async fn attach_echo_test(
        &self,
        timeout: Duration,
    ) -> crate::JanusGatewayResult<EchotestHandle> {
        let (handle, receiver) = match self.inner.attach_echo_test(timeout).await {
            Ok(handle) => handle,
            Err(why) => {
                return Err(JanusGatewayError::HandleCreationFailure {
                    plugin: "echotest".to_string(),
                    reason: why.to_string(),
                })
            }
        };
        Ok(EchotestHandle::new(handle, receiver))
    }
}

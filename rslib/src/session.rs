use crate::error::JanusGatewayCommunicationError;
use crate::error::JanusGatewayHandleError;
use crate::handle::Handle;
use crate::plugins::audiobridge::handle::AudioBridgeHandle;
use crate::plugins::echotest::handle::EchotestHandle;
use jarust::core::japlugin::Attach;
use jarust::core::jasession::JaSession;
use jarust::plugins::audio_bridge::jahandle_ext::AudioBridge;
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
    ) -> Result<Handle, JanusGatewayHandleError> {
        let (handle, receiver) = match self.inner.attach(plugin_id.to_string(), timeout).await {
            Ok(handle) => handle,
            Err(why) => {
                return Err(JanusGatewayHandleError::HandleCreationFailure {
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
    ) -> Result<EchotestHandle, JanusGatewayHandleError> {
        let (handle, receiver) = match self.inner.attach_echo_test(timeout).await {
            Ok(handle) => handle,
            Err(why) => {
                return Err(JanusGatewayHandleError::HandleCreationFailure {
                    plugin: "echotest".to_string(),
                    reason: why.to_string(),
                })
            }
        };
        Ok(EchotestHandle::new(handle, receiver))
    }

    pub async fn attach_audio_bridge(
        &self,
        timeout: Duration,
    ) -> Result<AudioBridgeHandle, JanusGatewayHandleError> {
        let (handle, receiver) = match self.inner.attach_audio_bridge(timeout).await {
            Ok(handle) => handle,
            Err(why) => {
                return Err(JanusGatewayHandleError::HandleCreationFailure {
                    plugin: "audiobridge".to_string(),
                    reason: why.to_string(),
                })
            }
        };
        Ok(AudioBridgeHandle::new(handle, receiver))
    }

    pub async fn destory(&self, timeout: Duration) -> Result<(), JanusGatewayCommunicationError> {
        if let Err(why) = self.inner.destroy(timeout).await {
            return Err(JanusGatewayCommunicationError::SendFailure {
                reason: why.to_string(),
            });
        };
        Ok(())
    }
}

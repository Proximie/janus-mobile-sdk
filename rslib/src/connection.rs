use crate::config::Config;
use crate::error::JanusGatewayCommunicationError;
use crate::error::JanusGatewayConnectionError;
use crate::error::JanusGatewaySessionError;
use crate::protocol::ServerInfo;
use crate::session::Session;
use jarust::core::connect;
use jarust::core::jaconfig::JaConfig;
use jarust::core::jaconfig::JanusAPI;
use jarust::core::jaconnection::JaConnection;
use jarust::interface::tgenerator::RandomTransactionGenerator;
use std::time::Duration;

#[derive(uniffi::Object)]
pub struct Connection {
    inner: JaConnection,
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn raw_janus_connect(config: Config) -> Result<Connection, JanusGatewayConnectionError> {
    let config = JaConfig {
        url: config.url,
        capacity: config.capacity.into(),
        apisecret: config.apisecret,
        server_root: config.server_root.unwrap_or("janus".to_string()),
    };

    let connection = match connect(config, JanusAPI::WebSocket, RandomTransactionGenerator).await {
        Ok(connection) => connection,
        Err(why) => {
            return Err(JanusGatewayConnectionError::ConnectionFailure {
                reason: why.to_string(),
            })
        }
    };

    Ok(Connection { inner: connection })
}

#[uniffi::export(async_runtime = "tokio")]
impl Connection {
    pub async fn create_session(
        &self,
        ka_interval: u32,
        timeout: Duration,
    ) -> Result<Session, JanusGatewaySessionError> {
        let mut connection = self.inner.clone();
        let session = match connection.create_session(ka_interval, timeout).await {
            Ok(session) => session,
            Err(why) => {
                return Err(JanusGatewaySessionError::SessionCreationFailure {
                    reason: why.to_string(),
                })
            }
        };
        Ok(Session::new(session))
    }

    pub async fn server_info(
        &self,
        timeout: Duration,
    ) -> Result<ServerInfo, JanusGatewayCommunicationError> {
        let info = match self.inner.server_info(timeout).await {
            Ok(info) => info,
            Err(why) => {
                return Err(JanusGatewayCommunicationError::SendFailure {
                    reason: why.to_string(),
                })
            }
        };
        Ok(info.into())
    }
}

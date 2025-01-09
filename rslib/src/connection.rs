use crate::config::Config;
use crate::error::JanusGatewayError;
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
pub async fn raw_janus_connect(config: Config) -> crate::JanusGatewayResult<Connection> {
    let config = JaConfig {
        url: config.url,
        capacity: config.capacity.into(),
        apisecret: config.apisecret,
        server_root: config.server_root.unwrap_or("janus".to_string()),
    };

    let connection = match connect(config, JanusAPI::WebSocket, RandomTransactionGenerator).await {
        Ok(connection) => connection,
        Err(why) => {
            return Err(JanusGatewayError::ConnectionFailure {
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
    ) -> crate::JanusGatewayResult<Session> {
        let mut connection = self.inner.clone();
        let session = match connection.create_session(ka_interval, timeout).await {
            Ok(session) => session,
            Err(why) => {
                return Err(JanusGatewayError::SessionCreationFailure {
                    reason: why.to_string(),
                })
            }
        };
        Ok(Session::new(session))
    }
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum JanusGatewayCommunicationError {
    #[error("Could not serialize ${body}")]
    Serialize { body: String },
    #[error("Failed to send ${reason}")]
    SendFailure { reason: String },
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum JanusGatewayConnectionError {
    #[error("Failed to connect to server, reason: ${reason}")]
    ConnectionFailure { reason: String },
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum JanusGatewaySessionError {
    #[error("Failed to create a session with server: reason ${reason}")]
    SessionCreationFailure { reason: String },
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum JanusGatewayHandleError {
    #[error("Failed to attach ${plugin} handle, reason ${reason}")]
    HandleCreationFailure { plugin: String, reason: String },
}

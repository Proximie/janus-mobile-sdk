//! Lets the host platform (iOS/Android) supply its own byte transport for Janus.
//!
//! The gateway is fronted by a Socket.IO server, which isn't a transport jarust ships
//! natively. Rather than run a second Socket.IO stack inside the FFI binary, the host
//! implements [`JanusTransport`] with its own client and we drive jarust's
//! `CustomInterface` (full Janus protocol) over it.
//!
//! Directionality:
//! - Outbound (Rust -> host): [`JanusTransport::connect`] (async), [`JanusTransport::send`]
//!   and [`JanusTransport::disconnect`] (sync fire-and-forget).
//! - Inbound (host -> Rust): the host pushes each raw `janus` payload into the
//!   [`TransportInbound`] sink handed to it on connect.

use bytes::Bytes;
use jarust::interface::transport_trait::Transport;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::mpsc;

/// Errors a host transport can report back to Rust.
#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum TransportError {
    #[error("Failed to connect the transport: {reason}")]
    ConnectionFailure { reason: String },
    #[error("Failed to send over the transport: {reason}")]
    SendFailure { reason: String },
}

/// Implemented on the host (Swift/Kotlin). Moves raw Janus bytes; it does not need to
/// understand the Janus protocol.
#[uniffi::export(callback_interface)]
#[async_trait::async_trait]
pub trait JanusTransport: Send + Sync + Debug {
    /// Establishes the underlying connection to `url`. The host must retain `sink` and
    /// feed every inbound `janus` payload into it via [`TransportInbound::receive`].
    async fn connect(
        &self,
        url: String,
        sink: Arc<TransportInbound>,
    ) -> Result<(), TransportError>;

    /// Sends one already-serialized Janus request. Fire-and-forget: response
    /// correlation happens on the Rust side via the inbound stream.
    fn send(&self, data: Vec<u8>);

    /// Tears down the underlying connection.
    fn disconnect(&self);
}

/// Rust-owned sink the host pushes inbound Janus payloads into. Each `receive` call
/// must carry the bytes of exactly one `{"janus": ...}` JSON message.
#[derive(uniffi::Object)]
pub struct TransportInbound {
    tx: mpsc::UnboundedSender<Bytes>,
}

impl Debug for TransportInbound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TransportInbound").finish_non_exhaustive()
    }
}

#[uniffi::export]
impl TransportInbound {
    /// Forwards one raw Janus payload from the host into jarust's demultiplexer.
    pub fn receive(&self, data: Vec<u8>) {
        if self.tx.send(Bytes::from(data)).is_err() {
            tracing::warn!("Dropping inbound payload: Janus interface is gone");
        }
    }
}

/// Bridges a host [`JanusTransport`] to jarust's [`Transport`] trait.
///
/// Owns the inbound channel's receiver until [`connect`](Transport::connect) hands it
/// to jarust; the matching sender lives inside the [`TransportInbound`] the host holds.
#[derive(Debug)]
pub struct TransportAdapter {
    inner: Arc<dyn JanusTransport>,
    // `connect` on jarust's trait takes `&self`, so the one-shot receiver is stashed
    // here and taken out on first connect.
    inbound_rx: Mutex<Option<mpsc::UnboundedReceiver<Bytes>>>,
    sink: Arc<TransportInbound>,
}

impl TransportAdapter {
    pub fn new(inner: Box<dyn JanusTransport>) -> Self {
        let (tx, rx) = mpsc::unbounded_channel::<Bytes>();
        Self {
            inner: Arc::from(inner),
            inbound_rx: Mutex::new(Some(rx)),
            sink: Arc::new(TransportInbound { tx }),
        }
    }
}

#[async_trait::async_trait]
impl Transport for TransportAdapter {
    async fn connect(
        &self,
        url: &str,
    ) -> Result<mpsc::UnboundedReceiver<Bytes>, jarust::interface::Error> {
        let rx = self
            .inbound_rx
            .lock()
            .expect("inbound receiver mutex poisoned")
            .take()
            .ok_or(jarust::interface::Error::TransportNotOpened)?;

        self.inner
            .connect(url.to_string(), self.sink.clone())
            .await
            .map_err(|why| jarust::interface::Error::InvalidJanusRequest {
                reason: why.to_string(),
            })?;

        Ok(rx)
    }

    async fn send(&self, data: &[u8], _path: &str) -> Result<(), jarust::interface::Error> {
        self.inner.send(data.to_vec());
        Ok(())
    }
}

impl Drop for TransportAdapter {
    fn drop(&mut self) {
        self.inner.disconnect();
    }
}

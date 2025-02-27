/// This macro is used to add the default handle functions to a plugin handle.
/// And adds a drop implementation.
#[macro_export]
macro_rules! base_handle {
    ($handle_name:ident) => {
        #[uniffi::export(async_runtime = "tokio")]
        impl $handle_name {
            pub async fn fire_and_forget(
                &self,
                data: Vec<u8>,
            ) -> Result<(), JanusGatewayCommunicationError> {
                let Ok(body) = serde_json::from_slice(&data) else {
                    return Err(JanusGatewayCommunicationError::Serialize {
                        body: String::from_utf8_lossy(&data).to_string(),
                    });
                };
                if let Err(why) = self.inner.fire_and_forget(body).await {
                    return Err(JanusGatewayCommunicationError::SendFailure {
                        reason: why.to_string(),
                    });
                };
                Ok(())
            }

            pub async fn fire_and_forget_with_jsep(
                &self,
                data: Vec<u8>,
                jsep: Jsep,
            ) -> Result<(), JanusGatewayCommunicationError> {
                let Ok(body) = serde_json::from_slice(&data) else {
                    return Err(JanusGatewayCommunicationError::Serialize {
                        body: String::from_utf8_lossy(&data).to_string(),
                    });
                };
                if let Err(why) = self
                    .inner
                    .fire_and_forget_with_jsep(body, jsep.into())
                    .await
                {
                    return Err(JanusGatewayCommunicationError::SendFailure {
                        reason: why.to_string(),
                    });
                };
                Ok(())
            }

            pub async fn send_waiton_ack(
                &self,
                data: Vec<u8>,
                timeout: Duration,
            ) -> Result<String, JanusGatewayCommunicationError> {
                let Ok(body) = serde_json::from_slice(&data) else {
                    return Err(JanusGatewayCommunicationError::Serialize {
                        body: String::from_utf8_lossy(&data).to_string(),
                    });
                };
                self.inner
                    .send_waiton_ack(body, timeout)
                    .await
                    .map_err(|err| JanusGatewayCommunicationError::SendFailure {
                        reason: err.to_string(),
                    })
            }

            pub async fn send_waiton_result(
                &self,
                data: Vec<u8>,
                timeout: Duration,
            ) -> Result<Vec<u8>, JanusGatewayCommunicationError> {
                let Ok(body) = serde_json::from_slice(&data) else {
                    return Err(JanusGatewayCommunicationError::Serialize {
                        body: String::from_utf8_lossy(&data).to_string(),
                    });
                };
                let result = match self.inner.send_waiton_rsp::<Value>(body, timeout).await {
                    Ok(result) => result,
                    Err(why) => {
                        return Err(JanusGatewayCommunicationError::SendFailure {
                            reason: why.to_string(),
                        });
                    }
                };
                let Ok(result) = serde_json::from_value(result) else {
                    return Err(JanusGatewayCommunicationError::Serialize {
                        body: String::from_utf8_lossy(&data).to_string(),
                    });
                };
                Ok(result)
            }

            pub async fn hangup(&self) -> Result<(), JanusGatewayCommunicationError> {
                self.inner.hangup().await.map_err(|err| {
                    JanusGatewayCommunicationError::SendFailure {
                        reason: err.to_string(),
                    }
                })
            }

            pub async fn detach(&self) -> Result<(), JanusGatewayCommunicationError> {
                self.inner.detach().await.map_err(|err| {
                    JanusGatewayCommunicationError::SendFailure {
                        reason: err.to_string(),
                    }
                })
            }

            pub async fn trickle_single_candidate(
                &self,
                candidate: Candidate,
                timeout: Duration,
            ) -> Result<(), JanusGatewayCommunicationError> {
                self.inner
                    .trickle_single_candidate(candidate.into(), timeout)
                    .await
                    .map_err(|err| JanusGatewayCommunicationError::SendFailure {
                        reason: err.to_string(),
                    })
            }

            pub async fn trickle_candidates(
                &self,
                candidates: Vec<Candidate>,
                timeout: Duration,
            ) -> Result<(), JanusGatewayCommunicationError> {
                self.inner
                    .trickle_candidates(candidates.into_iter().map(Into::into).collect(), timeout)
                    .await
                    .map_err(|err| JanusGatewayCommunicationError::SendFailure {
                        reason: err.to_string(),
                    })
            }

            pub async fn complete_trickle(
                &self,
                timeout: Duration,
            ) -> Result<(), JanusGatewayCommunicationError> {
                self.inner.complete_trickle(timeout).await.map_err(|err| {
                    JanusGatewayCommunicationError::SendFailure {
                        reason: err.to_string(),
                    }
                })
            }
        }

        impl Drop for $handle_name {
            fn drop(&mut self) {
                if let Ok(Some(abort_handle)) = self.abort_handle.lock().map(|mut x| x.take()) {
                    abort_handle.abort();
                }
            }
        }
    };
}

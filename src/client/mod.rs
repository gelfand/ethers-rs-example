use ethers::prelude::{FromErr, Middleware, PubsubClient};
use futures::StreamExt;
use std::fmt::Debug;
use thiserror::Error;
use tracing::{error, info};

#[derive(Debug, Error)]
pub struct ClientMiddleware<M>
where
    M: Middleware,
{
    inner: M,
}

impl<M> ClientMiddleware<M>
where
    M: Middleware,
{
    pub fn new(inner: M) -> Self
    where
        M: Middleware,
    {
        Self { inner }
    }

    pub async fn listen_transactions(&self) -> Result<(), ClientError<M>>
    where
        <M as Middleware>::Provider: PubsubClient + Send + Sync + Debug,
    {
        self.subscribe_pending_txs()
            .await
            .unwrap()
            .for_each(|hash| async move {
                match self.get_transaction(hash).await {
                    Ok(transaction) => {
                        if transaction.is_none() {
                            return;
                        }
                        let transaction = transaction.unwrap();
                        info!(
                            "Hash: {:?} From: {:?} Value: {:?}",
                            hash, transaction.from, transaction.value
                        );
                    }
                    _ => (),
                }
            })
            .await;
        Ok(())
    }
}

#[derive(Error, Debug)]
/// Thrown when an error happens at the Client level.
pub enum ClientError<M: Middleware> {
    /// Thrown when the internal middleware errors
    #[error("{0}")]
    MiddlewareError(M::Error),
}
impl<M: Middleware> FromErr<M::Error> for ClientError<M> {
    fn from(err: M::Error) -> Self {
        ClientError::MiddlewareError(err)
    }
}

impl<M> Middleware for ClientMiddleware<M>
where
    M: Middleware,
{
    type Error = ClientError<M>;
    type Provider = M::Provider;
    type Inner = M;
    fn inner(&self) -> &M {
        &self.inner
    }
}

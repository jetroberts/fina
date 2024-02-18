use std::{error::Error, fmt::Display, sync::Arc};

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::service::transaction::TransactionWrite;

use super::base::{DatabaseError, DatabaseInit};

pub struct Redis {
    client: Option<Arc<RwLock<redis::Client>>>,
    connection: Option<Arc<RwLock<redis::Connection>>>,
}

impl Redis {
    pub fn _new() -> Self {
        Self {
            client: None,
            connection: None,
        }
    }
}

#[derive(Debug)]
struct RedisClientConnectionError;

impl Display for RedisClientConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unable to create redis client")
    }
}

impl Error for RedisClientConnectionError {}

#[derive(Debug)]
struct RedisConnectionError;

impl Error for RedisConnectionError {}

impl Display for RedisConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unable to create redis connection")
    }
}

impl DatabaseInit for Redis {
    async fn connect(&mut self) -> Result<(), DatabaseError> {
        let client = redis::Client::open("redis://redis/")
            .map_err(|e| DatabaseError::ClientError(e.to_string()))?;

        let conn = client
            .get_connection()
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

        self.client = Some(Arc::new(RwLock::new(client)));
        self.connection = Some(Arc::new(RwLock::new(conn)));

        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), DatabaseError> {
        // TODO: improve this disconnect function. Currently this is a memory leak
        self.connection = None;
        self.client = None;
        Ok(())
    }
}

impl TransactionWrite for Redis {
    async fn create_transaction(
        &mut self,
        _create_transaction: crate::service::transaction::CreateTransaction,
    ) -> Result<Uuid, DatabaseError> {
        todo!()
    }

    async fn delete_transactions(&mut self) -> Result<(), DatabaseError> {
        todo!()
    }
}

fn create_new_uuid() -> String {
    let id = uuid::Uuid::new_v4();
    id.to_string()
}

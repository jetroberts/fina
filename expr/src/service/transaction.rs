use core::fmt;
use std::{fmt::Display, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    database::base::{DatabaseError, DatabaseInit},
    models::transaction::{CreateTransaction, Transaction},
};

pub enum TransactionError {
    SaveError(String),
    FindError(String),
}

impl Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionError::FindError(e) => {
                write!(f, "TransactionError -> FindError, {}", e)
            }
            TransactionError::SaveError(e) => {
                write!(f, "TransactionError -> SaveError, {}", e)
            }
        }
    }
}

pub trait TransactionWrite {
    async fn create_transaction(
        &self,
        create_transaction: CreateTransaction,
    ) -> Result<Uuid, DatabaseError>;

    async fn delete_transaction(&self, id: &str) -> Result<(), DatabaseError>;
}

pub trait TransactionRead {
    async fn get_transaction(&self, id: &str) -> Result<Option<Transaction>, DatabaseError>;
    async fn get_transactions(&self) -> Result<Vec<Transaction>, DatabaseError>;
}

pub struct TransactionService<T>
where
    for<'a> T: DatabaseInit + TransactionWrite + TransactionRead,
{
    db: Arc<RwLock<T>>,
}

impl<T> TransactionService<T>
where
    for<'a> T: DatabaseInit + TransactionWrite + TransactionRead,
{
    pub fn new(db: T) -> TransactionService<T> {
        let db = Arc::new(RwLock::new(db));
        Self { db }
    }

    pub async fn create_transaction(
        &self,
        create_transaction: CreateTransaction,
    ) -> Result<(), TransactionError> {
        let db_connection = self.db.write().await;

        db_connection
            .create_transaction(create_transaction)
            .await
            .map_err(|e| TransactionError::SaveError(e.to_string()))?;

        Ok(())
    }

    pub async fn find_transaction(
        &self,
        id: &str,
    ) -> Result<Option<Transaction>, TransactionError> {
        let db_connection = self.db.read().await;

        let transaction: Option<Transaction> = db_connection
            .get_transaction(id)
            .await
            .map_err(|e| TransactionError::FindError(e.to_string()))?;

        return Ok(transaction);
    }

    pub async fn find_transactions(&self) -> Result<Vec<Transaction>, TransactionError> {
        let db_connection = self.db.read().await;

        let transactions: Vec<Transaction> = db_connection
            .get_transactions()
            .await
            .map_err(|e| TransactionError::FindError(e.to_string()))?;

        return Ok(transactions);
    }

    pub async fn delete_transaction(&self, id: &str) -> Result<(), TransactionError> {
        let db_connection = self.db.read().await;

        let _ = db_connection
            .delete_transaction(id)
            .await
            .map_err(|e| TransactionError::FindError(e.to_string()))?;

        return Ok(());
    }
    // there will need to be something to do with categorising the transactions
}

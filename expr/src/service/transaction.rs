use chrono::{DateTime, NaiveDateTime, Utc};
use core::fmt;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, sync::Arc};
use tokio::sync::RwLock;

use crate::database::base::{DatabaseInit, DatabaseRead, DatabaseWrite, TransactionWrite};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id: String,
    pub account_type: String,
    pub payment_date: NaiveDateTime,
    pub amount: f64,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransaction {
    pub account_type: String,
    pub payment_date: NaiveDateTime,
    pub amount: f64,
    pub description: String,
}

impl Display for CreateTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "account_type: {}, date: {}, amount: {}",
            self.account_type, self.payment_date, self.amount
        )
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Id: {}\n, Type: {}\nDate: {}\nAmount: {}\nDescription: {}",
            self.id, self.account_type, self.payment_date, self.amount, self.description
        )
    }
}

pub enum TransactionError {
    SaveError(String),
    FindError(String),
    DeleteError(String),
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
            TransactionError::DeleteError(e) => {
                write!(f, "TransactionError -> DeleteError, {}", e)
            }
        }
    }
}

pub struct TransactionService<T>
where
    for<'a> T: DatabaseInit + DatabaseWrite + DatabaseRead,
{
    db: Arc<RwLock<T>>,
}

impl<T> TransactionService<T>
where
    for<'a> T: DatabaseInit + DatabaseWrite + DatabaseRead + TransactionWrite,
{
    pub fn new(db: T) -> TransactionService<T> {
        let db = Arc::new(RwLock::new(db));
        Self { db }
    }

    pub async fn create_transaction(
        &self,
        create_transaction: CreateTransaction,
    ) -> Result<(), TransactionError> {
        let mut db_connection = self.db.write().await;

        db_connection
            .save::<CreateTransaction>(create_transaction)
            .await
            .map_err(|e| TransactionError::SaveError(e.to_string()))?;

        Ok(())
    }

    pub async fn update_transaction(
        &self,
        create_transaction: CreateTransaction,
    ) -> Result<(), TransactionError> {
        let mut db_connection = self.db.write().await;

        db_connection
            .save::<CreateTransaction>(create_transaction)
            .await
            .map_err(|e| TransactionError::SaveError(e.to_string()))?;

        Ok(())
    }

    pub async fn find_transaction(
        &self,
        id: &str,
    ) -> Result<Option<Transaction>, TransactionError> {
        let mut db_connection = self.db.write().await;

        let transaction: Option<Transaction> = db_connection
            .find(id)
            .await
            .map_err(|e| TransactionError::FindError(e.to_string()))?;

        return Ok(transaction);
    }

    pub async fn find_transactions(&self) -> Result<Vec<Transaction>, TransactionError> {
        let mut db_connection = self.db.write().await;

        let transactions: Vec<Transaction> = db_connection
            .find_all()
            .await
            .map_err(|e| TransactionError::FindError(e.to_string()))?;

        return Ok(transactions);
    }

    pub async fn delete_transaction(&self, id: &str) -> Result<bool, TransactionError> {
        let mut db_connection = self.db.write().await;

        let has_deleted = db_connection
            .delete(id)
            .await
            .map_err(|e| TransactionError::DeleteError(e.to_string()))?;

        return Ok(has_deleted);
    }
    // there will need to be something to do with categorising the transactions
}

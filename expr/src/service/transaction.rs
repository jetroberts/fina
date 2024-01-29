use core::fmt;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, sync::Arc};
use tokio::sync::RwLock;

use crate::database::base::{DatabaseInit, DatabaseRead, DatabaseWrite};

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    id: String,
    account_type: String,
    date: String,
    amount: f64,
    description: String,
    created_at: String,
    category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransaction {
    pub account_type: String,
    pub date: String,
    pub amount: f64,
    pub description: String,
}

impl Display for CreateTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "account_type: {}, date: {}, amount: {}",
            self.account_type, self.date, self.amount
        )
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Id: {}\n, Type: {}\nDate: {}\nAmount: {}\nDescription: {}",
            self.id, self.account_type, self.date, self.amount, self.description
        )
    }
}

impl From<CreateTransaction> for Transaction {
    fn from(parsed_transaction: CreateTransaction) -> Self {
        // massive question mark around whether to create the id here...
        // might be a better idea to have a create / update / saved version of a Transaction
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            account_type: parsed_transaction.account_type,
            date: parsed_transaction.date,
            amount: parsed_transaction.amount,
            description: parsed_transaction.description,
            created_at: chrono::Utc::now().to_string(),
            category: None,
        }
    }
}

pub enum TransactionError {
    DatabaseConnectionError(String),
    SaveError(String),
    FindError(String),
    DeleteError(String),
}

impl Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionError::DatabaseConnectionError(e) => {
                write!(f, "TransactionError -> DatabaseError, {}", e)
            }
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
    for<'a> T: DatabaseInit + DatabaseWrite + DatabaseRead,
{
    pub fn new(db: T) -> TransactionService<T> {
        let db = Arc::new(RwLock::new(db));
        Self { db }
    }

    pub async fn save_transaction(
        &self,
        create_transaction: CreateTransaction,
    ) -> Result<(), TransactionError> {
        let mut db_connection = self.db.write().await;

        let t: Transaction = Transaction::from(create_transaction);

        db_connection
            .save::<Transaction>(t)
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

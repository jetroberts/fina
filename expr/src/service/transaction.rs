use core::fmt;
use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

use crate::database::base::{DatabaseInit, DatabaseRead, DatabaseWrite};

use super::parse::ParsedTransaction;

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    id: String,
    account_type: String,
    date: String,
    amount: f64,
    description: String,
}

pub enum TransactionError {
    DatabaseError(String),
}

impl Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionError::DatabaseError(e) => {
                write!(f, "TransactionError -> DatabaseError, {}", e)
            }
        }
    }
}

pub struct TransactionService<T>
where
    for<'a> T: DatabaseInit + DatabaseWrite + DatabaseRead,
{
    db: Arc<Mutex<T>>,
}

impl<T> TransactionService<T>
where
    for<'a> T: DatabaseInit + DatabaseWrite + DatabaseRead,
{
    pub fn new(db: T) -> TransactionService<T> {
        let db = Arc::new(Mutex::new(db));
        Self { db }
    }

    pub fn save_transaction(
        &self,
        new_transaction: ParsedTransaction,
    ) -> Result<(), TransactionError> {
        let mut db_connection = self
            .db
            .lock()
            .map_err(|e| TransactionError::DatabaseError(e.to_string()))?;

        db_connection
            .save(new_transaction)
            .map_err(|e| TransactionError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub fn find_transaction(&self, id: &str) -> Result<Transaction, TransactionError> {
        let mut db_connection = self
            .db
            .lock()
            .map_err(|e| TransactionError::DatabaseError(e.to_string()))?;

        let transaction: Transaction = db_connection
            .find(id)
            .map_err(|e| TransactionError::DatabaseError(e.to_string()))?;

        return Ok(transaction);
    }
}

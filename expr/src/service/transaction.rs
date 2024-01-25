use core::fmt;
use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

use crate::database::base::{DatabaseInit, DatabaseRead, DatabaseWrite};

use super::parse::ParsedTransaction;

enum TransactionError {
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
    T: DatabaseInit + DatabaseWrite + DatabaseRead,
{
    db: Arc<Mutex<T>>,
}

impl<T> TransactionService<T>
where
    T: DatabaseInit + DatabaseWrite + DatabaseRead,
{
    pub fn new(db: T) -> Self {
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
}

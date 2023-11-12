use std::error::Error;

use chrono::Utc;

use crate::{
    models::transactions::Transaction,
    t::{AddRequest, AddResponse, GetResponse},
};

pub trait DatabaseInit {
    fn connect(&mut self) -> Result<(), Box<dyn Error>>;
    fn disconnect(&mut self) -> Result<(), Box<dyn Error>>;
}

pub trait DatabaseWrite {
    fn save<T: ToString>(&mut self, id: String, record: T) -> Result<(), Box<dyn Error>>;
}

pub trait DatabaseRead {
    fn find(&mut self, id: String) -> Result<(), Box<dyn Error>>;
}

pub struct TransactionService<T>
where
    T: DatabaseInit + DatabaseWrite + DatabaseRead,
{
    db: T,
}

impl<T> TransactionService<T>
where
    T: DatabaseInit + DatabaseWrite + DatabaseRead,
{
    pub fn new(db: T) -> Self {
        Self { db }
    }

    pub fn get_transaction(&self) -> Result<GetResponse, Box<dyn Error>> {
        Ok(GetResponse {
            id: "1".to_string(),
            amount: 1.0,
            transaction_type: "test".to_string(),
            timestamp: 123,
            user: "user".to_string(),
            category: "category".to_string(),
            created_at: 123,
            updated_at: 123,
            deleted_at: 123,
        })
    }

    pub fn add_transaction(&mut self, request: AddRequest) -> Result<AddResponse, Box<dyn Error>> {
        self.db.connect()?;

        let timestamp = Utc::now();

        let new_transaction = Transaction::new(
            request.amount,
            request.transaction_type,
            timestamp,
            request.user,
        );

        self.db.save("1".to_string(), new_transaction);

        Ok(AddResponse { success: true })
    }
}

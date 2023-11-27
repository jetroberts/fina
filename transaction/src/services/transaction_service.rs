use std::{error::Error, sync::Arc};

use chrono::Utc;
use tokio::sync::Mutex;

use crate::{
    models::transactions::Transaction,
    t::{AddRequest, AddResponse, GetResponse},
};

pub trait DatabaseInit {
    fn connect(&mut self) -> Result<(), Box<dyn Error>>;
    fn disconnect(&mut self) -> Result<(), Box<dyn Error>>;
}

pub trait GetId {
    fn get_id(&self) -> String;
}

pub trait DatabaseWrite {
    fn save<T: ToString + GetId>(&mut self, record: T) -> Result<(), Box<dyn Error>>;
}

pub trait DatabaseRead {
    fn find(&mut self, id: String) -> Result<(), Box<dyn Error>>;
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

    pub async fn add_transaction(
        &self,
        request: AddRequest,
    ) -> Result<AddResponse, Box<dyn Error>> {
        let mut database = self.db.lock().await;
        database.connect()?;

        let timestamp = Utc::now();

        let new_transaction = Transaction::new(
            request.amount,
            request.transaction_type,
            timestamp,
            request.user,
        );

        let _ = database.save(new_transaction);

        Ok(AddResponse { success: true })
    }
}

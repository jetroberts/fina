use std::fs::{remove_file, File};

use uuid::Uuid;

use crate::service::transaction::{TransactionRead, TransactionWrite};

use super::base::{DatabaseError, DatabaseInit};

pub struct TextFile {
    filename: &'static str,
    file: Option<File>,
}

impl TextFile {
    pub fn new() -> Self {
        TextFile {
            filename: "tmp.test",
            file: None,
        }
    }
}

impl DatabaseInit for TextFile {
    async fn connect(&mut self) -> Result<(), DatabaseError> {
        let new_file = File::create(self.filename)
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

        self.file = Some(new_file);

        return Ok(());
    }

    async fn disconnect(&mut self) -> Result<(), DatabaseError> {
        self.file = None;
        remove_file(self.filename).map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;
        Ok(())
    }
}

impl TransactionWrite for TextFile {
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

impl TransactionRead for TextFile {
    async fn get_transaction(
        &self,
        _id: &str,
    ) -> Result<Option<crate::service::transaction::Transaction>, DatabaseError> {
        todo!()
    }
}

fn create_new_uuid() -> String {
    let id = uuid::Uuid::new_v4();
    id.to_string()
}

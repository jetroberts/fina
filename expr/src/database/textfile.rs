use std::{
    fs::{remove_file, File},
    io::Write,
};

use uuid::Uuid;

use crate::service::transaction::{TransactionRead, TransactionWrite};

use super::base::{DatabaseError, DatabaseInit, DatabaseRead, DatabaseWrite};

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

impl DatabaseRead for TextFile {
    async fn find<T: for<'a> serde::Deserialize<'a>>(
        &mut self,
        id: &str,
    ) -> Result<Option<T>, DatabaseError> {
        println!("{}", id);
        todo!()
    }

    async fn find_all<T>(&mut self) -> Result<Vec<T>, DatabaseError> {
        todo!()
    }
}

impl DatabaseWrite for TextFile {
    async fn save<T: ToString + serde::Serialize>(
        &mut self,
        record: T,
    ) -> Result<String, DatabaseError> {
        if self.file.is_none() {
            self.connect().await?;
        }

        match &mut self.file {
            Some(f) => {
                let new_id = create_new_uuid();
                let json = serde_json::to_string(&record)
                    .map_err(|e| DatabaseError::JsonError(e.to_string()))?;

                let _ = f
                    .write(json.as_bytes())
                    .map_err(|e| DatabaseError::SaveError(e.to_string()))?;

                return Ok(new_id);
            }
            None => Err(DatabaseError::ConnectionError(format!(
                "Tried to write to file but file was None"
            ))),
        }
    }

    async fn delete(&mut self, id: &str) -> Result<bool, DatabaseError> {
        println!("{}", id);
        todo!()
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
    return id.to_string();
}

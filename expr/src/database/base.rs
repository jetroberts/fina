use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::service::transaction::{CreateTransaction, Transaction};

pub enum DatabaseError {
    ConnectionError(String),
    ClientError(String),
    SaveError(String),
    GetError(String),
    DeleteError(String),
    JsonError(String),
    StringConversionError(String),
    UnknownValueError(String),
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::ConnectionError(e) => {
                write!(f, "DatabaseError -> ConnectionError, {}", e)
            }
            DatabaseError::ClientError(e) => write!(f, "DatabaseError -> ClientError, {}", e),
            DatabaseError::SaveError(e) => write!(f, "DatabaseError -> SaveError, {}", e),
            DatabaseError::GetError(e) => write!(f, "DatabaseError -> GetError, {}", e),
            DatabaseError::DeleteError(e) => write!(f, "DatabaseError -> DeleteError, {}", e),
            DatabaseError::JsonError(e) => write!(f, "DatabaseError -> JsonError, {}", e),
            DatabaseError::StringConversionError(e) => {
                write!(f, "DatabaseError -> StringConversionError, {}", e)
            }
            DatabaseError::UnknownValueError(e) => {
                write!(f, "DatabaseError -> UnknownValueError, {}", e)
            }
        }
    }
}

pub trait DatabaseInit {
    async fn connect(&mut self) -> Result<(), DatabaseError>;
    async fn disconnect(&mut self) -> Result<(), DatabaseError>;
}

pub trait GetId {
    fn get_id(&self) -> String;
}

pub trait DatabaseWrite {
    // do I want to split save into create and update? For nosql it's not an issue for sql it
    // probably will be
    async fn save<T: ToString + Serialize>(&mut self, record: T) -> Result<String, DatabaseError>;
    // may want to implement a soft delete
    async fn delete(&mut self, id: &str) -> Result<bool, DatabaseError>;
}

pub trait DatabaseRead {
    async fn find<T: for<'a> Deserialize<'a>>(
        &mut self,
        id: &str,
    ) -> Result<Option<T>, DatabaseError>;

    async fn find_all<T: for<'a> Deserialize<'a> + Send>(
        &mut self,
    ) -> Result<Vec<T>, DatabaseError>;
}

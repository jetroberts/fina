use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

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
    fn connect(&mut self) -> Result<(), DatabaseError>;
    fn disconnect(&mut self) -> Result<(), DatabaseError>;
}

pub trait GetId {
    fn get_id(&self) -> String;
}

pub trait DatabaseWrite {
    // do I want to split save into create and update? For nosql it's not an issue for sql it
    // probably will be
    fn save<T: ToString + Serialize>(&mut self, record: T) -> Result<String, DatabaseError>;
    // may want to implement a soft delete
    fn delete(&mut self, id: &str) -> Result<bool, DatabaseError>;
}

pub trait DatabaseRead {
    fn find<T: for<'a> Deserialize<'a>>(&mut self, id: &str) -> Result<Option<T>, DatabaseError>;
    fn find_all<T: for<'a> Deserialize<'a>>(&mut self) -> Result<Vec<T>, DatabaseError>;
}

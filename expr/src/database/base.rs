use std::{
    error::Error,
    fmt::{self, Display},
};

use serde::{Deserialize, Serialize};

pub trait DatabaseInit {
    fn connect(&mut self) -> Result<(), DatabaseError>;
    fn disconnect(&mut self) -> Result<(), Box<dyn Error>>;
}

pub trait GetId {
    fn get_id(&self) -> String;
}

pub trait DatabaseWrite {
    fn save<T: ToString + Serialize>(&mut self, record: T) -> Result<(), DatabaseError>;
}

pub trait DatabaseRead {
    fn find<T: for<'a> Deserialize<'a>>(&mut self, id: &str) -> Result<T, DatabaseError>;
    fn find_all<T>(&mut self) -> Result<Vec<T>, DatabaseError>;
}

pub enum DatabaseError {
    ConnectionError(String),
    ClientError(String),
    SaveError(String),
    GetError(String),
    JsonError(String),
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
            DatabaseError::JsonError(e) => write!(f, "DatabaseError -> JsonError, {}", e),
        }
    }
}

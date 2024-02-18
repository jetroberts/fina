use std::fmt::{self, Display};

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

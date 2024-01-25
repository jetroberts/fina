use std::error::Error;

pub trait DatabaseInit {
    fn connect(&mut self) -> Result<(), DatabaseError>;
    fn disconnect(&mut self) -> Result<(), Box<dyn Error>>;
}

pub trait GetId {
    fn get_id(&self) -> String;
}

pub trait DatabaseWrite {
    fn save<T: ToString>(&mut self, record: T) -> Result<(), Box<dyn Error>>;
}

pub trait DatabaseRead {
    fn find(&mut self, id: &str) -> Result<(), Box<dyn Error>>;
}

pub enum DatabaseError {
    ConnectionError(String),
    ClientError(String),
}

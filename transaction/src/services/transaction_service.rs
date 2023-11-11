use std::error::Error;

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

    pub fn get_transaction(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn add_transaction(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

use std::{error::Error, fmt::Display};

use super::base::{DatabaseError, DatabaseInit, DatabaseRead, DatabaseWrite};

pub struct Redis {
    client: Option<redis::Client>,
    connection: Option<redis::Connection>,
}

impl Redis {
    pub fn new() -> Self {
        Self {
            client: None,
            connection: None,
        }
    }
}

#[derive(Debug)]
struct RedisClientConnectionError;

impl Display for RedisClientConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unable to create redis client")
    }
}

impl Error for RedisClientConnectionError {}

#[derive(Debug)]
struct RedisConnectionError;

impl Error for RedisConnectionError {}

impl Display for RedisConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unable to create redis connection")
    }
}

impl DatabaseInit for Redis {
    fn connect(&mut self) -> Result<(), DatabaseError> {
        let client = redis::Client::open("redis://redis/")
            .map_err(|e| DatabaseError::ClientError(e.to_string()))?;

        let conn = client
            .get_connection()
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

        self.client = Some(client);
        self.connection = Some(conn);

        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), Box<dyn Error>> {
        self.connection = None;
        self.client = None;
        Ok(())
    }
}

impl DatabaseWrite for Redis {
    fn save<T: ToString>(&mut self, record: T) -> Result<(), Box<dyn Error>> {
        match self.connection.as_mut() {
            Some(c) => {
                redis::cmd("HSET")
                    .arg(create_new_uuid())
                    .arg(record.to_string())
                    .query(c)?;
            }
            None => {
                return Err("Redis connection was None".into());
            }
        }

        Ok(())
    }
}

impl DatabaseRead for Redis {
    fn find(&mut self, id: &str) -> Result<(), Box<dyn Error>> {
        match self.connection.as_mut() {
            Some(c) => {
                redis::cmd("HGET").arg(id).query(c)?;
            }
            None => return Err("Redis connection was None".into()),
        }

        Ok(())
    }
}

fn create_new_uuid() -> String {
    let id = uuid::Uuid::new_v4();
    return id.to_string();
}

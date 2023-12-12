use std::{error::Error, fmt::Display};

use crate::services::transaction_service::{DatabaseInit, DatabaseRead, DatabaseWrite, GetId};

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
    fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        let client = match redis::Client::open("redis://127.0.0.1/") {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Unable to create redis client, {}", e);
                let err = RedisClientConnectionError {};
                return Err(Box::new(err));
            }
        };
        let conn = match client.get_connection() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Unable to create redis connection, {}", e);
                let err = RedisConnectionError {};
                return Err(Box::new(err));
            }
        };
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
    fn save<T: ToString + GetId>(&mut self, record: T) -> Result<(), Box<dyn Error>> {
        match self.connection.as_mut() {
            Some(c) => {
                redis::cmd("HSET")
                    .arg(record.get_id())
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

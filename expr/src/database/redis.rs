use std::{error::Error, fmt::Display};

use redis::{Commands, RedisResult};
use serde::{Deserialize, Serialize};

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
    fn save<T: ToString + Serialize>(&mut self, record: T) -> Result<(), DatabaseError> {
        if self.connection.is_none() {
            self.connect()?;
            println!("Redis connection was None, attempting to connect");
        }

        match self.connection.as_mut() {
            Some(c) => {
                let uuid = create_new_uuid();
                println!("Saving record with uuid: {}", uuid);

                let json = serde_json::to_string(&record)
                    .map_err(|e| DatabaseError::JsonError(e.to_string()))?;

                redis::cmd("SET")
                    .arg(uuid.clone())
                    .arg(json)
                    .query(c)
                    .map_err(|e| DatabaseError::SaveError(e.to_string()))?;

                // need to figure out a good secondary index for searching by a subset of records
                // redis::cmd("SADD")
                // .arg(format!("Amex"))
                // .arg(uuid)
                // .query(c)
                // .map_err(|e| DatabaseError::SaveError(e.to_string()))?;

                println!("Record saved");
            }
            None => {
                eprintln!("Redis connection was None");
                return Err(DatabaseError::SaveError(
                    "Redis connection was None".to_string(),
                ));
            }
        }

        Ok(())
    }
}

impl DatabaseRead for Redis {
    fn find<T: for<'a> Deserialize<'a>>(&mut self, id: &str) -> Result<T, DatabaseError> {
        if self.connection.is_none() {
            self.connect()?;
        }

        match self.connection.as_mut() {
            Some(c) => {
                let res: String = c
                    .get(id)
                    .map_err(|e| DatabaseError::GetError(e.to_string()))?;

                let json: T = serde_json::from_str(&res)
                    .map_err(|e| DatabaseError::JsonError(e.to_string()))?;

                return Ok(json);
            }
            None => {
                return Err(DatabaseError::ConnectionError(
                    "No connection found whilst trying to find record".to_string(),
                ))
            }
        }
    }

    fn find_all<T>(&mut self) -> Result<Vec<T>, DatabaseError> {
        return Ok(Vec::new());
    }
}

fn create_new_uuid() -> String {
    let id = uuid::Uuid::new_v4();
    return id.to_string();
}

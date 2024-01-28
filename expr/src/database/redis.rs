use std::{error::Error, fmt::Display};

use redis::{Commands, FromRedisValue, RedisResult, Value};
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

    fn disconnect(&mut self) -> Result<(), DatabaseError> {
        // TODO: improve this disconnect function. Currently this is a memory leak
        self.connection = None;
        self.client = None;
        Ok(())
    }
}

impl DatabaseWrite for Redis {
    fn save<T: ToString + Serialize>(&mut self, record: T) -> Result<String, DatabaseError> {
        if self.connection.is_none() {
            self.connect()?;
            println!("Redis connection was None, attempting to connect");
        }

        match self.connection.as_mut() {
            Some(c) => {
                let uuid = create_new_uuid();

                let json = serde_json::to_string(&record)
                    .map_err(|e| DatabaseError::JsonError(e.to_string()))?;

                // potential issue is that the uuid is not part of the json object
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

                return Ok(uuid);
            }
            None => {
                eprintln!("Redis connection was None");
                return Err(DatabaseError::SaveError(
                    "Redis connection was None".to_string(),
                ));
            }
        }
    }

    fn delete(&mut self, _id: &str) -> Result<bool, DatabaseError> {
        todo!();
    }
}

impl DatabaseRead for Redis {
    fn find<T: for<'a> Deserialize<'a>>(&mut self, id: &str) -> Result<Option<T>, DatabaseError> {
        if self.connection.is_none() {
            self.connect()?;
        }

        match self.connection.as_mut() {
            Some(c) => {
                let res: Value = c
                    .get(id)
                    .map_err(|e| DatabaseError::GetError(e.to_string()))?;

                match res {
                    Value::Nil => return Ok(None),
                    Value::Data(d) => {
                        let data = String::from_utf8(d)
                            .map_err(|e| DatabaseError::StringConversionError(e.to_string()))?;

                        let json: T = serde_json::from_str(&data)
                            .map_err(|e| DatabaseError::JsonError(e.to_string()))?;
                        return Ok(Some(json));
                    }
                    _ => {
                        return Err(DatabaseError::UnknownValueError(
                            "Unknown value returned from redis".to_string(),
                        ))
                    }
                }
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

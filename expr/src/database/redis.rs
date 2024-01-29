use std::{error::Error, fmt::Display, sync::Arc};

use redis::{Commands, Value};
use serde::{Deserialize, Serialize};
use tokio::{sync::RwLock, task::JoinHandle};

use super::base::{DatabaseError, DatabaseInit, DatabaseRead, DatabaseWrite};

pub struct Redis {
    client: Option<Arc<RwLock<redis::Client>>>,
    connection: Option<Arc<RwLock<redis::Connection>>>,
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
    async fn connect(&mut self) -> Result<(), DatabaseError> {
        let client = redis::Client::open("redis://redis/")
            .map_err(|e| DatabaseError::ClientError(e.to_string()))?;

        let conn = client
            .get_connection()
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

        self.client = Some(Arc::new(RwLock::new(client)));
        self.connection = Some(Arc::new(RwLock::new(conn)));

        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), DatabaseError> {
        // TODO: improve this disconnect function. Currently this is a memory leak
        self.connection = None;
        self.client = None;
        Ok(())
    }
}

impl DatabaseWrite for Redis {
    async fn save<T: ToString + Serialize>(&mut self, record: T) -> Result<String, DatabaseError> {
        if self.connection.is_none() {
            self.connect().await?;
            println!("Redis connection was None, attempting to connect");
        }

        match self.connection.as_mut() {
            Some(c) => {
                let mut c = c.write().await;
                let uuid = create_new_uuid();

                let json = serde_json::to_string(&record)
                    .map_err(|e| DatabaseError::JsonError(e.to_string()))?;

                // potential issue is that the uuid is not part of the json object
                redis::cmd("SET")
                    .arg(uuid.clone())
                    .arg(json)
                    .query(&mut c)
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

    async fn delete(&mut self, id: &str) -> Result<bool, DatabaseError> {
        if self.connection.is_none() {
            self.connect().await?;
        }

        match self.connection.as_mut() {
            Some(conn) => {
                let mut c = conn.write().await;
                let _ = redis::cmd("DEL")
                    .arg(id)
                    .query::<()>(&mut c)
                    .map_err(|e| DatabaseError::DeleteError(e.to_string()))?;

                return Ok(true);
            }
            None => {
                eprintln!("Redis connection was None");
                return Err(DatabaseError::SaveError(
                    "Redis connection was None".to_string(),
                ));
            }
        }
    }
}

impl DatabaseRead for Redis {
    async fn find<T: for<'a> Deserialize<'a>>(
        &mut self,
        id: &str,
    ) -> Result<Option<T>, DatabaseError> {
        if self.connection.is_none() {
            self.connect().await?;
        }

        match self.connection.as_mut() {
            Some(c) => {
                let mut c = c.write().await;
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

    async fn find_all<T: for<'a> Deserialize<'a> + Send>(
        &mut self,
    ) -> Result<Vec<T>, DatabaseError> {
        if self.connection.is_none() {
            self.connect().await?;
        }

        let keys: Vec<Option<String>> = match self.connection.as_mut() {
            Some(conn) => {
                // this is going to be terrible for performance but prototypes eh
                let mut c = conn.write().await;
                let keys_values: Value = redis::cmd("KEYS")
                    .arg("*")
                    .query(&mut c)
                    .map_err(|e| DatabaseError::GetError(e.to_string()))?;

                // might need to refactor this into something simpler
                let keys: Vec<Option<String>> = match keys_values {
                    Value::Nil => Vec::new(),
                    Value::Bulk(keys) => keys
                        .into_iter()
                        .map(|k| match k {
                            Value::Data(d) => match String::from_utf8(d) {
                                Ok(res) => Some(res),
                                Err(e) => {
                                    eprintln!("Unable to convert redis key to string, {}", e);
                                    return None;
                                }
                            },
                            _ => None,
                        })
                        .collect(),
                    _ => Vec::new(),
                };

                if keys.is_empty() {
                    return Ok(Vec::new());
                }

                Ok(keys)
            }
            None => Err(DatabaseError::ConnectionError(
                "No connection found whilst trying to find all records".to_string(),
            )),
        }?;

        let entries: Vec<T> = Vec::new();

        let tasks: Vec<JoinHandle<_>> = keys
            .into_iter()
            .map(|key| {
                tokio::spawn(async move {
                    match key {
                        Some(k) => Some(self.find::<T>(&k).await),
                        None => None,
                    };
                })
            })
            .collect();

        for task in tasks {
            task.await;
        }

        return Ok(entries);
    }
}

fn create_new_uuid() -> String {
    let id = uuid::Uuid::new_v4();
    return id.to_string();
}

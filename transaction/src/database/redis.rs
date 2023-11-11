use crate::services::transaction_service::{DatabaseInit, DatabaseRead, DatabaseWrite};

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

impl DatabaseInit for Redis {
    fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let conn = client.get_connection()?;
        self.client = Some(client);
        self.connection = Some(conn);

        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.connection = None;
        self.client = None;
        Ok(())
    }
}

impl DatabaseWrite for Redis {
    fn save<T: ToString>(
        &mut self,
        id: String,
        record: T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self.connection.as_mut() {
            Some(c) => {
                redis::cmd("HSET")
                    .arg(id)
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
    fn find(&mut self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        match self.connection.as_mut() {
            Some(c) => {
                redis::cmd("HGET").arg(id).query(c)?;
            }
            None => return Err("Redis connection was None".into()),
        }

        Ok(())
    }
}

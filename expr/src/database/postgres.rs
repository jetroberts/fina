use sqlx::PgPool;

use super::base::{DatabaseError, DatabaseInit, DatabaseRead, DatabaseWrite};

pub struct Postgres {
    pool: Option<PgPool>,
}

impl Postgres {
    pub fn new() -> Self {
        Self { pool: None }
    }
}

impl DatabaseInit for Postgres {
    async fn connect(&mut self) -> Result<(), DatabaseError> {
        let pool = PgPool::connect("")
            .await
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

        self.pool = Some(pool);

        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), DatabaseError> {
        if let Some(pool) = &self.pool {
            pool.close().await;
            self.pool = None;
            return Ok(());
        }

        return Ok(());
    }
}

impl DatabaseRead for Postgres {
    async fn find<T: for<'a> serde::Deserialize<'a>>(
        &mut self,
        id: &str,
    ) -> Result<Option<T>, DatabaseError> {
        todo!()
    }

    async fn find_all<T: for<'a> serde::Deserialize<'a> + Send>(
        &mut self,
    ) -> Result<Vec<T>, DatabaseError> {
        todo!()
    }
}

impl DatabaseWrite for Postgres {
    async fn save<T: ToString + serde::Serialize>(
        &mut self,
        record: T,
    ) -> Result<String, DatabaseError> {
        if self.pool.is_some() {
            self.connect().await?;
        }

        let rows = sqlx::query!(
            r#"
                INSERT INTO transactions (, )
                VALUES ($1, $2)
                RETURNING id 
        "#
        );

        return Ok("".to_string());
    }

    async fn delete(&mut self, id: &str) -> Result<bool, DatabaseError> {
        todo!()
    }
}
